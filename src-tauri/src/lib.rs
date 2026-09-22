mod commands;
mod db;

use std::collections::HashSet;
use std::sync::Mutex;
use tauri::Manager;

use commands::{artists, auth, draw, ranking, vote, voters};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            let conn = db::init(app.handle())?;
            app.manage(db::Db(Mutex::new(conn)));
            app.manage(draw::DrawState(Mutex::new(HashSet::new())));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::admin_needs_setup,
            auth::admin_set_password,
            auth::admin_login,
            artists::list_artists,
            artists::add_artist,
            artists::update_artist,
            artists::delete_artist,
            ranking::get_ranking,
            draw::draw_voter,
            voters::list_voters,
            vote::list_artists_for_vote,
            vote::submit_vote,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
