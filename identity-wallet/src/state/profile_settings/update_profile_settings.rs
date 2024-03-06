use crate::{
    reducer,
    state::{
        actions::ActionTrait, Reducer,
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::{
    error::AppError::{self},
    state::{
        actions::{listen, Action},
        profile_settings::{Profile, ProfileSettings},
        AppState,
    },
};

/// Action to update the profile settings.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateProfileSettings.ts")]
pub struct UpdateProfileSettings {
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub picture: Option<String>,
    #[ts(optional)]
    pub theme: Option<String>,
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

