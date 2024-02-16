pub mod actions;
pub mod reducers;

/// This folder contains actions and reducers functioning the boot level of the application.
/// There is no corresponding field in the appstate for this module, this folder rather pertains to the entire appstate.

#[cfg(test)]
mod tests {
    use crate::state::{
        common::{actions::Reset, reducers::reset_state},
        profile_settings::{ProfileSettings, Locale, Profile},
        AppState
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
}
