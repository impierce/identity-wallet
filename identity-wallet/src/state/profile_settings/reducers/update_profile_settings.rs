use crate::{
    error::AppError::{self},
    state::{
        actions::{listen, Action},
        profile_settings::{actions::update_profile_settings::UpdateProfileSettings, Profile, ProfileSettings},
        AppState,
    },
};

#[tracing::instrument(skip_all, err)]
pub async fn update_profile_settings(state: AppState, action: Action) -> Result<AppState, AppError> {
    log::debug!("Updating profile settings");
    if let Some(UpdateProfileSettings { theme, name, picture }) = listen::<UpdateProfileSettings>(action) {
        if let Some(profile) = state.profile_settings.profile.clone() {
            return Ok(AppState {
                profile_settings: ProfileSettings {
                    profile: Some(Profile {
                        name: name.unwrap_or(profile.name),
                        picture: picture.or(profile.picture),
                        theme: theme.unwrap_or(profile.theme),
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
