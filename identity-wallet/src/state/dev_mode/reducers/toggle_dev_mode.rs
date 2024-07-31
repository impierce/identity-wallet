use crate::{
    error::AppError,
    state::{actions::Action, dev_mode::DevMode, AppState},
};

use log::info;

pub async fn toggle_dev_mode(state: AppState, _action: Action) -> Result<AppState, AppError> {
    info!("Toggle dev mode");

    let mut dev_mode = state.dev_mode;

    if dev_mode != DevMode::Off {
        dev_mode = DevMode::Off;
    } else {
        // We don't preserve if user had autologin enabled
        // So we just put it back to default (reload profile if you want to enable autologin again)
        dev_mode = DevMode::On;
    }

    Ok(AppState {
        dev_mode,
        current_user_prompt: None,
        ..state
    })
}
