use phonenumber::country;
use rusqlite::{Connection, OptionalExtension};
use tauri::State;
use unicode_normalization::UnicodeNormalization;

use crate::db::Db;

const NAME_MIN_LEN: usize = 2;
const NAME_MAX_LEN: usize = 50;
const MIN_ARTISTS_TO_VOTE: usize = 3;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicArtist {
    pub id: i64,
    pub name: String,
}

fn validate_name(raw: &str, field_label: &str) -> Result<String, String> {
    let trimmed = raw.trim().to_string();
    let len = trimmed.chars().count();
    if !(NAME_MIN_LEN..=NAME_MAX_LEN).contains(&len) {
        return Err(format!(
            "{} doit contenir entre {} et {} caractères.",
            field_label, NAME_MIN_LEN, NAME_MAX_LEN
        ));
    }
    Ok(trimmed)
}

/// Case-, accent- and extra-whitespace-insensitive key for the unique
/// first+last name constraint (e.g. "jean  DUPONT" == "Jean Dupont").
fn normalize_name_key_part(raw: &str) -> String {
    raw.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .nfd()
        .filter(|c| !matches!(*c as u32, 0x0300..=0x036F))
        .collect::<String>()
        .to_lowercase()
}

/// Accepts Belgian local format (e.g. "0470 12 34 56") or full international
/// format (e.g. "+33 6 12 34 56 78"), normalized to E.164 before storage.
fn normalize_phone(raw: &str) -> Result<String, String> {
    let parsed = phonenumber::parse(Some(country::Id::BE), raw)
        .map_err(|_| "Numéro de téléphone invalide.".to_string())?;
    if !phonenumber::is_valid(&parsed) {
        return Err("Numéro de téléphone invalide.".to_string());
    }
    Ok(phonenumber::format(&parsed).mode(phonenumber::Mode::E164).to_string())
}

fn list_artists_for_vote_impl(conn: &Connection) -> Result<Vec<PublicArtist>, String> {
    let mut stmt = conn
        .prepare("SELECT id, name FROM artists ORDER BY id")
        .map_err(|e| e.to_string())?;
    let artists = stmt
        .query_map([], |row| {
            Ok(PublicArtist {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    Ok(artists)
}

#[allow(clippy::too_many_arguments)]
fn submit_vote_impl(
    conn: &Connection,
    first_name: String,
    last_name: String,
    phone: String,
    top1_id: i64,
    top2_id: i64,
    top3_id: i64,
) -> Result<(), String> {
    let artist_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM artists", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;
    if (artist_count as usize) < MIN_ARTISTS_TO_VOTE {
        return Err("Le vote n'est pas encore ouvert : il faut au moins 3 artistes.".to_string());
    }

    let first_name = validate_name(&first_name, "Le prénom")?;
    let last_name = validate_name(&last_name, "Le nom")?;

    if top1_id == top2_id || top1_id == top3_id || top2_id == top3_id {
        return Err("Les trois artistes doivent être distincts.".to_string());
    }

    for id in [top1_id, top2_id, top3_id] {
        let exists: Option<i64> = conn
            .query_row("SELECT id FROM artists WHERE id = ?1", [id], |row| row.get(0))
            .optional()
            .map_err(|e| e.to_string())?;
        if exists.is_none() {
            return Err(format!("Aucun artiste avec le numéro #{}.", id));
        }
    }

    let phone = normalize_phone(&phone)?;

    let phone_taken: Option<i64> = conn
        .query_row("SELECT id FROM voters WHERE phone = ?1", [&phone], |row| row.get(0))
        .optional()
        .map_err(|e| e.to_string())?;
    if phone_taken.is_some() {
        return Err("Ce numéro de téléphone a déjà voté.".to_string());
    }

    let name_key = format!(
        "{}|{}",
        normalize_name_key_part(&first_name),
        normalize_name_key_part(&last_name)
    );
    let name_taken: Option<i64> = conn
        .query_row("SELECT id FROM voters WHERE name_key = ?1", [&name_key], |row| row.get(0))
        .optional()
        .map_err(|e| e.to_string())?;
    if name_taken.is_some() {
        return Err("Ce nom a déjà voté.".to_string());
    }

    conn.execute(
        "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&first_name, &last_name, &phone, &name_key, top1_id, top2_id, top3_id),
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn list_artists_for_vote(db: State<Db>) -> Result<Vec<PublicArtist>, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    list_artists_for_vote_impl(&conn)
}

#[tauri::command]
pub fn submit_vote(
    db: State<Db>,
    first_name: String,
    last_name: String,
    phone: String,
    top1_id: i64,
    top2_id: i64,
    top3_id: i64,
) -> Result<(), String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    submit_vote_impl(&conn, first_name, last_name, phone, top1_id, top2_id, top3_id)
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
    fn normalizes_belgian_local_numbers() {
        assert_eq!(normalize_phone("0470 12 34 56").unwrap(), "+32470123456");
    }

    #[test]
    fn normalizes_international_numbers() {
        assert_eq!(normalize_phone("+33 6 12 34 56 78").unwrap(), "+33612345678");
    }

    #[test]
    fn rejects_garbage_phone_numbers() {
        assert!(normalize_phone("not a phone number").is_err());
        assert!(normalize_phone("12345").is_err());
    }

    #[test]
    fn name_key_ignores_case_accents_and_spacing() {
        assert_eq!(normalize_name_key_part("  Léonard  "), "leonard");
        assert_eq!(normalize_name_key_part("DUPONT"), "dupont");
        assert_eq!(normalize_name_key_part("Jean   Paul"), "jean paul");
    }

    #[test]
    fn accepts_a_valid_vote() {
        let conn = setup();
        let result = submit_vote_impl(
            &conn,
            "Jean".to_string(),
            "Dupont".to_string(),
            "0470 12 34 56".to_string(),
            1,
            2,
            3,
        );
        assert!(result.is_ok(), "{:?}", result);
    }

    #[test]
    fn rejects_when_fewer_than_three_artists() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "CREATE TABLE artists (id INTEGER PRIMARY KEY, name TEXT NOT NULL);
             CREATE TABLE voters (id INTEGER PRIMARY KEY, first_name TEXT, last_name TEXT, phone TEXT UNIQUE, name_key TEXT UNIQUE, top1_id INTEGER, top2_id INTEGER, top3_id INTEGER);
             INSERT INTO artists (id, name) VALUES (1, 'Leonard'), (2, 'Frida');",
        )
        .unwrap();
        let result = submit_vote_impl(&conn, "Jean".into(), "Dupont".into(), "0470123456".into(), 1, 2, 1);
        assert!(result.unwrap_err().contains("3 artistes"));
    }

    #[test]
    fn rejects_non_distinct_top_ranks() {
        let conn = setup();
        let result = submit_vote_impl(&conn, "Jean".into(), "Dupont".into(), "0470123456".into(), 1, 1, 2);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_names_that_are_too_short() {
        let conn = setup();
        let result = submit_vote_impl(&conn, "J".into(), "Dupont".into(), "0470123456".into(), 1, 2, 3);
        assert!(result.is_err());
    }

    #[test]
    fn rejects_duplicate_phone_across_voters() {
        let conn = setup();
        submit_vote_impl(&conn, "Jean".into(), "Dupont".into(), "0470123456".into(), 1, 2, 3).unwrap();
        let result = submit_vote_impl(&conn, "Marc".into(), "Martin".into(), "0470 12 34 56".into(), 2, 1, 3);
        assert!(result.unwrap_err().contains("téléphone"));
    }

    #[test]
    fn rejects_duplicate_name_regardless_of_case_accents_and_spacing() {
        let conn = setup();
        submit_vote_impl(&conn, "Jean".into(), "Dupont".into(), "0470123456".into(), 1, 2, 3).unwrap();
        let result = submit_vote_impl(&conn, "  jean ".into(), "DUPONT".into(), "0471234567".into(), 2, 1, 3);
        assert!(result.unwrap_err().contains("nom"));
    }

    #[test]
    fn allows_different_first_names_with_same_last_name() {
        let conn = setup();
        submit_vote_impl(&conn, "Jean".into(), "Dupont".into(), "0470123456".into(), 1, 2, 3).unwrap();
        let result = submit_vote_impl(&conn, "Marc".into(), "Dupont".into(), "0471234567".into(), 2, 1, 3);
        assert!(result.is_ok());
    }
}
