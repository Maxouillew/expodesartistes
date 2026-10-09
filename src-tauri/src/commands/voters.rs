use rusqlite::Connection;
use tauri::{AppHandle, State};

use crate::commands::auth::verify_admin_password;
use crate::commands::draw::DrawState;
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

/// Deleting the voter row removes the vote with it (one row holds both), and
/// frees the phone number and name so the person can vote again.
fn delete_voter_impl(conn: &Connection, id: i64) -> Result<(), String> {
    let deleted = conn
        .execute("DELETE FROM voters WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    if deleted == 0 {
        return Err("Ce votant n'existe plus.".to_string());
    }
    Ok(())
}

fn delete_all_voters_impl(conn: &Connection) -> Result<usize, String> {
    conn.execute("DELETE FROM voters", []).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_voter(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    delete_voter_impl(&conn, id)
}

/// Wipes every voter and vote (artists are kept). Re-asks the admin password
/// since this cannot be undone. Returns the number of deleted voters.
#[tauri::command]
pub fn delete_all_voters(
    app: AppHandle,
    db: State<Db>,
    draw_state: State<DrawState>,
    password: String,
) -> Result<usize, String> {
    verify_admin_password(&app, &password)?;
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    let deleted = delete_all_voters_impl(&conn)?;
    draw_state
        .0
        .lock()
        .map_err(|_| "État du tirage indisponible.".to_string())?
        .clear();
    Ok(deleted)
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

    fn insert_voter(conn: &Connection, first: &str, phone: &str, key: &str) {
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES (?1, 'Dupont', ?2, ?3, 1, 2, 3)",
            [first, phone, key],
        )
        .unwrap();
    }

    #[test]
    fn deleting_a_voter_removes_their_vote_and_lets_them_vote_again() {
        let conn = setup();
        insert_voter(&conn, "Jean", "+32470000001", "jean|dupont");
        let id = list_voters_impl(&conn).unwrap()[0].id;

        delete_voter_impl(&conn, id).unwrap();
        assert!(list_voters_impl(&conn).unwrap().is_empty());

        // Same phone and same name are accepted again.
        insert_voter(&conn, "Jean", "+32470000001", "jean|dupont");
        assert_eq!(list_voters_impl(&conn).unwrap().len(), 1);
    }

    #[test]
    fn deleting_an_unknown_voter_fails() {
        let conn = setup();
        assert!(delete_voter_impl(&conn, 42).is_err());
    }

    #[test]
    fn deleting_all_voters_keeps_the_artists() {
        let conn = setup();
        insert_voter(&conn, "Jean", "+32470000001", "jean|dupont");
        insert_voter(&conn, "Marc", "+32470000002", "marc|dupont");

        assert_eq!(delete_all_voters_impl(&conn).unwrap(), 2);
        assert!(list_voters_impl(&conn).unwrap().is_empty());
        let artists: i64 = conn
            .query_row("SELECT COUNT(*) FROM artists", [], |row| row.get(0))
            .unwrap();
        assert_eq!(artists, 3);
    }
}
