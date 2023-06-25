mod command;
mod crypto;
mod did;
mod state;

use command::handle_action;
use lazy_static::lazy_static;
use log::info;
use state::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind, WEBVIEW_TARGET};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![handle_action])
        .setup(move |app| {
            info!("setting up tauri app");
            initialize_storage(app.handle()).ok();
            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                // .clear_targets()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                    // Target::new(TargetKind::LogDir {
                    //     file_name: Some("webview".into()),
                    // })
                    // .filter(|metadata| metadata.target() == WEBVIEW_TARGET),
                    // Target::new(TargetKind::LogDir {
                    //     file_name: Some("rust".into()),
                    // })
                    // .filter(|metadata| metadata.target() != WEBVIEW_TARGET),
                ])
                .with_colors(Default::default())
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref UNSAFE_DEV_STORAGE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// Initialize the storage file paths.
fn initialize_storage(app_handle: tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *UNSAFE_DEV_STORAGE.lock().unwrap() = app_handle.path().data_dir()?.join("unsafe.bin");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
    } else {
        *STATE_FILE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("state.json");
        *UNSAFE_DEV_STORAGE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("unsafe.bin");
        *STRONGHOLD.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("stronghold.bin");
    }
    info!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    info!("UNSAFE_STORAGE: {}", UNSAFE_DEV_STORAGE.lock().unwrap().display());
    info!("STRONGHOLD: {}", STRONGHOLD.lock().unwrap().display());

    Ok(())
}
