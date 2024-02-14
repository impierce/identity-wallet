use crate::{error::AppError, state::{AppState, actions::Action}};

/// Reducers

/// Reducer to cancel the user journey.
pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        user_journey: None,
        ..state
    })
}
