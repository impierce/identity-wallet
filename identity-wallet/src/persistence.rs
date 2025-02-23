use crate::state::APPSTATE_VERSION;
use crate::{error::AppError, state::AppState};
use lazy_static::lazy_static;
use log::info;
use log::{debug, warn};
use serde_json::{Map, Value};
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

pub const SUPPORTED_IMAGE_ASSET_EXTENSIONS: [&str; 2] = ["svg", "png"];

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

    match fs::create_dir_all(ASSETS_DIR.lock().unwrap().as_path()) {
        Ok(_) => info!("ASSETS_DIR: created"),
        Err(e) => info!("ASSETS_DIR: {}", e),
    };

    Ok(())
}

// State persistence functions.

/// Loads an [AppState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state() -> anyhow::Result<AppState> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let bytes = read(state_file).await?;
    let content = String::from_utf8(bytes)?;

    // Load state to a serde_json::Object instead of direct deserialization to avoid different versions breaking.
    let app_state_object: Map<String, Value> = serde_json::from_str(&content)?;
    let version = app_state_object.get("version").unwrap().as_u64().unwrap(); // TODO: remove unwrap
    let app_state: AppState = match version {
        APPSTATE_VERSION => {
            let app_state = serde_json::from_str(&content)?;
            debug!("state loaded from disk");
            app_state
        }
        _ => {
            debug!(
                "state version mismatch, performing data migration and appstate update from {} to {}",
                version, APPSTATE_VERSION
            );
            let app_state = appstate_version_update(app_state_object, version).await?;
            debug!("state successfully loaded from disk and updated");
            app_state
        }
    };

    Ok(app_state)
}

/// Persists a [AppState] to the app's data directory.
pub async fn save_state(app_state: &AppState) -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let mut file = File::create(state_file).await?;

    // Here we take out the credentials field before saving the state,
    // being sensitive data they should only be stored in the stronghold, nowhere else.
    let mut json_app_state = serde_json::to_value(app_state)?;
    json_app_state["credentials"] = Value::Array(Vec::new());

    file.write_all(serde_json::to_string(app_state)?.as_bytes()).await?;
    debug!("state saved to disk");
    Ok(())
}

/// Removes the state file from the app's data directory.
pub async fn delete_state_file() -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    remove_file(state_file).await?;
    debug!("state deleted from disk");
    Ok(())
}

/// Removes the stronghold file from the app's data directory.
pub async fn delete_stronghold() -> anyhow::Result<()> {
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
/// - supported file types: `.png`, `.svg`
pub async fn download_asset(url: reqwest::Url, id: &str) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    // Create `/tmp` folder if not exists
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let response = reqwest::get(url.clone()).await?;

    let file_extension = response
        .headers()
        .get("content-type")
        .map(|header_value| match header_value.to_str().unwrap() {
            "image/png" => Ok("png"),
            "image/svg+xml" => Ok("svg"),
            _ => {
                warn!("content_type is not supported: {:?}", header_value);
                Err(AppError::DownloadAborted("content-type is not supported"))
            }
        })
        .unwrap_or(Err(AppError::DownloadAborted("content-type is not set")))?;

    let mut content = Cursor::new(response.bytes().await?);

    // Abort download if file size is bigger than 2 MB
    if content.get_ref().len() > 1_024 * 1_024 * 2 {
        return Err(AppError::DownloadAborted("File size is bigger than 2 MB"));
    }

    let mut file = std::fs::File::create(tmp_dir.join(format!("{}.{}", id, file_extension)))?;

    copy(&mut content, &mut file)?;

    Ok(())
}

/// Persists an asset from the `/assets/tmp` folder to the `/assets` folder inside the system-specific data directory.
pub fn persist_asset(file_name: &str, id: &str) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    if let Some(extension) = SUPPORTED_IMAGE_ASSET_EXTENSIONS
        .iter()
        .find(|&e| tmp_dir.join(format!("{}.{}", file_name, e)).exists())
    {
        let new_file_name = format!("{}.{}", id, extension);
        std::fs::rename(
            tmp_dir.join(format!("{}.{}", file_name, extension)),
            assets_dir.join(&new_file_name),
        )?;
        debug!("Successfully persisted asset `{}` --> `{}`.", file_name, new_file_name);
    } else {
        warn!("No asset found for file_name: `{}`", file_name)
    };

    Ok(())
}

/// Hashes a given URL string to a unique SHA-256 string.
/// Used for temporary asset file names in `/assets/tmp` to prevent unintended frontend image caching.
pub fn hash(url: &str) -> String {
    sha256::digest(url).to_string()
}

// Data migration functions

/// This function is used to migrate the app state from one version to the next.
pub async fn appstate_version_update(
    mut app_state_object: Map<String, Value>,
    version: u64,
) -> anyhow::Result<AppState> {
    // Ordered vector of all migration functions up to the current version.
    let migrations: Vec<fn(&mut Map<String, Value>)> = vec![];

    for (index, migration) in migrations.iter().enumerate() {
        if (index + 1) >= version as usize {
            migration(&mut app_state_object);
        }
    }

    let app_state_value = Value::Object(app_state_object);
    let app_state: AppState = serde_json::from_value(app_state_value)?;

    save_state(&app_state).await?;

    info!(
        "state successfully migrated from version {} to {}.\n AppState:\n{:?}",
        version, APPSTATE_VERSION, app_state
    );

    Ok(app_state)
}

#[cfg(test)]
mod tests {
    use crate::state::AppState;
    use serde_json::{Map, Value};

    // To unit test the appstate_version_update function, we need to create an adapted function.
    // This is to bypass the need for a true STATE_FILE due to the fn `save_state`.
    // Also to bypass migration functions, since we don't have multiple AppState versions yet.

    pub async fn dummy_appstate_version_update(
        mut app_state_object: Map<String, Value>,
        version: u64,
    ) -> anyhow::Result<AppState> {
        let migrations: Vec<fn(&mut Map<String, Value>)> = vec![dummy_migrate_v0_to_v1];

        for (index, migration) in migrations.iter().enumerate() {
            if (index + 1) >= version as usize {
                migration(&mut app_state_object);
            }
        }

        let app_state_value = Value::Object(app_state_object);
        let app_state: AppState = serde_json::from_value(app_state_value)?;

        Ok(app_state)
    }

    fn dummy_migrate_v0_to_v1(app_state_object: &mut Map<String, Value>) {
        app_state_object.insert("version".to_string(), Value::from(1));
    }

    #[tokio::test]
    async fn appstate_version_update_test() {
        let rdr =
            std::fs::File::open("../unime/src-tauri/tests/fixtures/states/no_profile_redirect_welcome.json").unwrap();

        let app_state_object: Map<String, Value> = serde_json::from_reader(&rdr).unwrap();
        let test_app_state = dummy_appstate_version_update(app_state_object, 1).await.unwrap();
        let test_app_state_str = serde_json::to_string(&test_app_state).unwrap();

        let const_app_state: AppState = serde_json::from_str(
            r#"
        {
            "version": 1,
            "current_user_prompt": {
                "type": "redirect",
                "target": "welcome"
            }
        }
        "#,
        )
        .unwrap();
        let const_app_state_str = serde_json::to_string(&const_app_state).unwrap();

        assert_eq!(test_app_state_str, const_app_state_str);
    }
}
