use serde::Deserialize;
use tracing::info;
use ts_rs::TS;

use crate::state::{AppState, StateStatus, TransferState};

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
    window: tauri::Window,
) -> Result<TransferState, String> {
    info!("received command `{}` with payload `{}`", command, payload);

    match command.as_str() {
        "[INIT] Get initial state" => {
            let initial_state = AppState::new(StateStatus::Stable);
            let transfer_state = TransferState::from(initial_state);
            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
        _ => Err(format!("Invalid command: {}", command)),
    }
}
