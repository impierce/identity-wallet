use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

pub async fn dismiss_interaction_result(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        interaction_result: None,
        ..state
    })
}
