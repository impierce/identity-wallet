use tracing::info;

use crate::state::actions::{Action, ActionType};
use crate::state::persistence::{delete_state, load_state, save_state};
use crate::state::reducers::{create_did_key, reset_state, set_locale};
use crate::state::state::{AppState, TransferState};

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn handle_action(
    Action { r#type, payload }: Action,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), String> {
    info!("received action `{:?}` with payload `{:?}`", r#type, payload);

    // TODO: be more redux-idiomatic: do not deserialize to "known" actions, but return the state unchanged if the action is unknown

    match r#type {
        ActionType::GetState => {
            let transfer_state: TransferState = load_state(app_handle).await.unwrap_or_default();

            // TODO: find a better way to populate all fields with values from json file
            *app_state.status.lock().unwrap() = transfer_state.status;
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
        }
        ActionType::Reset => {
            if reset_state(app_state.inner(), Action { r#type, payload }).is_ok() {
                delete_state(app_handle).await.ok();
            }
        }
        ActionType::CreateNew => {
            if create_did_key(app_state.inner(), Action { r#type, payload }).is_ok() {
                save_state(app_handle, TransferState::from(app_state.inner()))
                    .await
                    .ok();
            }
        }
        ActionType::SetLocale => {
            if set_locale(app_state.inner(), Action { r#type, payload }).is_ok() {
                save_state(app_handle, TransferState::from(app_state.inner()))
                    .await
                    .ok();
            }
        }
    };

    let updated_state = TransferState::from(app_state.inner());
    emit_event(window, updated_state).ok();
    Ok(())
}

fn emit_event(window: tauri::Window, transfer_state: TransferState) -> anyhow::Result<()> {
    window.emit("state-changed", &transfer_state).unwrap();
    info!(
        "emitted event `{}` with payload `{:?}`",
        "state-changed", &transfer_state
    );
    Ok(())
}
