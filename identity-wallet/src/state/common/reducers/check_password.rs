use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::common::actions::check_password::CheckPassword;
use crate::state::core_utils::debug_log::{push_debug_message, DebugLevel};
use crate::state::AppState;
use crate::stronghold::StrongholdManager;

/// The security settings page reads the outcome from the end of the last debug message.
const PASSWORD_OK_MESSAGE: &str = "Stronghold password OK";
const WRONG_PASSWORD_MESSAGE: &str = "Wrong Stronghold password";

#[tracing::instrument(skip_all, err)]
pub async fn check_password(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<CheckPassword>(action).map(|payload| payload.password) {
        // TODO(refactor): In the current design of UniMe, there is no way to tell the frontend that the password is correct, except through a state update.
        //   We therefore push a debug message and return the state as is.
        //   TODO: possible solution: introduce unique "action id" to identify which command triggered with action (similar to tracing id)
        if StrongholdManager::load(&password).is_ok() {
            log::debug!("Stronghold password verification succeeded");
            return Ok(AppState {
                debug_messages: {
                    let mut debug_messages = state.debug_messages.clone();
                    push_debug_message(&mut debug_messages, DebugLevel::Ok, PASSWORD_OK_MESSAGE);
                    debug_messages
                },
                current_user_prompt: None,
                ..state
            });
        } else {
            log::warn!("Stronghold password verification failed");
            return Ok(AppState {
                debug_messages: {
                    let mut debug_messages = state.debug_messages.clone();
                    push_debug_message(&mut debug_messages, DebugLevel::Error, WRONG_PASSWORD_MESSAGE);
                    debug_messages
                },
                current_user_prompt: None,
                ..state
            });
        }
    }

    Ok(state)
}
