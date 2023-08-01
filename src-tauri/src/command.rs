use log::{info, warn};
use oid4vci::credential_offer::CredentialOfferQuery;
use siopv2::RequestUrl;

use crate::state::actions::{Action, ActionType};
use crate::state::persistence::{delete_state_file, delete_stronghold, load_state, save_state};
use crate::state::reducers::authorization::{read_authorization_request, send_authorization_response};
use crate::state::reducers::credential_offer::{read_credential_offer, send_credential_request};
use crate::state::reducers::{create_did_key, initialize_stronghold, load_dev_profile, reset_state, set_locale};
use crate::state::user_flow::{CurrentUserFlow, CurrentUserFlowType, Redirect};
use crate::state::{AppState, TransferState};

#[async_recursion::async_recursion]
pub(crate) async fn handle_action_inner<R: tauri::Runtime>(
    Action { r#type, payload }: Action,
    _app_handle: tauri::AppHandle<R>,
    app_state: &AppState,
) -> Result<(), String> {
    info!("received action `{:?}` with payload `{:?}`", r#type, payload);

    match r#type {
        ActionType::GetState => {
            let transfer_state: TransferState = load_state().await.unwrap_or(TransferState {
                active_profile: None,
                locale: "en".to_string(),
                credential_offer: None,
                credentials: None,
                current_user_flow: Some(CurrentUserFlow::Redirect(Redirect {
                    r#type: CurrentUserFlowType::Redirect,
                    target: "welcome".to_string(),
                })),
                debug_messages: vec![],
            });

            // TODO: find a better way to populate all fields with values from json file
            *app_state.active_profile.lock().unwrap() = transfer_state.active_profile;
            *app_state.locale.lock().unwrap() = transfer_state.locale;
            *app_state.credentials.lock().unwrap() = transfer_state.credentials;

            // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
            *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "welcome".to_string(),
            }));

            if (*app_state.active_profile.lock().unwrap()).is_some() {
                *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
                    r#type: CurrentUserFlowType::Redirect,
                    target: "profile".to_string(),
                }));
            }
        }
        ActionType::Reset => {
            if reset_state(app_state, Action { r#type, payload }).is_ok() {
                delete_state_file().await.ok();
                delete_stronghold().await.ok();
            }
        }
        ActionType::CreateNew => {
            let action = Action { r#type, payload };
            if initialize_stronghold(app_state, action.clone()).await.is_ok() {
                save_state(TransferState::from(app_state)).await.ok();
            }
            if create_did_key(app_state, action).await.is_ok() {
                save_state(TransferState::from(app_state)).await.ok();
            }
            // When everything is done, we redirect the user to the profile page
            *app_state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Redirect(Redirect {
                r#type: CurrentUserFlowType::Redirect,
                target: "profile".to_string(),
            }));
            save_state(TransferState::from(app_state)).await.ok();
        }
        ActionType::SetLocale => {
            if set_locale(app_state, Action { r#type, payload }).is_ok() {
                save_state(TransferState::from(app_state)).await.ok();
            }
            *app_state.current_user_flow.lock().unwrap() = None;
            save_state(TransferState::from(app_state)).await.ok();
        }
        ActionType::QrCodeScanned => {
            info!("qr code scanned: `{:?}`", payload);
            info!("Now doing some backend business logic with the QR code data...");
            let payload = payload.ok_or(anyhow::anyhow!("unable to read payload")).unwrap();

            let form_urlencoded = payload["form_urlencoded"].as_str().unwrap();

            if let Result::Ok(request_url) = form_urlencoded.parse::<RequestUrl>() {
                handle_action_inner(
                    Action {
                        r#type: ActionType::ReadRequest,
                        payload: Some(serde_json::to_value(request_url).unwrap()),
                    },
                    _app_handle,
                    app_state,
                )
                .await
                .ok();
            } else if let Result::Ok(credential_offer_query) = form_urlencoded.parse::<CredentialOfferQuery>() {
                handle_action_inner(
                    Action {
                        r#type: ActionType::ReadCredentialOffer,
                        payload: Some(serde_json::to_value(credential_offer_query).unwrap()),
                    },
                    _app_handle,
                    app_state,
                )
                .await
                .ok();
            } else {
                info!("Unable to parse QR code data");
            };
            save_state(TransferState::from(app_state)).await.ok();
        }
        ActionType::ReadRequest => {
            if read_authorization_request(app_state, Action { r#type, payload })
                .await
                .is_ok()
            {
                save_state(TransferState::from(app_state)).await.ok();
            }
        }
        ActionType::ReadCredentialOffer => {
            if read_credential_offer(app_state, Action { r#type, payload })
                .await
                .is_ok()
            {
                save_state(TransferState::from(app_state)).await.ok();
            }
        }
        ActionType::CancelUserFlow => {
            *app_state.current_user_flow.lock().unwrap() = None;
            save_state(TransferState::from(app_state)).await.ok();
        }
        ActionType::LoadDevProfile => {
            if load_dev_profile(app_state, Action { r#type, payload }).await.is_ok() {
                save_state(TransferState::from(app_state)).await.ok();
            }
        }
        ActionType::CredentialsSelected => {
            if send_authorization_response(app_state, Action { r#type, payload })
                .await
                .is_ok()
            {
                save_state(TransferState::from(app_state)).await.ok();
            }
        }
        ActionType::OffersSelected => {
            if send_credential_request(app_state, Action { r#type, payload })
                .await
                .is_ok()
            {
                save_state(TransferState::from(app_state)).await.ok();
            }
        }
        ActionType::Unknown => {
            warn!(
                "received unknown action type `{:?}` with payload `{:?}`",
                r#type, payload
            );
        }
    };
    Result::Ok(())
}

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
/// NOTE: Testing command handlers is not possible as of yet, see: https://github.com/tauri-apps/tauri/pull/4752
#[tauri::command]
pub async fn handle_action<R: tauri::Runtime>(
    action: Action,
    _app_handle: tauri::AppHandle<R>,
    app_state: tauri::State<'_, AppState>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    handle_action_inner(action, _app_handle, app_state.inner()).await.ok();

    let updated_state = TransferState::from(app_state.inner());
    emit_event(window, updated_state).ok();

    Result::Ok(())
}

fn emit_event<R: tauri::Runtime>(window: tauri::Window<R>, transfer_state: TransferState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, &transfer_state).unwrap();
    info!(
        "emitted event `{}` with payload `{:?}`",
        STATE_CHANGED_EVENT, &transfer_state
    );
    Ok(())
}
