use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        profile_settings::{actions::enable_biometrics::EnableBiometrics, ProfileSettings},
        AppState,
    },
};
use log::debug;

#[tracing::instrument(skip_all, err)]
pub async fn enable_biometrics(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(enable) = listen::<EnableBiometrics>(action).map(|payload| payload.enable) {
        debug!("biometrics enabled: `{enable:?}`");
        return Ok(AppState {
            profile_settings: ProfileSettings {
                biometrics_enabled: enable,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
