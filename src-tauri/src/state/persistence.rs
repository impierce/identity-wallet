use tokio::{
    fs::{read, File},
    io::AsyncWriteExt,
};
use tracing::{info, warn};

use crate::state::state::TransferState;

/// Loads a [TransferState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
pub async fn load_state(app_handle: tauri::AppHandle) -> anyhow::Result<TransferState> {
    let state_file_path = app_handle.path_resolver().app_data_dir().unwrap().join("state.json");
    let bytes = read(state_file_path).await?;
    let content = String::from_utf8(bytes)?;
    let transfer_state: TransferState = serde_json::from_str(&content).unwrap();
    info!("<<< successfully loaded state from disk");

    // let transfer_state: TransferState = match bytes {
    //     Ok(successful_bytes) => {
    //         let content = String::from_utf8(successful_bytes);
    //         match content {
    //             Ok(successful_content) => {
    //                 let transfer_state_result: Result<TransferState, serde_json::Error> =
    //                     serde_json::from_str(&successful_content);
    //                 match transfer_state_result {
    //                     Ok(transfer_state) => {
    //                         info!("successfully loaded existing state `{:?}`", transfer_state);
    //                         transfer_state
    //                     }
    //                     Err(error) => {
    //                         warn!(
    //                             "could not load existing state. falling back to default. deserialization error? reason: {}",
    //                             error
    //                         );
    //                         TransferState::default()
    //                     }
    //                 }
    //             }
    //             Err(error) => {
    //                 warn!(
    //                     "could not load existing state, falling back to default. reason: {}",
    //                     error
    //                 );
    //                 TransferState::default()
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         warn!(
    //             "could not load existing state, falling back to default. not exists? reason: {}",
    //             error
    //         );
    //         TransferState::default()
    //     }
    // };
    Ok(transfer_state)
}

/// Persists a [TransferState] to the app's data directory.
pub async fn save_state(app_handle: tauri::AppHandle, transfer_state: TransferState) -> anyhow::Result<()> {
    let state_file_path = app_handle.path_resolver().app_data_dir().unwrap().join("state.json");
    let mut file = File::create(state_file_path).await?;
    file.write_all(serde_json::to_string(&transfer_state).unwrap().as_bytes())
        .await?;
    info!(">>> successfully persisted state to disk");
    Ok(())
}
