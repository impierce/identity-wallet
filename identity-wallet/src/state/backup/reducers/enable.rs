use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

use log::info;
// use tauri_plugin_cloud_storage::CloudStorageExt;

pub async fn enable_backup(state: AppState, _action: Action) -> Result<AppState, AppError> {
    // tauri_plugin_cloud_storage::CloudStorageExt::cloud_storage(&self).ping(PingRequest {
    //     value: Some("ping".to_string()),
    // })?;
    // info!("response: {:?}", response);
    Ok(AppState { ..state })
}
