use std::fs;

use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit};
use argon2::Argon2;
use log::info;
use sha2::{Digest, Sha256};
use tauri_plugin_fs;

use crate::{
    error::AppError,
    persistence::{STATE_FILE, STRONGHOLD},
    state::{
        actions::{listen, Action},
        backup::actions::create::CreateBackup,
        AppState,
    },
};

pub async fn create_backup(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateBackup { path, password }) = listen::<CreateBackup>(action) {
        // 1. Hash the password using SHA-256
        // TODO: prefer Argon2? Problem: where do we store the salt?
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        let key = hasher.finalize();

        // 2. Use the hashed password as key for AES-256-GCM
        let cipher = Aes256Gcm::new(&key);

        // 3. TODO: Generate a random nonce (problem: how to store it?)
        let nonce = [0; 12]; // 96 bits

        let state_file = fs::read(STATE_FILE.lock().unwrap().as_path())?;
        let stronghold_file = fs::read(STRONGHOLD.lock().unwrap().as_path())?;
        // TODO: also backup ASSETS_DIR

        let data = vec![state_file, stronghold_file].concat();

        let res = cipher
            .encrypt(&nonce.into(), data.as_ref())
            .map_err(|_| AppError::Error("Encryption failed".to_string()))?;

        info!("Writing backup to file: {} ({} bytes)", path, data.len());

        fs::write(path, res)?;

        return Ok(AppState {
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
