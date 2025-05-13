use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::common::actions::check_password::CheckPassword;
use crate::state::AppState;
use crate::stronghold::StrongholdManager;

pub async fn check_password(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<CheckPassword>(action).map(|payload| payload.password) {
        // TODO(refactor): In the current design of UniMe, there is no way to tell the frontend that the password is correct, except through a state update.
        //   We therefore push a debug message and return the state as is.
        //   TODO: possible solution: introduce unique "action id" to identify which command triggered with action (similar to tracing id)
        if StrongholdManager::load(&password).is_ok() {
            return Ok(AppState {
                debug_messages: {
                    let mut debug_messages = state.debug_messages.clone();
                    debug_messages.push_back("Stronghold password OK".to_string());
                    debug_messages
                },
                current_user_prompt: None,
                ..state
            });
        } else {
            return Ok(AppState {
                debug_messages: {
                    let mut debug_messages = state.debug_messages.clone();
                    debug_messages.push_back("Wrong Stronghold password".to_string());
                    debug_messages
                },
                current_user_prompt: None,
                ..state
            });
        }
    }

    Ok(state)
}
