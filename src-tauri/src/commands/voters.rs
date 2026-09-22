use rusqlite::Connection;
use tauri::State;

use crate::db::Db;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Voter {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub created_at: String,
}

fn list_voters_impl(conn: &Connection) -> Result<Vec<Voter>, String> {
    let mut stmt = conn
        .prepare("SELECT id, first_name, last_name, phone, created_at FROM voters ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let voters: Vec<Voter> = stmt
        .query_map([], |row| {
            Ok(Voter {
                id: row.get(0)?,
                first_name: row.get(1)?,
                last_name: row.get(2)?,
                phone: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    Ok(voters)
}

#[tauri::command]
pub fn list_voters(db: State<Db>) -> Result<Vec<Voter>, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    list_voters_impl(&conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", true).unwrap();
        conn.execute_batch(
            "CREATE TABLE artists (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
             CREATE TABLE voters (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                first_name TEXT NOT NULL,
                last_name TEXT NOT NULL,
                phone TEXT NOT NULL UNIQUE,
                name_key TEXT NOT NULL UNIQUE,
                top1_id INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
                top2_id INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
                top3_id INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                CHECK (top1_id <> top2_id AND top1_id <> top3_id AND top2_id <> top3_id)
             );
             INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');",
        )
        .unwrap();
        conn
    }

    #[test]
    fn lists_voters_most_recent_first_without_their_artist_choices() {
        let conn = setup();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000001', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Marie', 'Curie', '+32470000002', 'marie|curie', 2, 3, 1)",
            [],
        )
        .unwrap();

        let voters = list_voters_impl(&conn).unwrap();
        assert_eq!(voters.len(), 2);
        assert_eq!(voters[0].first_name, "Marie");
        assert_eq!(voters[1].first_name, "Jean");
        assert_eq!(voters[1].phone, "+32470000001");
    }

    #[test]
    fn returns_an_empty_list_when_there_are_no_voters() {
        let conn = setup();
        assert!(list_voters_impl(&conn).unwrap().is_empty());
    }
}
