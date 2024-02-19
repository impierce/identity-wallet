pub mod actions;
pub mod reducers;

/// This folder contains actions and reducers functioning the boot level of the application.
/// There is no corresponding field in the appstate for this module, this folder rather pertains to the entire appstate.

#[cfg(test)]
mod tests {
    use crate::state::{
        common::{actions::{CancelUserFlow, Reset}, reducers::{cancel_user_flow, reset_state}}, profile_settings::{Locale, Profile, ProfileSettings}, user_prompt::CurrentUserPrompt, AppState
    };
    use std::sync::Arc;

    #[tokio::test]
    async fn test_reset_state() {
        let mut app_state = AppState {
            profile_settings: ProfileSettings { 
                profile: Some(Profile {
                    name: "Ferris".to_string(),
                    picture: Some("&#129408".to_string()),
                    theme: Some("system".to_string()),
                    primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
                }),
                ..Default::default()
            },
            ..Default::default()
        };

        app_state = reset_state(app_state, Arc::new(Reset)).await.unwrap();

        assert_eq!(app_state.extensions.is_empty(), true);
        assert_eq!(app_state.profile_settings.locale, Locale::default());
    }

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
