use crate::error::AppError;
use std::{
    fs::File,
    io::{copy, Cursor},
};

pub async fn download_asset(url: &str, file_name: &str) -> Result<(), AppError> {
    let response = reqwest::get(url).await?;

    // TODO: read from app_handle
    let data_dir = std::path::Path::new("/Users/daniel/Library/Application Support/com.impierce.identity_wallet");

    let mut file = File::create(data_dir.join("assets").join(file_name))?;

    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut file)?;

    Ok(())
}
