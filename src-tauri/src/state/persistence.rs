use tokio::{
    fs::{read, remove_file, File},
    io::AsyncWriteExt,
};
use tracing::info;
// use tauri::path::resolve_path;

use crate::state::state::TransferState;

/// Loads a [TransferState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state(_app_handle: tauri::AppHandle) -> anyhow::Result<TransferState> {
    //TODO: use tauri::path::BaseDirectory::AppData;
    let state_file_path = dirs_next::data_dir().unwrap().join("com.tauri.dev").join("state.json");
    let bytes = read(&state_file_path).await?;
    let content = String::from_utf8(bytes)?;
    let transfer_state: TransferState = serde_json::from_str(&content).unwrap();
    info!("state loaded from app_data_dir: {}", state_file_path.display());
    Ok(transfer_state)
}

/// Persists a [TransferState] to the app's data directory.
pub async fn save_state(_app_handle: tauri::AppHandle, transfer_state: TransferState) -> anyhow::Result<()> {
    //TODO: use tauri::path::BaseDirectory::AppData;
    let state_file_path = dirs_next::data_dir().unwrap().join("com.tauri.dev").join("state.json");
    let mut file = File::create(&state_file_path).await?;
    file.write_all(serde_json::to_string(&transfer_state).unwrap().as_bytes())
        .await?;
    info!("state saved to app_data_dir: {}", state_file_path.display());
    Ok(())
}

// Removes the state file from the app's data directory.
pub async fn delete_state(_app_handle: tauri::AppHandle) -> anyhow::Result<()> {
    //TODO: use tauri::path::BaseDirectory::AppData;
    let state_file_path = dirs_next::data_dir().unwrap().join("com.tauri.dev").join("state.json");
    remove_file(&state_file_path).await.unwrap();
    info!("state deleted from app_data_dir: {}", state_file_path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_load_state() {
        // TODO: how to mock the app_handle?
    }
}
