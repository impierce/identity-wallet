use serde::Deserialize;
use tracing::info;
use ts_rs::TS;

use crate::state::{AppState, TransferState};

/// Holds the command String and payload String of the command message coming from the frontend.
#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CommandMessage {
    pub(crate) command: String,
    pub(crate) payload: String,
}

/// This command handler is the single point of entrance to the business logic in the backend. It will delegate the
/// command it receives and delegates its corresponding payload to the designated command function.
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn execute_command(
    CommandMessage { command, payload }: CommandMessage,
    _app_state: tauri::State<'_, AppState>,
) -> Result<TransferState, String> {
    info!("command `{}` received with payload `{}`", command, payload);

    match command.as_str() {
        _ => Err(format!("Invalid command: {}", command)),
    }
}
