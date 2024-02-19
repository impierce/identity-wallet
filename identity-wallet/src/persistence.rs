use crate::{error::AppError, state::AppState};
use tokio::{
    fs::{read, remove_file, File},
    io::AsyncWriteExt,
};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use std::{fs, io::{copy, Cursor}, sync::Mutex};
use tauri::Manager;
use strum::Display;
// This file uses both std::fs::File and tokio::fs::File, please be aware of the difference.
// One is imported (tokio::fs::File) and the other is qualified in line 129 (std::fs::File).

lazy_static! {
    pub static ref STATE_FILE: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref STRONGHOLD: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
    pub static ref ASSETS_DIR: Mutex<std::path::PathBuf> = Mutex::new(std::path::PathBuf::new());
}

/// The persistence.rs is where we define our app persistence functions.

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


/// Loads an [AppState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state() -> anyhow::Result<AppState> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let bytes = read(state_file).await?;
    let content = String::from_utf8(bytes)?;
    let app_state: AppState = serde_json::from_str(&content)?;
    debug!("state loaded from disk");
    Ok(app_state)
}

/// Persists an [AppState] to the app's data directory.
pub async fn save_state(app_state: &AppState) -> anyhow::Result<()> {
    let state_file = STATE_FILE.lock().unwrap().clone();
    let mut file = File::create(state_file).await?;

    let mut json_app_state = serde_json::to_value(app_state)?;
    json_app_state["credentials"] = serde_json::Value::Array(Vec::new());

    file.write_all(json_app_state.to_string().as_bytes()).await?;
    debug!("state saved to disk");
    Ok(())
}

// Removes the state file from the app's data directory.
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

/// Downloads an asset to the system-specific data directory.
/// The file is saved into the `assets/tmp` folder.
/// Since the `/tmp` folder is cleared on each app restart,
/// the downloaded file needs to be further treated and moved out of the `/tmp` folder.
///
/// Restrictions:
/// - max. file size: 2MB
/// - supported file types: `.png`, `.svg`
pub async fn download_asset(url: reqwest::Url, logo_type: LogoType, index: usize) -> Result<(), AppError> {
    let file_name = url.path_segments().unwrap().last().unwrap();
    let extension = file_name.split('.').last().unwrap();

    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    // Create `/tmp` folder if not exists
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    // TODO: in batch offer, use format!("{}_{}.{}", logo_type, index, extension)
    let mut file = std::fs::File::create(tmp_dir.join(format!("{}_{}.{}", logo_type, index, extension)))?;

    let response = reqwest::get(url.clone()).await?;

    if let Some(content_type) = response.headers().get("content-type") {
        if !["image/png", "image/svg+xml"].contains(&content_type.to_str().unwrap()) {
            warn!("content_type: {:?}", content_type);
            return Err(AppError::DownloadAborted("content-type is not supported"));
        }
    } else {
        return Err(AppError::DownloadAborted("content-type is not set"));
    };

    let mut content = Cursor::new(response.bytes().await?);

    // Abort download if file size is bigger than 2MB
    if content.get_ref().len() > 1_024 * 1_024 * 2 {
        return Err(AppError::DownloadAborted("File size is bigger than 2MB"));
    }

    copy(&mut content, &mut file)?;

    Ok(())
}

#[derive(Display)]
pub enum LogoType {
    #[strum(serialize = "issuer")]
    IssuerLogo,
    #[strum(serialize = "credential")]
    CredentialLogo,
}

/// Moves an asset from the `/assets/tmp` folder to the `/assets` folder, 
/// conversing it from temporary to persistent.
pub fn persist_asset(file_name: &str, id: &str) -> Result<(), AppError> {
    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    let extensions = ["svg", "png"];

    if let Some(extension) = extensions
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
        debug!("No asset found for file_name: `{}`", file_name)
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::error::AppError;
    use crate::persistence::{download_asset, LogoType};

    #[tokio::test]
    async fn when_size_is_less_than_2mb_then_download_should_start() {
        let mock_server = MockServer::start().await;

        // generate 1MB of random bytes
        let random_bytes: Vec<u8> = (0..(1_024 * 1_024)).map(|_| rand::random::<u8>()).collect();

        Mock::given(method("GET"))
            .and(path("/image.png"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
            .expect(1)
            .mount(&mock_server)
            .await;

        assert!(download_asset(
            format!("{}/image.png", &mock_server.uri()).parse().unwrap(),
            LogoType::CredentialLogo,
            0
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn when_size_is_bigger_than_2mb_then_download_should_fail() {
        let mock_server = MockServer::start().await;

        // generate 3MB of random bytes
        let random_bytes: Vec<u8> = (0..(1_024 * 1_024 * 3)).map(|_| rand::random::<u8>()).collect();

        Mock::given(method("GET"))
            .and(path("/image.png"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(random_bytes, "image/png"))
            .expect(1)
            .mount(&mock_server)
            .await;

        assert!(download_asset(
            format!("{}/image.png", &mock_server.uri()).parse().unwrap(),
            LogoType::CredentialLogo,
            0
        )
        .await
        .is_err());
    }

    #[tokio::test]
    async fn when_content_type_is_supported_then_download_should_start() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/image"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/svg+xml"))
            .expect(1)
            .mount(&mock_server)
            .await;

        assert!(download_asset(
            format!("{}/image", &mock_server.uri()).parse().unwrap(),
            LogoType::CredentialLogo,
            0
        )
        .await
        .is_ok());
    }

    #[tokio::test]
    async fn when_content_type_is_not_supported_then_download_should_fail() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/image.png")) // file extension is ignored (even if it's supported), only content-type is checked
            .respond_with(ResponseTemplate::new(200).set_body_raw(vec![], "image/jpeg"))
            .expect(1)
            .mount(&mock_server)
            .await;

        assert!(matches!(
            download_asset(
                format!("{}/image.png", &mock_server.uri()).parse().unwrap(),
                LogoType::CredentialLogo,
                0
            )
            .await,
            Err(AppError::DownloadAborted("content-type is not supported"))
        ));
    }
}
