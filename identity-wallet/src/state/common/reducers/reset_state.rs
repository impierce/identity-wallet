use crate::error::AppError::{self};
use crate::persistence::{clear_all_assets, delete_state_file, delete_stronghold};
use crate::state::actions::Action;
use crate::state::dev_mode::DevMode;
use crate::state::user_prompt::CurrentUserPrompt;
use crate::state::AppState;

/// Completely resets the state to its default values.
pub async fn reset_state(state: AppState, _action: Action) -> Result<AppState, AppError> {
    delete_state_file().await.ok();
    delete_stronghold().await.ok();
    clear_all_assets().ok();

    // Preserve dev_mode state, but drop "auto login".
    // This ensures that for a fresh profile, the default test password is not automtically injected.
    let dev_mode = match state.dev_mode {
        DevMode::On | DevMode::OnWithAutologin => DevMode::On,
        DevMode::Off => DevMode::Off,
    };

    Ok(AppState {
        current_user_prompt: Some(CurrentUserPrompt::Redirect {
            target: "welcome".to_string(),
        }),
        dev_mode,
        ..Default::default()
    })
}
