mod command;
mod did;
mod state;

// use argon2::{Argon2, PasswordHasher};
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
        .invoke_handler(tauri::generate_handler![handle_action])
        .setup(|app| {
            initialize_storage(app.handle()).ok();
            Ok(())
        })
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_stronghold::Builder::new(|password| {
                let config = argon2::Config {
                    lanes: 2,
                    mem_cost: 50_000,
                    time_cost: 30,
                    thread_mode: argon2::ThreadMode::from_threads(2),
                    variant: argon2::Variant::Argon2id,
                    ..Default::default()
                };

                let config = argon2::Config::default();

                let key = argon2::hash_raw(password.as_ref(), b"SALT_IDEALLY_SHOULD_BE_RANDOM", &config)
                    .expect("failed to hash password");

                key.to_vec()
            })
            .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref UNSAFE_STORAGE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// Initialize the storage file paths.
fn initialize_storage(app_handle: tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *UNSAFE_STORAGE.lock().unwrap() = app_handle.path().data_dir()?.join("unsafe.bin");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
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
        *STRONGHOLD.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("stronghold.bin");
    }
    info!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    info!("UNSAFE_STORAGE: {}", UNSAFE_STORAGE.lock().unwrap().display());
    info!("STRONGHOLD: {}", STRONGHOLD.lock().unwrap().display());

    Ok(())
}
