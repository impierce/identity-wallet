use std::str::FromStr;

use tokio::fs::{read, remove_file, File, OpenOptions};
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn};

use crate::clients::iota::create_iota_identity;
use crate::state::actions::{Action, KnownAction};
use crate::state::persistence::{load_state, save_state};
use crate::state::state::{AppState, Profile, StateStatus, TransferState};

/// This command handler is the single point of entrance to the business logic in the backend. It will delegate the
/// command it receives and delegates its corresponding payload to the designated command function.
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn execute_command(
    Action { r#type, payload }: Action,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<TransferState, String> {
    info!("received action `{}` with payload `{:?}`", r#type, payload);

    let state_file_path = app_handle.path_resolver().app_data_dir().unwrap().join("state.json");

    // TODO: better pattern would be to return the state unchanged if the action is unknown
    let action = KnownAction::from_str(&r#type).expect(&format!("Unknown action: `{}`", &r#type));

    // the match structure functions as the "root reducer" in the redux pattern
    match action {
        KnownAction::GetState => {
            let transfer_state: TransferState = load_state(app_handle).await.unwrap_or_default();

            // TODO: find a better way to populate all fields with values from json file
            *app_state.status.lock().unwrap() = transfer_state.status;
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
        }
        KnownAction::Reset => {
            remove_file(&state_file_path).await.unwrap();

            *app_state.status.lock().unwrap() = Default::default();
            *app_state.active_profile.lock().unwrap() = None;
            *app_state.locale.lock().unwrap() = "en".to_string();
        }
        KnownAction::CreateNew => {
            // let iota_identity = create_iota_identity();

            let mock_profile = Profile {
                display_name: payload.unwrap(),
                primary_did: "did:atoi:123".to_string(),
            };

            *app_state.status.lock().unwrap() = StateStatus::Stable;
            *app_state.active_profile.lock().unwrap() = Some(mock_profile);

            save_state(app_handle, TransferState::from(app_state.inner())).await.unwrap();
        }
        KnownAction::SetLocale => {
            *app_state.locale.lock().unwrap() = payload.unwrap();

            save_state(app_handle, TransferState::from(app_state.inner())).await.unwrap();
        },
    };

    let updated_state = TransferState::from(app_state.inner());
    emit_event(window, updated_state.clone()).unwrap();
    Ok(updated_state)
}

fn emit_event(window: tauri::Window, transfer_state: TransferState) -> anyhow::Result<()> {
    window.emit("state-changed", &transfer_state).unwrap();
    info!(
        "emitted event `{}` with payload `{:?}`",
        "state-changed", &transfer_state
    );
    Ok(())
}
