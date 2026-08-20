use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

#[tracing::instrument(skip_all, err)]
pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    log::debug!("Cancelling user journey");
    Ok(AppState {
        user_journey: None,
        current_user_prompt: None,
        ..state
    })
}
