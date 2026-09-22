use std::fs;
use std::path::PathBuf;

use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use argon2::Argon2;
use tauri::{AppHandle, Manager};

const MIN_PASSWORD_LEN: usize = 8;

/// Set at build time (e.g. `ADMIN_PASSWORD_HASH=$(argon2 hash) npm run tauri build`)
/// to ship a fixed admin password. When absent, the app falls back to a
/// hash stored on first launch (see `config_path`).
fn build_time_hash() -> Option<&'static str> {
    option_env!("ADMIN_PASSWORD_HASH")
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AdminConfig {
    password_hash: String,
}

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|_| "Impossible de localiser le dossier de configuration.".to_string())?;
    fs::create_dir_all(&dir).map_err(|_| "Impossible de créer le dossier de configuration.".to_string())?;
    Ok(dir.join("admin.json"))
}

fn read_stored_hash(app: &AppHandle) -> Option<String> {
    let path = config_path(app).ok()?;
    let content = fs::read_to_string(path).ok()?;
    let config: AdminConfig = serde_json::from_str(&content).ok()?;
    Some(config.password_hash)
}

fn write_stored_hash(app: &AppHandle, hash: &str) -> Result<(), String> {
    let path = config_path(app)?;
    let config = AdminConfig {
        password_hash: hash.to_string(),
    };
    let json = serde_json::to_string_pretty(&config)
        .map_err(|_| "Impossible de sérialiser le mot de passe.".to_string())?;
    fs::write(path, json).map_err(|_| "Impossible d'enregistrer le mot de passe.".to_string())
}

fn hash_password(password: &str) -> Result<String, String> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| "Impossible de calculer le hash du mot de passe.".to_string())
}

fn verify_password(password: &str, hash: &str) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), hash)
        .is_ok()
}

/// True when no admin password exists yet (neither built into the binary
/// nor set on a previous launch) — the frontend should show a "create your
/// admin password" screen instead of the login form.
#[tauri::command]
pub fn admin_needs_setup(app: AppHandle) -> bool {
    build_time_hash().is_none() && read_stored_hash(&app).is_none()
}

#[tauri::command]
pub fn admin_set_password(app: AppHandle, password: String) -> Result<(), String> {
    if build_time_hash().is_some() {
        return Err("Le mot de passe admin est déjà configuré au build.".to_string());
    }
    if read_stored_hash(&app).is_some() {
        return Err("Un mot de passe admin est déjà configuré.".to_string());
    }
    if password.len() < MIN_PASSWORD_LEN {
        return Err(format!(
            "Le mot de passe doit contenir au moins {} caractères.",
            MIN_PASSWORD_LEN
        ));
    }
    let hash = hash_password(&password)?;
    write_stored_hash(&app, &hash)
}

#[tauri::command]
pub fn admin_login(app: AppHandle, password: String) -> Result<(), String> {
    let hash = build_time_hash()
        .map(|s| s.to_string())
        .or_else(|| read_stored_hash(&app))
        .ok_or_else(|| "Aucun mot de passe admin configuré.".to_string())?;

    if verify_password(&password, &hash) {
        Ok(())
    } else {
        Err("Mot de passe incorrect.".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn hashes_and_verifies_a_password() {
        let hash = hash_password("correct-horse-battery-staple").unwrap();
        assert!(verify_password("correct-horse-battery-staple", &hash));
    }

    #[test]
    fn rejects_a_wrong_password() {
        let hash = hash_password("correct-horse-battery-staple").unwrap();
        assert!(!verify_password("wrong-password", &hash));
    }

    #[test]
    fn rejects_a_malformed_hash() {
        assert!(!verify_password("anything", "not-a-real-hash"));
    }
}
