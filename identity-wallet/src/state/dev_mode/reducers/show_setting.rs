use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        dev_mode::{actions::show_setting::ShowDevModeSetting, DevMode},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};

use log::debug;

pub async fn show_setting(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(show) = listen::<ShowDevModeSetting>(action).map(|payload| payload.show) {
        debug!("Show dev mode setting: {show}");

        if show {
            return Ok(AppState {
                show_dev_mode_setting: true,
                current_user_prompt: Some(CurrentUserPrompt::Redirect {
                    target: "me/settings/app".to_string(),
                }),
                ..state
            });
        } else {
            return Ok(AppState {
                dev_mode: DevMode::Off,
                show_dev_mode_setting: false,
                current_user_prompt: Some(CurrentUserPrompt::Redirect {
                    target: "me".to_string(),
                }),
                ..state
            });
        }
    }

    Ok(state)
}
