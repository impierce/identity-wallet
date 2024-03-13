use crate::{
    error::AppError::{self},
    state::{
        actions::{listen, Action},
        profile_settings::{actions::update_profile_settings::UpdateProfileSettings, Profile, ProfileSettings},
        AppState,
    },
};

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
