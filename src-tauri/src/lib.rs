pub mod command;
pub mod crypto;
pub mod error;
pub mod state;
pub mod utils;
pub mod verifiable_credential_record;

use command::handle_action;
use fern::colors::Color;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use log::{info, LevelFilter};
use state::AppStateContainer;
use std::{fs, sync::Mutex};
use tauri::Manager;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};

use crate::state::persistence::clear_assets_tmp_folder;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_action])
        .setup(move |app| {
            info!("setting up tauri app");
            initialize_storage(app.handle()).ok();
            clear_assets_tmp_folder().ok();
            dotenvy::dotenv().ok();
            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            }
            Ok(())
        })
        .manage(AppStateContainer(Default::default()))
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
                .level(LevelFilter::Info)
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
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref ASSETS_DIR: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// Initialize the storage file paths.
fn initialize_storage(app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_handle.path().data_dir()?.join("assets");
    } else {
        *STATE_FILE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity-wallet")
            .join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity-wallet")
            .join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.identity-wallet")
            .join("assets");
    }
    info!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    info!("STRONGHOLD: {}", STRONGHOLD.lock().unwrap().display());
    // TODO: on iOS, when running the app for the first time,
    // is the assets folder even created?
    // bug: images are not downloaded/displayed for a credential offer (only on first start of the app)
    match fs::create_dir(ASSETS_DIR.lock().unwrap().as_path()) {
        Ok(_) => info!("ASSETS_DIR: created"),
        Err(e) => info!("ASSETS_DIR: {}", e),
    };

    Ok(())
}

// Get the claims from a JWT without performing validation.
pub fn get_unverified_jwt_claims(jwt: &serde_json::Value) -> serde_json::Value {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    decode(jwt.as_str().unwrap(), &key, &validation).unwrap().claims
}
