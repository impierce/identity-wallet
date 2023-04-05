use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio::fs::{read, remove_file, File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn};
use ts_rs::TS;

use crate::state::{AppState, Profile, StateStatus, TransferState};

/// Holds the command String and payload String of the command message coming from the frontend.
#[derive(Deserialize, TS)]
#[ts(export)]
pub struct CommandMessage {
    pub(crate) command: String,
    pub(crate) payload: String,
}

/// Commands that the backend knows how to handle.
enum Command {
    GetInitialState,
    Reset,
    CreateNew,
}

/// Maps a given string to a [Command].
impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "[INIT] Get initial state" => Ok(Command::GetInitialState),
            "[INIT] Reset" => Ok(Command::Reset),
            "[DID] Create new" => Ok(Command::CreateNew),
            _ => Err(()),
        }
    }
}

/// This command handler is the single point of entrance to the business logic in the backend. It will delegate the
/// command it receives and delegates its corresponding payload to the designated command function.
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn execute_command(
    CommandMessage { command, payload }: CommandMessage,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<TransferState, String> {
    info!("received command `{}` with payload `{}`", command, payload);

    let state_file_path = app_handle.path_resolver().app_data_dir().unwrap().join("state.json");

    let command = Command::from_str(&command).expect(&format!("Unknown command: `{}`", &command));

    match command {
        Command::GetInitialState => {
            let transfer_state: TransferState = load_state(app_handle).await;

            // TODO: find a better way to populate all fields with values from json file
            *app_state.status.lock().unwrap() = transfer_state.status;
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;

            let transfer_state = TransferState::from(app_state.inner());
            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
        Command::Reset => {
            remove_file(&state_file_path).await.unwrap();
            let initial_state = AppState::new(StateStatus::Stable, None);
            let transfer_state = TransferState::from(initial_state);
            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
        Command::CreateNew => {
            let profile = Profile {
                display_name: payload,
                primary_did: "did:atoi:123".to_string(),
            };
            let app_state = AppState::new(StateStatus::Stable, Some(profile));
            let transfer_state = TransferState::from(app_state);
            window.emit("state-changed", &transfer_state).unwrap();
            let mut f = File::create(&state_file_path).await.unwrap();
            f.write(serde_json::to_string(&transfer_state).unwrap().as_bytes())
                .await
                .unwrap();
            // let file = File::open(&file_path);
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
    }
}

/// Loads a [TransferState] from the app's data directory.
/// If it does not exist or it cannot be parsed, it will fallback to default values.
async fn load_state(app_handle: tauri::AppHandle) -> TransferState {
    let state_file_path = app_handle.path_resolver().app_data_dir().unwrap().join("state.json");
    let bytes = read(state_file_path).await;
    let transfer_state: TransferState = match bytes {
        Ok(successful_bytes) => {
            let content = String::from_utf8(successful_bytes);
            match content {
                Ok(successful_content) => {
                    let transfer_state_result: Result<TransferState, serde_json::Error> =
                        serde_json::from_str(&successful_content);
                    match transfer_state_result {
                        Ok(transfer_state) => {
                            info!("successfully loaded existing state `{:?}`", transfer_state);
                            transfer_state
                        }
                        Err(error) => {
                            warn!(
                                "could not load existing state. falling back to default. deserialization error? reason: {}",
                                error
                            );
                            TransferState {
                                status: Default::default(),
                                active_profile: Default::default(),
                            }
                        }
                    }
                }
                Err(error) => {
                    warn!(
                        "could not load existing state, falling back to default. reason: {}",
                        error
                    );
                    TransferState {
                        status: Default::default(),
                        active_profile: Default::default(),
                    }
                }
            }
        }
        Err(error) => {
            warn!(
                "could not load existing state, falling back to default. not exists? reason: {}",
                error
            );
            TransferState {
                status: Default::default(),
                active_profile: Default::default(),
            }
        }
    };
    transfer_state
}
