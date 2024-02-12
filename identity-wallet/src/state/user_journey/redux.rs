use crate::{error::AppError, reducer, state::{actions::{Action, ActionTrait, Reducer}, AppState}};
use ts_rs::TS;

/// Actions

/// Action to cancel the user journey.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserJourney.ts")]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}

/// Reducers

/// Reducer to cancel the user journey.
pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        user_journey: None,
        ..state
    })
}
