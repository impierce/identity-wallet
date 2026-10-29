use crate::http_client::fetch_public_destination;
use crate::migrations::apply_state_migrations;
use crate::state::APP_STATE_VERSION;
use crate::{error::AppError, state::AppState};
use lazy_static::lazy_static;
use log::{debug, warn};
use std::io::{copy, Cursor};
use std::{fs, sync::Mutex};
use tauri::Manager;
use tokio::{
    fs::{read, remove_file, File},
    io::AsyncWriteExt,
};
// This file uses both std::fs::File and tokio::fs::File, please be aware of the difference.
// One is imported above (tokio::fs::File) and the other is qualified in line 134 (std::fs::File).

// The persistence.rs is where we define our app persistence functions.

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref ASSETS_DIR: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// The largest asset (logo) that is stored, in bytes.
const MAX_ASSET_SIZE: usize = 1_024 * 1_024 * 2;

pub const SUPPORTED_IMAGE_ASSET_EXTENSIONS: [&str; 3] = ["svg", "png", "jpg"];

/// Initialize the storage file paths.
pub fn initialize_storage(app_handle: &tauri::AppHandle) -> Result<(), AppError> {
    // TODO: create folder if not exists (not automatically created on macOS)
    if cfg!(target_os = "android") {
        *STATE_FILE.lock().unwrap() = app_handle.path().data_dir()?.join("state.json");
        *STRONGHOLD.lock().unwrap() = app_handle.path().data_dir()?.join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_handle.path().data_dir()?.join("assets");
    } else {
        let app_data_dir = app_handle.path().app_data_dir()?;
        *STATE_FILE.lock().unwrap() = app_data_dir.join("state.json");
        *STRONGHOLD.lock().unwrap() = app_data_dir.join("stronghold.bin");
        *ASSETS_DIR.lock().unwrap() = app_data_dir.join("assets");
    }
    debug!("STATE_FILE: {}", STATE_FILE.lock().unwrap().display());
    debug!("STRONGHOLD: {}", STRONGHOLD.lock().unwrap().display());

    match fs::create_dir_all(ASSETS_DIR.lock().unwrap().as_path()) {
        Ok(_) => debug!("ASSETS_DIR: created"),
        Err(e) => debug!("ASSETS_DIR: {e}"),
    };

    Ok(())
}

// State persistence functions.

/// Loads an [AppState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state() -> Result<AppState, AppError> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let bytes = read(state_file).await?;
    let content = String::from_utf8(bytes.clone()).map_err(|e| AppError::Error(e.utf8_error().to_string()))?;

    // Load state into a `serde_json::Object` first to run data model migrations before deserialization into `AppState`.
    let app_state_object: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&content)?;
    debug!("state loaded from disk");

    // Get the version from the state file, if no version field is found, default to 0.
    let version = app_state_object
        .get("version")
        .unwrap_or(&serde_json::Value::from(0))
        .as_u64()
        .map(|v| v as u32)
        .ok_or_else(|| {
            AppError::AppStateMigrationError(
                0,
                APP_STATE_VERSION,
                "Failed to get valid current version from AppState loaded from disk".to_string(),
            )
        })?;

    let app_state: AppState = match version {
        APP_STATE_VERSION => {
            let app_state = serde_json::from_str(&content)?;
            debug!("App state is already at version {APP_STATE_VERSION} (latest)");
            app_state
        }
        _ => {
            let app_state = apply_state_migrations(app_state_object, version)?;
            debug!("App state is now at version {APP_STATE_VERSION} (latest)");

            save_state(&app_state)
                .await
                .map_err(|_| AppError::Error("Failed to save state after applying migrations".to_string()))?;

            app_state
        }
    };

    // A result pop-up belongs to the session it happened in.
    Ok(AppState {
        interaction_result: None,
        ..app_state
    })
}

/// Persists a [AppState] to the app's data directory.
pub async fn save_state(app_state: &AppState) -> Result<(), AppError> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let mut file = File::create(state_file).await?;

    // Here we take out the credentials field before saving the state,
    // being sensitive data they should only be stored in the stronghold, nowhere else.
    let mut json_app_state = serde_json::to_value(app_state)?;
    json_app_state["credentials"] = serde_json::Value::Array(Vec::new());

    file.write_all(serde_json::to_string(app_state)?.as_bytes()).await?;
    debug!("state saved to disk");
    Ok(())
}

/// Removes the state file from the app's data directory.
pub async fn delete_state_file() -> Result<(), AppError> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    remove_file(state_file).await?;
    debug!("state deleted from disk");
    Ok(())
}

/// Removes the stronghold file from the app's data directory.
pub async fn delete_stronghold() -> Result<(), AppError> {
    let stronghold_file = STRONGHOLD.lock().unwrap().clone();
    remove_file(&stronghold_file).await?;
    remove_file(stronghold_file.join(".snapshot")).await?;
    debug!("stronghold deleted from disk");
    Ok(())
}

// Asset persistence functions

/// Clears the `/assets/tmp` folder inside the system-specific data directory.
/// This prevents downloaded assets that are only needed one single time or that receive no further processing from
/// cluttering the data directory and filling up space ("dead files").
pub fn clear_assets_tmp_folder() -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");
    if tmp_dir.exists() {
        std::fs::remove_dir_all(tmp_dir)?;
    }
    debug!("Successfully removed `/assets/tmp` folder and all its contents.");
    Ok(())
}

/// Clears the `/assets` folder inside the system-specific data directory.
/// This is only used when resetting the app to factory defaults.
pub fn clear_all_assets() -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    if assets_dir.exists() {
        std::fs::remove_dir_all(assets_dir.clone())?;
        std::fs::create_dir_all(assets_dir)?;
    }
    debug!("Successfully removed all items inside `/assets` folder.");
    Ok(())
}

/// Downloads an asset to the system-specific data directory.
/// The file is saved into the `assets/tmp` folder.
/// Since the `assets/tmp` folder is cleared on each app restart,
/// the downloaded file needs to be further treated and moved out of the `/tmp` folder
/// by running `persist_asset()`.
///
/// Restrictions:
/// - max. file size: 2 MB
/// - supported file types: `.png`, `.jpg`, `.svg`
/// - the URL, and every redirect it takes, has to point at a public host
/// - a `data:` URL is decoded locally instead of being requested
pub async fn download_asset(url: reqwest::Url, id: &str) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    // Create `/tmp` folder if not exists
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let (file_extension, bytes) = if url.scheme() == "data" {
        decode_data_url(url.as_str())?
    } else {
        let response = fetch_public_destination(url).await?;

        let file_extension = response
            .headers()
            .get("content-type")
            .map(|header_value| file_extension_for(header_value.to_str().unwrap_or_default()))
            .unwrap_or(Err(AppError::DownloadAborted("content-type is not set")))?;

        (file_extension, response.bytes().await?.to_vec())
    };

    let mut content = Cursor::new(bytes);

    // Abort download if file size is bigger than 2 MB
    if content.get_ref().len() > MAX_ASSET_SIZE {
        return Err(AppError::DownloadAborted("File size is bigger than 2 MB"));
    }

    let mut file = std::fs::File::create(tmp_dir.join(format!("{id}.{file_extension}")))?;

    copy(&mut content, &mut file)?;

    Ok(())
}

/// Maps the media type of a downloaded asset to the extension it is stored with.
fn file_extension_for(media_type: &str) -> Result<&'static str, AppError> {
    match media_type
        .split(';')
        .next()
        .unwrap_or_default()
        .trim()
        .to_ascii_lowercase()
        .as_str()
    {
        "image/png" => Ok("png"),
        "image/jpeg" | "image/jpg" => Ok("jpg"),
        "image/svg+xml" => Ok("svg"),
        _ => {
            warn!("content_type is not supported: {media_type:?}");
            Err(AppError::DownloadAborted("content-type is not supported"))
        }
    }
}

/// Decodes a `data:` URL (RFC 2397), such as an inline SVG logo in issuer metadata, into its extension and bytes.
///
/// Only the image types accepted for downloads are supported. The image is shown through an `<img>` element like any
/// downloaded asset, so an SVG's scripts don't run.
fn decode_data_url(data_url: &str) -> Result<(&'static str, Vec<u8>), AppError> {
    // Percent-encoding at most triples the size of the 2 MB asset, so anything longer is rejected before it's decoded.
    if data_url.len() > MAX_ASSET_SIZE * 3 {
        return Err(AppError::DownloadAborted("File size is bigger than 2 MB"));
    }

    let (header, payload) = data_url
        .strip_prefix("data:")
        .and_then(|rest| rest.split_once(','))
        .ok_or(AppError::DownloadAborted("data URL is malformed"))?;

    let mut parameters = header.split(';');
    let file_extension = file_extension_for(parameters.next().unwrap_or_default())?;
    let is_base64 = parameters.any(|parameter| parameter.trim().eq_ignore_ascii_case("base64"));

    let percent_decoded = urlencoding::decode_binary(payload.as_bytes()).into_owned();
    let bytes = if is_base64 {
        use base64::{
            alphabet::STANDARD,
            engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
            Engine,
        };
        // Like browsers, accept Base64 with or without padding.
        const BASE64: GeneralPurpose = GeneralPurpose::new(
            &STANDARD,
            GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent),
        );
        let base64: Vec<u8> = percent_decoded
            .into_iter()
            .filter(|byte| !byte.is_ascii_whitespace())
            .collect();
        BASE64
            .decode(base64)
            .map_err(|_| AppError::DownloadAborted("data URL is not valid base64"))?
    } else {
        percent_decoded
    };

    Ok((file_extension, bytes))
}

/// Persists an asset from the `/assets/tmp` folder to the `/assets` folder inside the system-specific data directory.
pub fn persist_asset(file_name: &str, id: &str) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    if let Some(extension) = SUPPORTED_IMAGE_ASSET_EXTENSIONS
        .iter()
        .find(|&e| tmp_dir.join(format!("{file_name}.{e}")).exists())
    {
        let new_file_name = format!("{id}.{extension}");
        std::fs::rename(
            tmp_dir.join(format!("{file_name}.{extension}")),
            assets_dir.join(&new_file_name),
        )?;

        // The frontend picks the first extension it finds, so a stale asset with another extension would shadow this one.
        for stale_extension in SUPPORTED_IMAGE_ASSET_EXTENSIONS.iter().filter(|&e| e != extension) {
            let stale_file = assets_dir.join(format!("{id}.{stale_extension}"));
            if stale_file.exists() {
                std::fs::remove_file(stale_file)?;
            }
        }

        debug!("Successfully persisted asset `{file_name}` --> `{new_file_name}`.");
    } else {
        warn!("No asset found for file_name: `{file_name}`")
    };

    Ok(())
}

/// Hashes a given URL string to a unique SHA-256 string.
/// Used for temporary asset file names in `/assets/tmp` to prevent unintended frontend image caching.
pub fn hash(url: &str) -> String {
    sha256::digest(url).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    #[serial_test::serial]
    fn persist_asset_replaces_asset_with_other_extension() {
        let assets_dir = TempDir::new().unwrap().keep();
        *ASSETS_DIR.lock().unwrap() = assets_dir.clone();
        fs::create_dir_all(assets_dir.join("tmp")).unwrap();
        fs::write(assets_dir.join("conn-123.svg"), b"old-logo").unwrap();
        fs::write(assets_dir.join("tmp").join("logo-hash.png"), b"new-logo").unwrap();

        persist_asset("logo-hash", "conn-123").unwrap();

        assert!(!assets_dir.join("conn-123.svg").exists());
        assert_eq!(fs::read(assets_dir.join("conn-123.png")).unwrap(), b"new-logo");
    }

    #[test]
    #[serial_test::serial]
    fn persist_asset_keeps_existing_asset_when_rename_fails() {
        let assets_dir = TempDir::new().unwrap().keep();
        *ASSETS_DIR.lock().unwrap() = assets_dir.clone();
        fs::create_dir_all(assets_dir.join("tmp")).unwrap();
        fs::write(assets_dir.join("conn-123.svg"), b"old-logo").unwrap();
        fs::write(assets_dir.join("tmp").join("logo-hash.png"), b"new-logo").unwrap();
        // A non-empty directory at the destination makes the rename fail on every platform.
        fs::create_dir_all(assets_dir.join("conn-123.png").join("blocker")).unwrap();

        assert!(persist_asset("logo-hash", "conn-123").is_err());

        assert_eq!(fs::read(assets_dir.join("conn-123.svg")).unwrap(), b"old-logo");
    }

    #[test]
    fn decode_data_url_reads_percent_encoded_svg() {
        let (extension, bytes) =
            decode_data_url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%2F%3E")
                .unwrap();

        assert_eq!(extension, "svg");
        assert_eq!(bytes, br#"<svg xmlns="http://www.w3.org/2000/svg"/>"#);
    }

    #[test]
    fn decode_data_url_reads_base64_png() {
        let (extension, bytes) = decode_data_url("data:image/png;base64,iVBORw0KGgo=").unwrap();

        assert_eq!(extension, "png");
        assert_eq!(bytes, b"\x89PNG\r\n\x1a\n");

        // Browsers accept Base64 without padding too.
        let (_, unpadded) = decode_data_url("data:image/png;base64,iVBORw0KGgo").unwrap();
        assert_eq!(unpadded, bytes);
    }

    #[test]
    fn decode_data_url_rejects_oversized_urls() {
        let data_url = format!("data:image/svg+xml,{}", "a".repeat(MAX_ASSET_SIZE * 3));

        assert!(decode_data_url(&data_url).is_err());
    }

    #[test]
    #[serial_test::serial]
    fn persist_asset_moves_jpeg() {
        let assets_dir = TempDir::new().unwrap().keep();
        *ASSETS_DIR.lock().unwrap() = assets_dir.clone();
        fs::create_dir_all(assets_dir.join("tmp")).unwrap();
        fs::write(assets_dir.join("tmp").join("logo-hash.jpg"), b"jpeg-logo").unwrap();

        persist_asset("logo-hash", "credential-123").unwrap();

        assert_eq!(fs::read(assets_dir.join("credential-123.jpg")).unwrap(), b"jpeg-logo");
    }

    #[test]
    fn decode_data_url_rejects_other_media_types_and_malformed_urls() {
        assert!(decode_data_url("data:text/html,%3Cscript%3Ealert(1)%3C%2Fscript%3E").is_err());
        assert!(decode_data_url("data:image/svg+xml").is_err());
        assert!(decode_data_url("data:image/png;base64,not base64!").is_err());
    }
}
