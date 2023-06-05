mod command;
mod did;
mod provider;
mod state;

use command::handle_action;
use lazy_static::lazy_static;
use state::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tracing::info;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            initialize_storage(app.handle()).ok();

            #[cfg(desktop)]
            tauri_plugin_deep_link::register("siopv2", |_| {}).unwrap();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![handle_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref UNSAFE_STORAGE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// Initialize the storage file paths.
fn initialize_storage(app_handle: tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *UNSAFE_STORAGE.lock().unwrap() = app_handle.path().data_dir()?.join("unsafe.bin");
    } else {
        *STATE_FILE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("state.json");
        *UNSAFE_STORAGE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("unsafe.bin");
    }
    info!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    info!("UNSAFE_STORAGE: {}", UNSAFE_STORAGE.lock().unwrap().display());

    Ok(())
}
