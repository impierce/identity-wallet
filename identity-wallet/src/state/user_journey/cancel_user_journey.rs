use crate::reducer;
use crate::state::{actions::ActionTrait, Reducer};
use crate::{
    error::AppError,
    state::{actions::Action, AppState},
};

/// Action to cancel the user journey.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CancelUserJourney;

#[typetag::serde(name = "[User Journey] Cancel")]
impl ActionTrait for CancelUserJourney {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_journey)]
    }
}

pub async fn cancel_user_journey(state: AppState, _action: Action) -> Result<AppState, AppError> {
    Ok(AppState {
        user_journey: None,
        current_user_prompt: None,
        ..state
    })
}

#[cfg(test)]
mod tests {
    use super::cancel_user_journey;
    use super::CancelUserJourney;
    use crate::state::AppState;
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cancel_user_journey() {
        let mut app_state = AppState {
            user_journey: Some(json!("Some Journey")),
            ..AppState::default()
        };

        app_state = cancel_user_journey(app_state, Arc::new(CancelUserJourney))
            .await
            .unwrap();

        assert_eq!(app_state.user_journey, None);
    }
}
