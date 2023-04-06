use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio::fs::{read, remove_file, File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn};
use ts_rs::TS;

use crate::clients::iota::create_iota_identity;
use crate::state::persistence::{load_state, save_state};
use crate::state::state::{AppState, Profile, StateStatus, TransferState};

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
    SetLocale,
}

/// Maps a given string to a [Command].
impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Command, Self::Err> {
        match s {
            "[INIT] Get initial state" => Ok(Command::GetInitialState),
            "[INIT] Reset" => Ok(Command::Reset),
            "[DID] Create new" => Ok(Command::CreateNew),
            "[SETTINGS] Set locale" => Ok(Command::SetLocale),
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
            let transfer_state: TransferState = load_state(app_handle).await.unwrap_or_default();

            // TODO: find a better way to populate all fields with values from json file
            *app_state.status.lock().unwrap() = transfer_state.status;
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;

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

            *app_state.status.lock().unwrap() = StateStatus::Stable;
            *app_state.active_profile.lock().unwrap() = None;
            *app_state.locale.lock().unwrap() = "en".to_string();

            let transfer_state = TransferState::from(app_state.inner());
            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
        Command::CreateNew => {
            // let iota_identity = create_iota_identity();

            let profile = Profile {
                display_name: payload,
                primary_did: "did:atoi:123".to_string(),
            };

            *app_state.status.lock().unwrap() = StateStatus::Stable;
            *app_state.active_profile.lock().unwrap() = Some(profile);

            let transfer_state = TransferState::from(app_state.inner());

            save_state(app_handle, transfer_state.clone()).await.unwrap();

            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
        Command::SetLocale => {
            *app_state.locale.lock().unwrap() = payload;

            let transfer_state = TransferState::from(app_state.inner());

            save_state(app_handle, transfer_state.clone()).await.unwrap();

            window.emit("state-changed", &transfer_state).unwrap();
            info!(
                "emitted event `{}` with payload `{:?}`",
                "state-changed", &transfer_state
            );
            Ok(transfer_state)
        }
    }
}
