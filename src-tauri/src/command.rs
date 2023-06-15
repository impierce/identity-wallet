use tracing::{info, warn};

use crate::did::persistence::load_existing_keypair;
use crate::state::actions::{Action, ActionType};
use crate::state::persistence::{delete_state_file, delete_stronghold, load_state, save_state};
use crate::state::reducers::{create_did_key, initialize_stronghold, load_dev_profile, reset_state, set_locale};
use crate::state::{AppState, TransferState};

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn handle_action(
    Action { r#type, payload }: Action,
    _app_handle: tauri::AppHandle,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window,
) -> Result<(), String> {
    info!("received action `{:?}` with payload `{:?}`", r#type, payload);

    match r#type {
        ActionType::GetState => {
            let transfer_state: TransferState = load_state().await.unwrap_or(TransferState {
                active_profile: None,
                locale: "en".to_string(),
                credentials: None,
            });

            let _keypair = match load_existing_keypair().await {
                Ok(keypair) => Some(keypair),
                Err(_) => {
                    info!("no existing keypair found");
                    None
                }
            };

            // TODO: find a better way to populate all fields with values from json file
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
        }
        ActionType::Reset => {
            if reset_state(app_state.inner(), Action { r#type, payload }).is_ok() {
                delete_state_file().await.ok();
                delete_stronghold().await.ok();
            }
        }
        ActionType::CreateNew => {
            let action = Action { r#type, payload };
            if initialize_stronghold(app_state.inner(), action.clone()).await.is_ok() {
                save_state(TransferState::from(app_state.inner())).await.ok();
            }
            if create_did_key(app_state.inner(), action).await.is_ok() {
                save_state(TransferState::from(app_state.inner())).await.ok();
            }
        }
        ActionType::SetLocale => {
            if set_locale(app_state.inner(), Action { r#type, payload }).is_ok() {
                save_state(TransferState::from(app_state.inner())).await.ok();
            }
        }
        ActionType::LoadDevProfile => {
            if load_dev_profile(app_state.inner(), Action { r#type, payload })
                .await
                .is_ok()
            {
                save_state(TransferState::from(app_state.inner())).await.ok();
            }
        }
        ActionType::Unknown => {
            warn!(
                "received unknown action type `{:?}` with payload `{:?}`",
                r#type, payload
            );
        }
    };

    let updated_state = TransferState::from(app_state.inner());
    emit_event(window, updated_state).ok();
    Ok(())
}

fn emit_event(window: tauri::Window, transfer_state: TransferState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, &transfer_state).unwrap();
    info!(
        "emitted event `{}` with payload `{:?}`",
        STATE_CHANGED_EVENT, &transfer_state
    );
    Ok(())
}
