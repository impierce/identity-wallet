pub mod command;
pub mod crypto;
mod did;
pub mod state;

use command::handle_action;
use did_key::{generate, Ed25519KeyPair};
use fern::colors::Color;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use state::AppState;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind, WEBVIEW_TARGET};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![handle_action])
        .setup(move |app| {
            info!("setting up tauri app");
            initialize_storage(&app.handle()).ok();
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
                .level(LevelFilter::Debug)
                .level_for("identity_wallet", LevelFilter::Debug)
                .with_colors(
                    ColoredLevelConfig::new()
                        .trace(Color::White)
                        .debug(Color::Cyan)
                        .info(Color::Green)
                        .warn(Color::Yellow)
                        .error(Color::Red),
                )
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref PROVIDER_MANAGER: tauri::async_runtime::Mutex<Option<ProviderManager>> =
        tauri::async_runtime::Mutex::new(None);
}

/// Initialize the storage file paths.
fn initialize_storage(app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
    } else {
        *STATE_FILE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity_wallet")
            .join("stronghold.bin");
    }
    info!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    info!("STRONGHOLD: {}", STRONGHOLD.lock().unwrap().display());

    // Temporary solution to initialize the provider manager with a keypair.
    let subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(
        "this-is-a-very-UNSAFE-secret-key".as_bytes(),
    )));
    let provider_manager = ProviderManager::new([Arc::new(subject)]).unwrap();
    tauri::async_runtime::block_on(async { PROVIDER_MANAGER.lock().await.replace(provider_manager) });

    Ok(())
}

// Get the claims from a JWT without performing validation.
pub fn get_unverified_jwt_claims(jwt: &serde_json::Value) -> serde_json::Value {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    decode(jwt.as_str().unwrap(), &key, &validation).unwrap().claims
}
