use crate::error::AppError;
use crate::persistence::save_state;
use crate::state::actions::Action;
use crate::state::{AppState, AppStateContainer};
use futures::StreamExt;
use itertools::Itertools;
use log::{debug, error, info};
use std::time::Duration;
use tauri::Emitter;

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

// This value is based on an estimated guess. Can be adjusted in case lower/higher timeouts are desired.
const TIMEOUT_SECS: u64 = 10;

/// This function is used to prevent deadlocks in the backend. It will sleep for a certain amount of time and then return.
async fn await_timeout() {
    tokio::time::sleep(Duration::from_secs(TIMEOUT_SECS)).await;
}

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
pub async fn main_exec<R: tauri::Runtime>(
    action: Action,
    _app_handle: tauri::AppHandle<R>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    info!("received action: `{action:?}`");

    let mut guard = container.0.lock().await;

    // Get a copy of the current state and pass it to the root reducer.
    match reduce(guard.clone(), action).await {
        // If the state update succeeds, we replace the old state with the new one.
        Ok(app_state) => *guard = app_state,
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
                let _ = emit_error(&window, error.to_string());
            }
            error!("state update failed: {}", error);
        }
    };

    // Save and emit the state.
    save_state(&guard).await.ok();
    emit_event(&window, &guard).ok();

    Result::Ok(())
}

pub async fn handle_action<R: tauri::Runtime>(
    action: Action,
    app_handle: tauri::AppHandle<R>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    tokio::select! {
        res = main_exec(action, app_handle, container, window.clone()) => {
            debug!("Finish invoke");
            res
        }
        _ = await_timeout() => {
            error!("Operation timed out");
            emit_error(&window, "Operation timed out".to_string()).ok();
            Err("Operation timed out".to_string())
        }
    }
}

/// This function emits an event to the frontend with the updated state.
pub fn emit_event<R: tauri::Runtime>(window: &tauri::Window<R>, app_state: &AppState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, app_state)?;

    let app_state_json_str = serde_json::to_string(app_state).unwrap();

    debug!(
        "emitted event `{}` with payload:\n{}",
        STATE_CHANGED_EVENT, app_state_json_str
    );
    Ok(())
}

/// This function emits an error to the frontend.
pub fn emit_error<R: tauri::Runtime>(window: &tauri::Window<R>, error: String) -> anyhow::Result<()> {
    const ERROR_EVENT: &str = "error";
    window.emit(ERROR_EVENT, &error)?;

    debug!("emitted error event `{}` with payload:\n{}", ERROR_EVENT, error);
    Ok(())
}
