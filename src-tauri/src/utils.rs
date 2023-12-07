use log::debug;

use crate::{error::AppError, ASSETS_DIR};
use std::{
    fs::File,
    io::{copy, Cursor},
};

/// Downloads an asset to the system-specific data directory.
/// The file is saved into the `assets/tmp` folder.
/// Since the `/tmp` folder is cleared on each app restart,
/// the downloaded file needs to be further treated and moved out of the `/tmp` folder.
///
/// Restrictions:
/// - max. file size: 2MB
/// - supported file types: `.png`, `.svg`
pub async fn download_asset(url: &str) -> Result<(), AppError> {
    let url: reqwest::Url = url.parse().unwrap();

    let file_name = url.path_segments().unwrap().last().unwrap();

    // Abort download if file type is not supported
    if !["png", "svg"].contains(&file_name.split('.').last().unwrap()) {
        return Err(AppError::DownloadAborted("MIME type is not supported"));
    }

    let assets_dir = ASSETS_DIR.lock().unwrap().as_path().to_owned();
    let tmp_dir = assets_dir.join("tmp");

    // Create `/tmp` folder if not exists
    if !tmp_dir.exists() {
        std::fs::create_dir(&tmp_dir)?;
    }

    let mut file = File::create(tmp_dir.join(file_name))?;

    let response = reqwest::get(url.clone()).await?;
    let mut content = Cursor::new(response.bytes().await?);

    // Abort download if file size is bigger than 2MB
    if content.get_ref().len() > 1_024 * 1_024 * 2 {
        return Err(AppError::DownloadAborted("File size is bigger than 2MB"));
    }

    copy(&mut content, &mut file)?;

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
