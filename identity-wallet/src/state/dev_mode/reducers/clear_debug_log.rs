use std::collections::VecDeque;

use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

#[tracing::instrument(skip_all, err)]
pub async fn clear_debug_log(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        debug_messages: VecDeque::new(),
        current_user_prompt: None,
        ..state
    })
}
