use crate::error::AppError;
use crate::state::actions::{listen, Action};
use crate::state::connections::actions::load_issued_credentials::LoadIssuedCredentials;
use crate::state::connections::issued_credentials::mark_owned;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

#[tracing::instrument(skip_all, err)]
pub async fn load_issued_credentials(state: AppState, action: Action) -> Result<AppState, AppError> {
    let Some(load_issued_credentials) = listen::<LoadIssuedCredentials>(action) else {
        return Ok(state);
    };

    // Loaded in `prepare` when dispatched from the frontend, or here when reduced directly.
    let mut page = match load_issued_credentials.loaded {
        Some(page) => page,
        None => load_issued_credentials.load(&state).await,
    };

    page.request_id = load_issued_credentials.request_id;

    // Against the current state: credentials may have been added while the page loaded.
    mark_owned(&mut page, &state.credentials);

    // A redirect is followed as soon as it is emitted, so one still in the state (such as the one to `me` after
    // unlocking) would be followed again on this update. Other prompts may have arrived while the page loaded, and stay.
    let current_user_prompt = match state.current_user_prompt {
        Some(CurrentUserPrompt::Redirect { .. }) => None,
        prompt => prompt,
    };

    Ok(AppState {
        issued_credentials: Some(page),
        current_user_prompt,
        ..state
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::connections::{Connection, Connections};
    use std::sync::Arc;

    fn load(connection_id: &str) -> Action {
        Arc::new(LoadIssuedCredentials {
            request_id: 7,
            connection_id: connection_id.to_string(),
            search_term: "badge".to_string(),
            offset: 0,
            limit: 20,
            loaded: None,
        })
    }

    #[tokio::test]
    async fn stores_the_page_of_an_unknown_connection_as_unavailable() {
        let state = AppState {
            connections: Connections(vec![Connection {
                id: "known".to_string(),
                ..Default::default()
            }]),
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..Default::default()
        };

        let state = load_issued_credentials(state, load("unknown")).await.unwrap();

        assert!(state.current_user_prompt.is_none());
        let page = state.issued_credentials.unwrap();
        assert_eq!(page.request_id, 7);
        assert_eq!(page.connection_id, "unknown");
        assert_eq!(page.search_term, "badge");
        assert!(page.unavailable);
    }

    #[tokio::test]
    async fn keeps_a_prompt_that_is_not_a_redirect() {
        let state = AppState {
            current_user_prompt: Some(CurrentUserPrompt::PasswordRequired),
            ..Default::default()
        };

        let state = load_issued_credentials(state, load("unknown")).await.unwrap();

        assert_eq!(state.current_user_prompt, Some(CurrentUserPrompt::PasswordRequired));
    }
}
