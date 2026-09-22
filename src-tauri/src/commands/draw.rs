use std::collections::HashSet;
use std::sync::Mutex;

use rusqlite::{params_from_iter, Connection};
use tauri::State;

use crate::db::Db;

/// Voter ids already drawn during the current app session, so "Tirer à
/// nouveau" never repeats a winner. Reset naturally on app restart.
pub struct DrawState(pub Mutex<HashSet<i64>>);

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DrawnVoter {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

fn draw_voter_impl(conn: &Connection, excluded: &HashSet<i64>) -> Result<DrawnVoter, String> {
    let total_voters: i64 = conn
        .query_row("SELECT COUNT(*) FROM voters", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if total_voters == 0 {
        return Err("Aucun votant enregistré.".to_string());
    }

    let excluded_ids: Vec<i64> = excluded.iter().copied().collect();
    let result = if excluded_ids.is_empty() {
        let mut stmt = conn
            .prepare("SELECT id, first_name, last_name, phone FROM voters ORDER BY RANDOM() LIMIT 1")
            .map_err(|e| e.to_string())?;
        stmt.query_row([], |row| {
            Ok(DrawnVoter {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone: row.get(3)?,
            })
        })
    } else {
        let placeholders = excluded_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            "SELECT id, first_name, last_name, phone FROM voters WHERE id NOT IN ({}) ORDER BY RANDOM() LIMIT 1",
            placeholders
        );
        let mut stmt = conn.prepare(&query).map_err(|e| e.to_string())?;
        stmt.query_row(params_from_iter(excluded_ids.iter()), |row| {
            Ok(DrawnVoter {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone: row.get(3)?,
            })
        })
    };

    match result {
        Ok(voter) => Ok(voter),
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            Err("Tous les votants ont déjà été tirés au sort.".to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn draw_voter(db: State<Db>, draw_state: State<DrawState>) -> Result<DrawnVoter, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    let mut drawn = draw_state
        .0
        .lock()
        .map_err(|_| "État du tirage indisponible.".to_string())?;

    let voter = draw_voter_impl(&conn, &drawn)?;
    drawn.insert(voter.id);
    Ok(voter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE artists (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
             CREATE TABLE voters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                phone TEXT NOT NULL UNIQUE,
                name_key TEXT NOT NULL UNIQUE,
                top1_id INTEGER, top2_id INTEGER, top3_id INTEGER
             );
             INSERT INTO artists (id, name) VALUES (1, 'Leonard');
             INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
                VALUES ('A', 'A', '+3200000001', 'a|a', 1, 1, 1);
             INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
                VALUES ('B', 'B', '+3200000002', 'b|b', 1, 1, 1);",
        )
        .unwrap();
        conn
    }

    #[test]
    fn errors_when_no_voters_exist() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE voters (id INTEGER PRIMARY KEY, first_name TEXT, last_name TEXT, phone TEXT);",
        )
        .unwrap();
        let result = draw_voter_impl(&conn, &HashSet::new());
        assert!(result.is_err());
    }

    #[test]
    fn draws_a_voter_not_in_the_excluded_set() {
        let conn = setup();
        let mut excluded = HashSet::new();
        excluded.insert(1);
        let voter = draw_voter_impl(&conn, &excluded).unwrap();
        assert_eq!(voter.id, 2);
    }

    #[test]
    fn errors_once_every_voter_has_been_excluded() {
        let conn = setup();
        let mut excluded = HashSet::new();
        excluded.insert(1);
        excluded.insert(2);
        let result = draw_voter_impl(&conn, &excluded);
        assert!(result.is_err());
    }
}
