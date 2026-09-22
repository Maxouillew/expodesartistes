use rusqlite::{Connection, OptionalExtension};
use tauri::State;

use crate::db::Db;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithVotes {
    pub id: i64,
    pub name: String,
    pub vote_count: i64,
}

fn vote_count(conn: &Connection, id: i64) -> rusqlite::Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM voters WHERE top1_id = ?1 OR top2_id = ?1 OR top3_id = ?1",
        [id],
        |row| row.get(0),
    )
}

fn find_artist(conn: &Connection, id: i64) -> Result<Option<ArtistWithVotes>, String> {
    let name: Option<String> = conn
        .query_row("SELECT name FROM artists WHERE id = ?1", [id], |row| row.get(0))
        .optional()
        .map_err(|e| e.to_string())?;

    match name {
        None => Ok(None),
        Some(name) => {
            let count = vote_count(conn, id).map_err(|e| e.to_string())?;
            Ok(Some(ArtistWithVotes {
                id,
                name,
                vote_count: count,
            }))
        }
    }
}

fn validate_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim().to_string();
    if trimmed.is_empty() {
        return Err("Le nom de l'artiste est requis.".to_string());
    }
    Ok(trimmed)
}

fn validate_id(id: i64) -> Result<(), String> {
    if id <= 0 {
        return Err("Le numéro doit être un entier positif.".to_string());
    }
    Ok(())
}

fn list_artists_impl(conn: &Connection) -> Result<Vec<ArtistWithVotes>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name FROM artists ORDER BY id")
        .map_err(|e| e.to_string())?;
    let ids: Vec<(i64, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    ids.into_iter()
        .map(|(id, name)| {
            let count = vote_count(conn, id).map_err(|e| e.to_string())?;
            Ok(ArtistWithVotes {
                id,
                name,
                vote_count: count,
            })
        })
        .collect()
}

fn add_artist_impl(conn: &Connection, id: i64, name: String) -> Result<ArtistWithVotes, String> {
    validate_id(id)?;
    let name = validate_name(&name)?;

    if find_artist(conn, id)?.is_some() {
        return Err(format!("Le numéro #{} est déjà attribué.", id));
    }

    conn.execute("INSERT INTO artists (id, name) VALUES (?1, ?2)", (id, &name))
        .map_err(|e| e.to_string())?;

    Ok(ArtistWithVotes {
        id,
        name,
        vote_count: 0,
    })
}

fn update_artist_impl(
    conn: &Connection,
    current_id: i64,
    new_id: i64,
    new_name: String,
) -> Result<ArtistWithVotes, String> {
    validate_id(new_id)?;
    let new_name = validate_name(&new_name)?;

    if find_artist(conn, current_id)?.is_none() {
        return Err(format!("Aucun artiste avec le numéro #{}.", current_id));
    }

    if new_id != current_id && find_artist(conn, new_id)?.is_some() {
        return Err(format!("Le numéro #{} est déjà attribué.", new_id));
    }

    conn.execute(
        "UPDATE artists SET id = ?1, name = ?2 WHERE id = ?3",
        (new_id, &new_name, current_id),
    )
    .map_err(|e| e.to_string())?;

    let count = vote_count(conn, new_id).map_err(|e| e.to_string())?;
    Ok(ArtistWithVotes {
        id: new_id,
        name: new_name,
        vote_count: count,
    })
}

fn delete_artist_impl(conn: &Connection, id: i64) -> Result<(), String> {
    let artist = find_artist(conn, id)?.ok_or_else(|| format!("Aucun artiste avec le numéro #{}.", id))?;

    if artist.vote_count > 0 {
        return Err(format!(
            "Impossible de supprimer l'artiste #{} {} : {} vote(s) déjà enregistré(s).",
            artist.id, artist.name, artist.vote_count
        ));
    }

    conn.execute("DELETE FROM artists WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn list_artists(db: State<Db>) -> Result<Vec<ArtistWithVotes>, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    list_artists_impl(&conn)
}

#[tauri::command]
pub fn add_artist(db: State<Db>, id: i64, name: String) -> Result<ArtistWithVotes, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    add_artist_impl(&conn, id, name)
}

#[tauri::command]
pub fn update_artist(
    db: State<Db>,
    current_id: i64,
    new_id: i64,
    new_name: String,
) -> Result<ArtistWithVotes, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    update_artist_impl(&conn, current_id, new_id, new_name)
}

#[tauri::command]
pub fn delete_artist(db: State<Db>, id: i64) -> Result<(), String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    delete_artist_impl(&conn, id)
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
             );",
        )
        .unwrap();
        conn
    }

    #[test]
    fn adds_and_lists_an_artist() {
        let conn = setup();
        add_artist_impl(&conn, 1, "  Leonard  ".to_string()).unwrap();
        let artists = list_artists_impl(&conn).unwrap();
        assert_eq!(artists.len(), 1);
        assert_eq!(artists[0].name, "Leonard");
        assert_eq!(artists[0].vote_count, 0);
    }

    #[test]
    fn rejects_duplicate_artist_number() {
        let conn = setup();
        add_artist_impl(&conn, 1, "Leonard".to_string()).unwrap();
        let result = add_artist_impl(&conn, 1, "Frida".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn rejects_blank_name() {
        let conn = setup();
        let result = add_artist_impl(&conn, 1, "   ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn rejects_non_positive_number() {
        let conn = setup();
        let result = add_artist_impl(&conn, 0, "Leonard".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn renumbering_to_a_free_slot_succeeds_and_cascades() {
        let conn = setup();
        add_artist_impl(&conn, 1, "Leonard".to_string()).unwrap();
        add_artist_impl(&conn, 2, "Frida".to_string()).unwrap();
        add_artist_impl(&conn, 3, "Banksy".to_string()).unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();

        update_artist_impl(&conn, 1, 10, "Leonard de Vinci".to_string()).unwrap();

        let top1: i64 = conn
            .query_row("SELECT top1_id FROM voters WHERE name_key = 'jean|dupont'", [], |row| row.get(0))
            .unwrap();
        assert_eq!(top1, 10);
    }

    #[test]
    fn renumbering_to_a_taken_slot_is_rejected() {
        let conn = setup();
        add_artist_impl(&conn, 1, "Leonard".to_string()).unwrap();
        add_artist_impl(&conn, 2, "Frida".to_string()).unwrap();
        let result = update_artist_impl(&conn, 1, 2, "Leonard".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn blocks_deleting_an_artist_with_votes() {
        let conn = setup();
        add_artist_impl(&conn, 1, "Leonard".to_string()).unwrap();
        add_artist_impl(&conn, 2, "Frida".to_string()).unwrap();
        add_artist_impl(&conn, 3, "Banksy".to_string()).unwrap();
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', '+32470000000', 'jean|dupont', 1, 2, 3)",
            [],
        )
        .unwrap();

        let result = delete_artist_impl(&conn, 1);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("1 vote"));
    }

    #[test]
    fn allows_deleting_an_artist_without_votes() {
        let conn = setup();
        add_artist_impl(&conn, 1, "Leonard".to_string()).unwrap();
        assert!(delete_artist_impl(&conn, 1).is_ok());
        assert!(list_artists_impl(&conn).unwrap().is_empty());
    }
}
