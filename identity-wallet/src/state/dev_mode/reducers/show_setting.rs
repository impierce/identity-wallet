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

pub async fn show_setting(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(show) = listen::<ShowDevModeSetting>(action).map(|payload| payload.show) {
        debug!("Show dev mode setting: {}", show);

        if show {
            state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
                target: "me/settings/app".to_string(),
            })
        } else {
            state.dev_mode = DevMode::Off;
            state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            });
        }

        return Ok(AppState {
            show_dev_mode_setting: show,
            ..state
        });
    }

    Ok(state)
}
