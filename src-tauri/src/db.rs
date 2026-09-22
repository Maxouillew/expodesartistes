use std::path::PathBuf;
use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

pub struct Db(pub Mutex<Connection>);

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS artists (
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS voters (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name  TEXT NOT NULL,
    phone      TEXT NOT NULL UNIQUE,
    name_key   TEXT NOT NULL UNIQUE,
    top1_id    INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    top2_id    INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    top3_id    INTEGER NOT NULL REFERENCES artists(id) ON UPDATE CASCADE ON DELETE RESTRICT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    CHECK (top1_id <> top2_id AND top1_id <> top3_id AND top2_id <> top3_id)
);
"#;

fn db_path(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir");
    std::fs::create_dir_all(&dir).expect("failed to create app data dir");
    dir.join("expodesartistes.sqlite3")
}

/// Opens the app's SQLite connection, enables foreign keys (off by default
/// in SQLite, but required for ON UPDATE CASCADE / ON DELETE RESTRICT to
/// take effect) and ensures the schema exists.
pub fn init(app: &AppHandle) -> rusqlite::Result<Connection> {
    let conn = Connection::open(db_path(app))?;
    conn.pragma_update(None, "foreign_keys", true)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::SCHEMA;
    use rusqlite::Connection;

    fn setup() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.pragma_update(None, "foreign_keys", true).unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        conn
    }

    #[test]
    fn accepts_a_valid_voter() {
        let conn = setup();
        conn.execute_batch(
            "INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');",
        )
        .unwrap();
        let result = conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        );
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_duplicate_top_ranks() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida');")
            .unwrap();
        let result = conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 1, 2)",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_phone() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');")
            .unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();
        let result = conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Marc', 'Martin', '+32470000000', 'marc|martin', 2, 1, 3)",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_name_key() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');")
            .unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();
        let result = conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('jean', ' DUPONT ', '+32470000001', 'jean|dupont', 2, 1, 3)",
            [],
        );
        assert!(result.is_err());
    }

    #[test]
    fn renumbering_an_artist_cascades_to_votes() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');")
            .unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();
        conn.execute("UPDATE artists SET id = 10 WHERE id = 1", [])
            .unwrap();
        let top1: i64 = conn
            .query_row(
                "SELECT top1_id FROM voters WHERE name_key = 'jean|dupont'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(top1, 10);
    }

    #[test]
    fn deleting_an_artist_with_votes_is_blocked() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida'), (3, 'Banksy');")
            .unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();
        let result = conn.execute("DELETE FROM artists WHERE id = 1", []);
        assert!(result.is_err());
    }

    #[test]
    fn deleting_an_artist_without_votes_succeeds() {
        let conn = setup();
        conn.execute_batch("INSERT INTO artists (id, name) VALUES (1, 'Leonard');")
            .unwrap();
        let result = conn.execute("DELETE FROM artists WHERE id = 1", []);
        assert!(result.is_ok());
    }
}
