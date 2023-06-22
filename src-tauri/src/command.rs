use log::{info, warn};

use crate::state::actions::{Action, ActionType};
use crate::state::persistence::{delete_state_file, delete_stronghold, load_state, save_state};
use crate::state::reducers::{create_did_key, initialize_stronghold, load_dev_profile, reset_state, set_locale};
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect, Selection};
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
                current_user_flow: Some(CurrentUserFlow::Redirect(Redirect {
                    r#type: CurrentUserFlowType::Redirect,
                    target: "welcome".to_string(),
                })),
            });

            // TODO: find a better way to populate all fields with values from json file
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
            *app_state.credentials.lock().unwrap() = transfer_state.credentials;

            // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
            // *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
            //     r#type: CurrentUserFlowType::Redirect,
            //     target: "welcome".to_string(),
            // }));

            if (*app_state.active_profile.lock().unwrap()).is_some() {
                *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
                    r#type: CurrentUserFlowType::Redirect,
                    target: "profile".to_string(),
                }));
            }
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
            // When everything is done, we redirect the user to the profile page
            *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "profile".to_string(),
            }));
            save_state(TransferState::from(app_state.inner())).await.ok();
        }
        ActionType::SetLocale => {
            if set_locale(app_state.inner(), Action { r#type, payload }).is_ok() {
                save_state(TransferState::from(app_state.inner())).await.ok();
            }
            *app_state.current_user_flow.lock().unwrap() = None;
            save_state(TransferState::from(app_state.inner())).await.ok();
        }
        ActionType::QrCodeScanned => {
            info!("qr code scanned: `{:?}`", payload);
            info!("Now doing some backend business logic with the QR code data...");
            std::thread::sleep(std::time::Duration::from_millis(2_000));
            // TODO: actually do something with the QR code data
            *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Selection(Selection {
                r#type: CurrentUserFlowType::SelectCredentials,
                options: vec![
                    (
                        "givenName".to_string(),
                        "http://example.edu/credentials/3732".to_string(),
                    ), // claim name, credential id
                    (
                        "familyName".to_string(),
                        "http://example.edu/credentials/3732".to_string(),
                    ),
                    (
                        "birthdate".to_string(),
                        "http://example.edu/credentials/3732".to_string(),
                    ),
                    ("email".to_string(), "http://example.edu/credentials/3732".to_string()),
                ],
            }));
            // save_state(TransferState::from(app_state.inner())).await.ok();
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
