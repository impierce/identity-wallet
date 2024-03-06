use crate::error::AppError::{self};
use crate::state::actions::{listen, Action};
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;
use crate::{
    reducer,
    state::{actions::ActionTrait, Reducer},
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to cancel the user flow and redirect to a specific route.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserFlow.ts")]
pub struct CancelUserFlow {
    #[ts(optional)]
    pub redirect: Option<String>,
}

#[typetag::serde(name = "[User Flow] Cancel")]
impl ActionTrait for CancelUserFlow {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_flow)]
    }
}

pub async fn cancel_user_flow(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(cancel_user_flow) = listen::<CancelUserFlow>(action) {
        return Ok(AppState {
            current_user_prompt: cancel_user_flow
                .redirect
                .map(|target| CurrentUserPrompt::Redirect { target }),
            ..state
        });
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cancel_user_flow() {
        let current_user_prompt = Some(CurrentUserPrompt::ShareCredentials {
            client_name: "Impierce Technologies".to_string(),
            logo_uri: Some("logo.png".to_string()),
            options: vec![],
        });

        let mut app_state = AppState {
            current_user_prompt: current_user_prompt.clone(),
            ..AppState::default()
        };

        app_state = cancel_user_flow(app_state, Arc::new(CancelUserFlow { redirect: None }))
            .await
            .unwrap();

        assert_eq!(app_state.current_user_prompt, None);

        let mut app_state = AppState {
            current_user_prompt,
            ..AppState::default()
        };

        app_state = cancel_user_flow(
            app_state,
            Arc::new(CancelUserFlow {
                redirect: Some("welcome".to_string()),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.current_user_prompt,
            Some(CurrentUserPrompt::Redirect {
                target: "welcome".to_string(),
            })
        );
    }
}
