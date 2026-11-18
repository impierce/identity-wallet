use crate::error::AppError;
use crate::persistence::save_state;
use crate::state::actions::{listen, Action};
use crate::state::credentials::actions::credential_offers_selected::CredentialOffersSelected;
use crate::state::qr_code::actions::qrcode_scanned::QrCodeScanned;
use crate::state::{AppState, AppStateContainer};
use crate::user_error::{host_of_scanned_request, UserError};
use futures::StreamExt;
use itertools::Itertools;
use log::{debug, error, info};
use tauri::Emitter;

// TODO: remove this once we refactor our tests in `/unime/src-tauri/tests`
/// Define the runtime based on whether we are in test mode or not.
#[cfg(feature = "test_utils")]
pub type Runtime = tauri::test::MockRuntime;
#[cfg(not(feature = "test_utils"))]
pub type Runtime = tauri::Wry;

// The command.rs holds the functions through which the front and backend communicate using actions and reducers.

/// This function represents the root reducer of the application. It will delegate the state update to the reducers that
/// are listening to the action.
pub(crate) async fn reduce(state: AppState, action: Action) -> Result<AppState, AppError> {
    // Extract the reducers listening to this action.
    let reducers = action
        .reducers()
        .into_iter()
        .map(|reducer| (reducer, action.clone()))
        .collect_vec();

    // Apply the reducers to the state.
    #[allow(clippy::manual_try_fold)]
    futures::stream::iter(reducers)
        .fold(Ok(state), |app_state, (reducer, action)| async move {
            match app_state {
                Ok(app_state) => reducer(app_state, action).await,
                error => error,
            }
        })
        .await
}

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
pub async fn main_exec(
    action: Action,
    app_handle: tauri::AppHandle<Runtime>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<Runtime>,
) -> Result<(), String> {
    // Taken before waiting for the state: this is when the user's answer, if the action is one, was given.
    let start = std::time::Instant::now();
    let action_type = action.typetag_name();
    let scanned_host = listen::<QrCodeScanned>(action.clone())
        .and_then(|qr_code_scanned| host_of_scanned_request(&qr_code_scanned.form_urlencoded));
    let tx_code_entered = listen::<CredentialOffersSelected>(action.clone())
        .is_some_and(|credential_offers_selected| credential_offers_selected.tx_code.is_some());
    info!("Dispatching action: `{action_type}`");
    crate::log_payload!("Action payload: {action:?}");

    let mut guard = container.0.lock().await;

    // Slow work runs without the lock, so it does not hold up other actions.
    let action = match action.prepare(&guard) {
        Some(preparation) => {
            drop(guard);
            let action = preparation.await;
            guard = container.0.lock().await;
            action
        }
        None => action,
    };

    guard.core_utils.app_handle = Some(app_handle);

    let flow_started_before = guard.core_utils.flow_started_at;

    // Get a copy of the current state and pass it to the root reducer.
    let reduced = match answers_a_replaced_request(&action, start, &guard) {
        true => Err(AppError::FlowExpiredError(
            "Another request arrived before this answer could be handled".to_string(),
        )),
        false => reduce(guard.clone(), action).await,
    };
    match reduced {
        // If the state update succeeds, we replace the old state with the new one.
        Ok(app_state) => {
            debug!("State reduce succeeded in {:?}", start.elapsed());
            replace_once_saved(&mut guard, app_state, async |app_state| {
                save_state(app_state).await.ok();
            })
            .await;
        }
        // If the state update fails, we log the error and keep the old state.
        Err(error) => {
            {
                let debug_messages = &mut guard.debug_messages;
                while debug_messages.len() > 100 {
                    debug_messages.remove(0);
                }
                debug_messages.push_back(format!(
                    "{} {:?}",
                    chrono::Utc::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    error
                ));
                let counterparty = guard
                    .core_utils
                    .pending_connection_data
                    .as_ref()
                    .map(|data| data.client_metadata.client_name.clone())
                    .or(scanned_host);
                let mut user_error = UserError::new(&error, action_type, counterparty);
                if tx_code_entered {
                    user_error = user_error.after_tx_code_entered();
                }
                let _ = emit_error(&window, &user_error);
            }
            error!("State update failed after {:?}: {error}", start.elapsed());
            save_state(&guard).await.ok();
        }
    };

    // The state is saved. Nothing from here on waits, so the action's time limit cannot cut in between the state
    // being emitted and the request in it being counted as on screen.
    emit_event(&window, &guard).ok();

    // A request that took the place of another one is on screen from now on, not from the moment its reducer
    // finished: an answer that arrived in between was still given to the previous screen.
    if guard.core_utils.flow_started_at != flow_started_before {
        guard.core_utils.flow_started_at = Some(std::time::Instant::now());
    }

    Result::Ok(())
}

/// Makes `next` the current state once `save` has saved it.
///
/// Saving waits for the disk, and an action that runs into its time limit while it waits is cut short there (see
/// [`handle_action`]). The current state is therefore replaced only afterwards: a state that was not saved was not
/// emitted either, and a request in it would be the active one while the screen still shows the previous request, so
/// that the user's answer to what they see would answer a request they have not seen.
async fn replace_once_saved(current: &mut AppState, next: AppState, save: impl AsyncFnOnce(&AppState)) {
    save(&next).await;
    *current = next;
}

/// Whether `action` is the user's answer to a request that is no longer the one on screen.
///
/// Actions are handled one at a time. While a new request is read, which takes network requests, the screen still
/// shows the previous one, and an answer the user gives then waits its turn. By the time it is handled, the new
/// request is the active one: without this check, accepting the connection, the offer or the share that was on
/// screen would answer a request the user has not seen. An answer is therefore refused when the active request
/// started after the answer arrived.
fn answers_a_replaced_request(action: &Action, arrived_at: std::time::Instant, state: &AppState) -> bool {
    action.answers_request()
        && state
            .core_utils
            .flow_started_at
            .is_some_and(|flow_started_at| flow_started_at > arrived_at)
}

pub async fn handle_action(
    action: Action,
    app_handle: tauri::AppHandle<Runtime>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<Runtime>,
) -> Result<(), String> {
    let action_type = action.typetag_name();
    // Prevents deadlocks in the backend from hanging the frontend forever.
    let time_limit = action.time_limit();
    tokio::select! {
        res = main_exec(action, app_handle, container, window.clone()) => {
            debug!("Finished command invoke");
            res
        }
        _ = tokio::time::sleep(time_limit) => {
            error!("Operation timed out after {time_limit:?}");
            emit_error(&window, &UserError::timeout(action_type, None)).ok();
            Err("Operation timed out".to_string())
        }
    }
}

/// This function emits an event to the frontend with the updated state.
pub fn emit_event(window: &tauri::Window<Runtime>, app_state: &AppState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, app_state)?;

    // Print to console or write to file (default)
    if crate::logging::debug_flag("LOG_STATE_UPDATES_TO_CONSOLE") {
        debug!(
            "emitted event `{}` with payload:\n{}",
            STATE_CHANGED_EVENT,
            serde_json::to_string(app_state).unwrap()
        );
    } else if cfg!(debug_assertions) {
        if std::fs::create_dir_all("../../debug").is_err() {
            debug!("Failed to create debug directory");
        } else {
            std::fs::write(
                "../../debug/state.json",
                serde_json::to_string_pretty(app_state).unwrap(),
            )
            .unwrap_or_else(|_| debug!("Failed to write debug state file"));
        }
    };

    debug!("emitted event `{STATE_CHANGED_EVENT}`");
    Ok(())
}

/// This function emits an error to the frontend.
pub fn emit_error(window: &tauri::Window<Runtime>, error: &UserError) -> anyhow::Result<()> {
    const ERROR_EVENT: &str = "error";
    window.emit(ERROR_EVENT, error)?;

    debug!("emitted error event `{ERROR_EVENT}` with payload:\n{error:?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::common::actions::cancel_user_flow::CancelUserFlow;
    use crate::state::connections::actions::connection_accepted::ConnectionAccepted;
    use crate::state::core_utils::CoreUtils;
    use crate::state::credentials::actions::credentials_selected::CredentialsSelected;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    fn state_with_a_request_started_at(flow_started_at: Option<Instant>) -> AppState {
        AppState {
            core_utils: CoreUtils {
                flow_started_at,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn a_state_replaces_the_current_one_once_it_is_saved() {
        let mut current = state_with_a_request_started_at(None);
        let started_at = Some(Instant::now());

        replace_once_saved(&mut current, state_with_a_request_started_at(started_at), async |_| {}).await;

        assert_eq!(current.core_utils.flow_started_at, started_at);
    }

    #[tokio::test]
    async fn a_state_that_is_cut_short_while_it_is_saved_does_not_replace_the_current_one() {
        let mut current = state_with_a_request_started_at(None);
        let next = state_with_a_request_started_at(Some(Instant::now()));

        // The action's time limit ends the action while the state is being saved, as `handle_action` does.
        let cut_short = tokio::time::timeout(
            Duration::from_millis(10),
            replace_once_saved(&mut current, next, async |_| std::future::pending().await),
        )
        .await;

        assert!(cut_short.is_err());
        assert_eq!(current.core_utils.flow_started_at, None);
    }

    fn answers() -> Vec<Action> {
        vec![
            Arc::new(ConnectionAccepted),
            Arc::new(CredentialsSelected {
                selections: vec![],
                is_interactive: false,
            }),
            Arc::new(CredentialOffersSelected {
                credential_configuration_ids: vec![],
                tx_code: None,
            }),
        ]
    }

    #[test]
    fn an_answer_given_before_the_request_on_screen_was_replaced_is_refused() {
        let answer_arrived_at = Instant::now();
        let replaced_later = state_with_a_request_started_at(Some(answer_arrived_at + Duration::from_millis(1)));

        for answer in answers() {
            assert!(
                answers_a_replaced_request(&answer, answer_arrived_at, &replaced_later),
                "{answer:?}"
            );
        }
    }

    #[test]
    fn an_answer_to_the_request_on_screen_is_handled() {
        let request_started_at = Instant::now();
        let answer_arrived_at = request_started_at + Duration::from_millis(1);

        for state in [
            state_with_a_request_started_at(Some(request_started_at)),
            // No request has started, as in a state restored from disk.
            state_with_a_request_started_at(None),
        ] {
            for answer in answers() {
                assert!(
                    !answers_a_replaced_request(&answer, answer_arrived_at, &state),
                    "{answer:?}"
                );
            }
        }
    }

    #[test]
    fn only_an_answer_to_a_request_is_refused() {
        let arrived_at = Instant::now();
        let replaced_later = state_with_a_request_started_at(Some(arrived_at + Duration::from_millis(1)));
        let cancel: Action = Arc::new(CancelUserFlow { redirect: None });

        assert!(!answers_a_replaced_request(&cancel, arrived_at, &replaced_later));
    }
}
