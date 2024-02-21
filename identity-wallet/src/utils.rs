use log::warn;
use std::{
    fs::File,
    io::{copy, Cursor},
};
use strum::Display;

use crate::{error::AppError, ASSETS_DIR};

/// Downloads an asset to the system-specific data directory.
/// The file is saved into the `assets/tmp` folder.
/// Since the `/tmp` folder is cleared on each app restart,
/// the downloaded file needs to be further treated and moved out of the `/tmp` folder.
///
/// Restrictions:
/// - max. file size: 2MB
/// - supported file types: `.png`, `.svg`
pub async fn download_asset(url: reqwest::Url, logo_type: LogoType, index: usize) -> Result<(), AppError> {
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

    // Abort download if file size is bigger than 2MB
    if content.get_ref().len() > 1_024 * 1_024 * 2 {
        return Err(AppError::DownloadAborted("File size is bigger than 2MB"));
    }

    let mut file = File::create(tmp_dir.join(format!("{}_{}.{}", logo_type, index, file_extension)))?;

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
