use tracing::info;

use crate::state::actions::{Action, ActionType};
use crate::state::persistence::{delete_state, load_state, save_state};
use crate::state::reducers::{create_did_key, reset_state, set_locale};
use crate::state::state::{AppState, TransferState};

/// This command handler is the single point of entrance to the business logic in the backend. It will delegate the
/// command it receives and delegates its corresponding payload to the designated command function.
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn handle_action(
    Action { r#type, payload }: Action,
    app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), String> {
    info!("received action `{:?}` with payload `{:?}`", r#type, payload);

    // TODO: redux-idiomatic: return the state unchanged if the action is unknown

    // This match structure functions as the "root reducer" (redux pattern)
    match r#type {
        ActionType::GetState => {
            let transfer_state: TransferState = load_state(app_handle).await.unwrap_or_default();

            // TODO: find a better way to populate all fields with values from json file
            *app_state.status.lock().unwrap() = transfer_state.status;
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
        }
        ActionType::Reset => {
            reset_state(app_state.inner(), Action { r#type, payload }).unwrap();
            delete_state(app_handle).await.unwrap();
        }
        ActionType::CreateNew => {
            create_did_key(app_state.inner(), Action { r#type, payload }).unwrap();
            save_state(app_handle, TransferState::from(app_state.inner()))
                .await
                .unwrap();
        }
        ActionType::SetLocale => {
            set_locale(app_state.inner(), Action { r#type, payload }).unwrap();
            save_state(app_handle, TransferState::from(app_state.inner()))
                .await
                .unwrap();
        }
    };

    let updated_state = TransferState::from(app_state.inner());
    emit_event(window, updated_state.clone()).unwrap();
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
