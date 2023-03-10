use serde::Deserialize;
use tracing::info;

use crate::state::{AppState, TransferState};

/// Holds the command String and payload String of the command message coming from the frontend.
#[derive(Deserialize)]
pub struct CommandMessage {
    pub command: String,
    pub payload: String,
}

/// This command handler is the single point of entrance to the business logic in the backend. It will delegate the
/// command it receives and delegates its corresponding payload to the designated command function.
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn execute_command(
    CommandMessage { command, payload }: CommandMessage,
    app_state: tauri::State<'_, AppState>,
) -> Result<TransferState, String> {
    info!("command `{}` received with payload `{}`", command, payload);

    match command.as_str() {
        "create_stronghold" => create_stronghold(payload, app_state).await,
        "create_account" => create_account(app_state).await,
        "get_did" => get_did(app_state).await,
        _ => Err(format!("Invalid command: {}", command)),
    }
}

async fn create_stronghold(_payload: String, _state: tauri::State<'_, AppState>) -> Result<TransferState, String> {
    todo!();
}

async fn create_account(_state: tauri::State<'_, AppState>) -> Result<TransferState, String> {
    todo!();
}

async fn get_did(_state: tauri::State<'_, AppState>) -> Result<TransferState, String> {
    todo!();
}
