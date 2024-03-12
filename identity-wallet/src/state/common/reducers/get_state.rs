use crate::error::AppError::{self};
use crate::persistence::load_state;
use crate::state::actions::Action;
use crate::state::dev_mode::DevMode;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

use log::debug;

pub async fn get_state(_state: AppState, _action: Action) -> Result<AppState, AppError> {
    debug!("get_state reducer called");
    let mut state = load_state().await.unwrap_or_default();

    if state.profile_settings.profile.is_some() {
        state.current_user_prompt = Some(CurrentUserPrompt::PasswordRequired);
    } else {
        // TODO: bug: if state is present, but empty, user will never be redirected to neither welcome or profile page
        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        });
    }

    // Overwrite dev_mode_enabled if environment variable is set
    if let Some(dev_mode) = std::env::var("DEV_MODE_ENABLED")
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
    {
        if dev_mode {
            if state.dev_mode == DevMode::Off {
                state.dev_mode = DevMode::On;
            }
        } else {
            state.dev_mode = DevMode::Off;
        }
    }

    Ok(state)
}
