use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        user_journey: None,
        current_user_prompt: None,
        ..state
    })
}
