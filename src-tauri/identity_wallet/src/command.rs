use crate::error::AppError::{self, *};
use crate::state::actions::Action;
use crate::state::persistence::{delete_state_file, delete_stronghold, load_state, save_state};
use crate::state::reducers::authorization::{
    handle_oid4vp_authorization_request, handle_siopv2_authorization_request, read_authorization_request,
};
use crate::state::reducers::credential_offer::{read_credential_offer, send_credential_request};
use crate::state::reducers::dev_mode::{load_dev_profile, set_dev_mode};
use crate::state::reducers::storage::unlock_storage;
use crate::state::reducers::user_data_query::user_data_query;
use crate::state::reducers::{
    create_identity, initialize_stronghold, reset_state, set_locale, update_credential_metadata,
    update_profile_settings,
};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::{AppState, AppStateContainer};
use async_recursion::async_recursion;
use log::{debug, info, warn};
use oid4vci::credential_offer::CredentialOfferQuery;
use tauri::Manager;

#[async_recursion]
pub async fn handle_action_inner<R: tauri::Runtime>(
    app_state: &mut AppState,
    action: Action,
    _app_handle: tauri::AppHandle<R>,
) -> Result<(), AppError> {
    info!("received action `{action:?}`");

    match action {
        Action::GetState => {
            // TODO: find a better way to populate all fields with values from json file
            let loaded_state = load_state().await.unwrap_or_default();

            app_state.active_profile = loaded_state.active_profile.clone();
            app_state.locale = loaded_state.locale.clone();
            app_state.connections = loaded_state.connections.clone();
            app_state.debug_messages = loaded_state.debug_messages.clone();

            if app_state.active_profile.is_some() {
                app_state.current_user_prompt = Some(CurrentUserPrompt::PasswordRequired);
            } else {
                // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
                app_state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
                    target: "welcome".to_string(),
                });
            }

            app_state.dev_mode_enabled = loaded_state.dev_mode_enabled;
            // Overwrite dev_mode_enabled if environment variable is set
            if let Some(b) = std::env::var("DEV_MODE_ENABLED")
                .ok()
                .and_then(|s| s.parse::<bool>().ok())
            {
                app_state.dev_mode_enabled = b;
            }

            Ok(())
        }
        Action::UnlockStorage { .. } => unlock_storage(app_state, action).await,
        Action::Reset => {
            reset_state(app_state, action)?;
            delete_state_file().await.map_err(StateFileDeletionError)?;
            delete_stronghold().await.map_err(StrongholdDeletionError)
        }
        Action::CreateNew { .. } => {
            let action = action;
            initialize_stronghold(app_state, action.clone()).await?;
            create_identity(app_state, action).await?;

            // When everything is done, we redirect the user to the "me" page
            app_state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            });
            Ok(())
        }
        Action::SetLocale { .. } => set_locale(app_state, action),
        Action::UpdateProfileSettings { .. } => update_profile_settings(app_state, action),
        Action::QrCodeScanned { form_urlencoded } => {
            info!("qr code scanned: `{:?}`", form_urlencoded);

            let is_authorization_request = {
                let state_guard = app_state.managers.lock().await;
                let provider_manager = &state_guard
                    .identity_manager
                    .as_ref()
                    .ok_or(MissingManagerError("identity"))?
                    .provider_manager;

                provider_manager.validate_request(form_urlencoded.clone()).await.is_ok()
            };

            if is_authorization_request {
                handle_action_inner(
                    app_state,
                    Action::ReadRequest {
                        authorization_request: form_urlencoded,
                    },
                    _app_handle,
                )
                .await
            } else if let Result::Ok(credential_offer_uri) = form_urlencoded.parse::<CredentialOfferQuery>() {
                handle_action_inner(
                    app_state,
                    Action::ReadCredentialOffer { credential_offer_uri },
                    _app_handle,
                )
                .await
            } else {
                Err(InvalidQRCodeError(form_urlencoded))
            }
        }
        Action::ReadRequest { .. } => read_authorization_request(app_state, action).await,
        Action::ConnectionAccepted => handle_siopv2_authorization_request(app_state, action).await,
        Action::ReadCredentialOffer { .. } => read_credential_offer(app_state, action).await,
        Action::CancelUserFlow { redirect } => {
            if let Some(redirect) = redirect {
                app_state
                    .current_user_prompt
                    .replace(CurrentUserPrompt::Redirect { target: redirect });
            } else {
                app_state.current_user_prompt.take();
            }
            Ok(())
        }
        Action::SetDevMode { .. } => set_dev_mode(app_state, action).await,
        Action::LoadDevProfile => load_dev_profile(app_state, action).await,
        Action::CredentialsSelected { .. } => handle_oid4vp_authorization_request(app_state, action).await,
        Action::CredentialOffersSelected { .. } => send_credential_request(app_state, action).await,
        Action::UpdateCredentialMetadata { .. } => {
            update_credential_metadata(app_state, action).await?;
            app_state.current_user_prompt = None;
            Ok(())
        }
        Action::CancelUserJourney => {
            app_state.user_journey = None;
            Ok(())
        }
        Action::UserDataQuery(_) => user_data_query(app_state, action).await,
        Action::Unknown => Err(InvalidActionError { action }),
    }
}

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
pub async fn handle_action<R: tauri::Runtime>(
    action: Action,
    _app_handle: tauri::AppHandle<R>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    // Keep a copy of the state before it is altered, so that we can revert to it if the state update fails.
    let mut guard = container.0.lock().await;
    let app_state = &mut *guard;

    match handle_action_inner(app_state, action, _app_handle).await {
        Ok(()) => {
            debug!("state updated successfully");
            save_state(app_state).await.ok();
            emit_event(window, app_state).ok();
        }
        Err(error) => {
            // If the state update fails, we revert to the unaltered state and add the error to the debug messages.
            {
                let debug_messages = &mut app_state.debug_messages;
                while debug_messages.len() > 100 {
                    debug_messages.remove(0);
                }
                debug_messages.push_back(format!(
                    "{} {:?}",
                    chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    error
                ));
            }
            warn!("state update failed: {}", error);

            // Save and emit the unaltered state including the error message.
            save_state(app_state).await.ok();
            emit_event(window, app_state).ok();
        }
    }

    Result::Ok(())
}

pub fn emit_event<R: tauri::Runtime>(window: tauri::Window<R>, app_state: &AppState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, app_state)?;
    debug!("emitted event `{}` with payload `{:?}`", STATE_CHANGED_EVENT, app_state);
    Ok(())
}
