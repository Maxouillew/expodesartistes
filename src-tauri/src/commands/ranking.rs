use rusqlite::Connection;
use tauri::State;

use crate::db::Db;

// Keep in sync with src/lib/config/points.ts (the frontend's single source
// of truth for display purposes) — Rust owns the actual ranking computation
// per the architecture note that classement calculation is server-side.
const POINTS_TOP1: i64 = 3;
const POINTS_TOP2: i64 = 2;
const POINTS_TOP3: i64 = 1;

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistRanking {
    pub rank: i64,
    pub id: i64,
    pub name: String,
    pub score: i64,
    pub top1_count: i64,
    pub top2_count: i64,
    pub top3_count: i64,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RankingResponse {
    pub total_votes: i64,
    pub artists: Vec<ArtistRanking>,
}

fn get_ranking_impl(conn: &Connection) -> Result<RankingResponse, String> {
    let total_votes: i64 = conn
        .query_row("SELECT COUNT(*) FROM voters", [], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT a.id, a.name,
                (SELECT COUNT(*) FROM voters v WHERE v.top1_id = a.id) AS top1_count,
                (SELECT COUNT(*) FROM voters v WHERE v.top2_id = a.id) AS top2_count,
                (SELECT COUNT(*) FROM voters v WHERE v.top3_id = a.id) AS top3_count
             FROM artists a",
        )
        .map_err(|e| e.to_string())?;

    let mut artists: Vec<ArtistRanking> = stmt
        .query_map([], |row| {
            let top1_count: i64 = row.get(2)?;
            let top2_count: i64 = row.get(3)?;
            let top3_count: i64 = row.get(4)?;
            Ok(ArtistRanking {
                rank: 0,
                id: row.get(0)?,
                name: row.get(1)?,
                score: top1_count * POINTS_TOP1 + top2_count * POINTS_TOP2 + top3_count * POINTS_TOP3,
                top1_count,
                top2_count,
                top3_count,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    artists.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(b.top1_count.cmp(&a.top1_count))
            .then(b.top2_count.cmp(&a.top2_count))
            .then(a.id.cmp(&b.id))
    });

    for (index, artist) in artists.iter_mut().enumerate() {
        artist.rank = (index + 1) as i64;
    }

    Ok(RankingResponse { total_votes, artists })
}

#[tauri::command]
pub fn get_ranking(db: State<Db>) -> Result<RankingResponse, String> {
    let conn = db.0.lock().map_err(|_| "Connexion à la base indisponible.".to_string())?;
    get_ranking_impl(&conn)
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

    fn insert_voter(conn: &Connection, phone: &str, top1: i64, top2: i64, top3: i64) {
        conn.execute(
            "INSERT INTO voters (first_name, last_name, phone, name_key, top1_id, top2_id, top3_id)
             VALUES ('Jean', 'Dupont', ?1, ?1, ?2, ?3, ?4)",
            (phone, top1, top2, top3),
        )
        .unwrap();
    }

    #[test]
    fn scores_and_ranks_by_weighted_points() {
        let conn = setup();
        // Voter1: 1=top1, 3=top2, 2=top3. Voter2: 1=top1, 2=top2, 3=top3. Voter3: 2=top1, 1=top2, 3=top3.
        // Artist 1: two Top1 + one Top2 = 2*3 + 1*2 = 8 pts.
        // Artist 2: one Top1 + one Top2 + one Top3 = 3 + 2 + 1 = 6 pts.
        // Artist 3: one Top2 + two Top3 = 2 + 2*1 = 4 pts.
        insert_voter(&conn, "+32470000001", 1, 3, 2);
        insert_voter(&conn, "+32470000002", 1, 2, 3);
        insert_voter(&conn, "+32470000003", 2, 1, 3);

        let ranking = get_ranking_impl(&conn).unwrap();
        assert_eq!(ranking.total_votes, 3);
        assert_eq!(ranking.artists[0].id, 1);
        assert_eq!(ranking.artists[0].score, 8);
        assert_eq!(ranking.artists[0].rank, 1);
        assert_eq!(ranking.artists[1].id, 2);
        assert_eq!(ranking.artists[1].score, 6);
        assert_eq!(ranking.artists[2].id, 3);
        assert_eq!(ranking.artists[2].score, 4);
    }

    #[test]
    fn ties_break_by_top1_then_top2_then_artist_number() {
        let conn = setup();
        // Artist 1 and 2 both score 3 pts via one Top1 each -> tie broken by top1 count (equal),
        // then top2 count (equal), then ascending artist number.
        insert_voter(&conn, "+32470000001", 1, 3, 2);
        insert_voter(&conn, "+32470000002", 2, 3, 1);

        let ranking = get_ranking_impl(&conn).unwrap();
        assert_eq!(ranking.artists[0].id, 1);
        assert_eq!(ranking.artists[1].id, 2);
    }
}
