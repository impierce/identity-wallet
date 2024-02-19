pub mod actions;
pub mod reducers;

#[cfg(test)]
mod tests {
    use crate::state::{user_journey::{actions::CancelUserJourney, reducers::cancel_user_journey}, AppState};
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
