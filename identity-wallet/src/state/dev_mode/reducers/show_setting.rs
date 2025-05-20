use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        dev_mode::actions::show_setting::ShowDevModeSetting,
        AppState,
    },
};

use log::debug;

pub async fn show_setting(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(show) = listen::<ShowDevModeSetting>(action).map(|payload| payload.show) {
        debug!("Show dev mode setting: {}", show);
        return Ok(AppState {
            show_dev_mode_setting: show,
            ..state
        });
    }

    Ok(state)
}
