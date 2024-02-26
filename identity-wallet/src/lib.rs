pub mod command;
pub mod crypto;
pub mod error;
pub mod state;
pub mod utils;
pub mod verifiable_credential_record;

// Re-exports
pub use oid4vc::{oid4vc_core, oid4vc_manager, oid4vci, oid4vp, siopv2};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use log::info;
use std::{fs, sync::Mutex};
use tauri::Manager;

/// This folder is where the main backend rust code lives together with all the business logic.
/// The folder state is where our appstate and it's features are defined, completely according to the redux pattern.
/// The command.rs holds the functions through which the front and backend comminicate using actions and reducers.
/// The error.rs defines our app_error types, implemented throughout the code using the thiserror crate.
/// The persistence.rs is where we define our app persistence functions.
/// The stronghold.rs is where we implement the stronghold library for our app, which is used to store sensitive data.

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref ASSETS_DIR: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// Initialize the storage file paths.
pub fn initialize_storage(app_handle: &tauri::AppHandle) -> anyhow::Result<()> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_handle.path().data_dir()?.join("assets");
    } else {
        *STATE_FILE.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.unime")
            .join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle
            .path()
            .data_dir()?
            .join("com.impierce.unime")
            .join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_handle.path().data_dir()?.join("com.impierce.unime").join("assets");
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

/// Get the claims from a JWT without performing validation.
pub fn get_unverified_jwt_claims(jwt: &serde_json::Value) -> serde_json::Value {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    decode(jwt.as_str().unwrap(), &key, &validation).unwrap().claims
}
