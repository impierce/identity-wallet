use crate::{
    error::AppError::{self},
    state::{
        actions::{listen, Action},
        profile_settings::{actions::set_locale::SetLocale, ProfileSettings},
        AppState,
    },
};

use log::debug;

/// Sets the locale to the given value. If the locale is not supported yet, the current locale will stay unchanged.
pub async fn set_locale(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(locale) = listen::<SetLocale>(action).map(|payload| payload.locale) {
        debug!("locale set to: `{:?}`", locale);
        return Ok(AppState {
            profile_settings: ProfileSettings {
                locale,
                ..state.profile_settings
            },
            current_user_prompt: None,
            ..state
        });
    }
    Ok(state)
}
