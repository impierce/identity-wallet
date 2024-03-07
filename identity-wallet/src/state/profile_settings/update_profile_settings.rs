use crate::{
    error::AppError,
    reducer,
    state::{
        actions::{listen, Action, ActionTrait},
        profile_settings::{Profile, ProfileSettings},
        AppState, AppTheme, Reducer,
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to update the profile settings.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<AppTheme>,
}

#[typetag::serde(name = "[Settings] Update profile")]
impl ActionTrait for UpdateProfileSettings {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_profile_settings)]
    }
}

pub async fn update_profile_settings(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(UpdateProfileSettings { theme, name, picture }) = listen::<UpdateProfileSettings>(action) {
        if let Some(profile) = state.profile_settings.profile.clone() {
            return Ok(AppState {
                profile_settings: ProfileSettings {
                    profile: Some(Profile {
                        name: name.unwrap_or(profile.name),
                        picture: picture.or(profile.picture),
                        theme: theme.or(profile.theme),
                        ..profile
                    }),
                    ..state.profile_settings
                },
                current_user_prompt: None,
                ..state
            });
        }
    }

    Ok(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::state::AppTheme;

    #[tokio::test]
    async fn test_update_profile_settings() {
        let active_profile = Profile {
            name: "Ferris".to_string(),
            picture: Some("&#129408".to_string()),
            theme: Some(AppTheme::System),
            primary_did: "did:mock:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK".to_string(),
        };

        let mut app_state = AppState {
            profile_settings: ProfileSettings {
                profile: Some(active_profile.clone()),
                ..ProfileSettings::default()
            },
            ..AppState::default()
        };

        app_state = update_profile_settings(
            app_state,
            Arc::new(UpdateProfileSettings {
                name: None,
                picture: None,
                theme: Some(AppTheme::Light),
            }),
        )
        .await
        .unwrap();

        assert_eq!(
            app_state.profile_settings.profile,
            Some(Profile {
                theme: Some(AppTheme::Light),
                ..active_profile
            })
        );
    }
}
