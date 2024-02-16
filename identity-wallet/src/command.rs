use crate::state::{actions::Action, AppState, AppStateContainer};
use crate::error::AppError;
use crate::persistence::save_state;
use log::{debug, error, info, warn};
use std::time::Duration;
use futures::StreamExt;
use itertools::Itertools;
use tauri::Manager;

/// This function represents the root reducer of the application. It will delegate the state update to the reducers that
/// are listening to the action.
async fn reduce(state: AppState, action: Action) -> Result<AppState, AppError> {
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

async fn deadlock_safety() {
    tokio::time::sleep(Duration::from_secs(6)).await;
}

async fn main_exec<R: tauri::Runtime>(
    action: Action,
    _app_handle: tauri::AppHandle<R>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    let mut guard = container.0.lock().await;

    println!("Action: {action:?}\n\n");

    // Get a copy of the current state and pass it to the root reducer.
    match reduce(
        AppState {
            connections: guard.connections.clone(),
            credentials: guard.credentials.clone(),
            user_data_query: guard.user_data_query.clone(),
            back_end_utils: guard.back_end_utils.clone(),
            profile_settings: guard.profile_settings.clone(),
            current_user_prompt: guard.current_user_prompt.clone(),
            debug_messages: guard.debug_messages.clone(),
            user_journey: guard.user_journey.clone(),
            extensions: guard.extensions.clone(),
            dev_mode_enabled: guard.dev_mode_enabled,
        },
        action,
    )
    .await
    {
        // If the state update succeeds, we replace the old state with the new one.
        Ok(app_state) => {
            println!("APP STATE: {app_state:?}\n\n");
            *guard = app_state
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
            }
            warn!("state update failed: {}", error);
        }
    };

    // Save and emit the state.
    save_state(&guard).await.ok();
    emit_event(&window, &guard).ok();

    Result::Ok(())
}

/// This command handler is the single point of entry to the business logic in the backend. It will delegate the
/// command it receives to the designated functions that modify the state (see: "reducers" in the Redux pattern).
pub async fn handle_action<R: tauri::Runtime>(
    action: Action,
    app_handle: tauri::AppHandle<R>,
    container: tauri::State<'_, AppStateContainer>,
    window: tauri::Window<R>,
) -> Result<(), String> {
    tokio::select! {
        res = main_exec(action, app_handle, container, window) => {
            info!("Finish invoke");
            res
        }
        _ = deadlock_safety() => {
            error!("Operation timed out");
            Err("timed out".to_string())
        }
    }
}

pub fn emit_event<R: tauri::Runtime>(window: &tauri::Window<R>, app_state: &AppState) -> anyhow::Result<()> {
    const STATE_CHANGED_EVENT: &str = "state-changed";
    window.emit(STATE_CHANGED_EVENT, app_state)?;
    debug!("emitted event `{}` with payload `{:?}`", STATE_CHANGED_EVENT, app_state);
    Ok(())
}
