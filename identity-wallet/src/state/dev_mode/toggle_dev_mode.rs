use crate::{
    error::AppError,
    reducer,
    state::{actions::Action, dev_mode::DevMode, AppState},
    state::{actions::ActionTrait, Reducer},
};
use log::info;
use serde::{Deserialize, Serialize};

/// Action to toggle the DEV mode.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleDevMode;

#[typetag::serde(name = "[DEV] Toggle DEV mode")]
impl ActionTrait for ToggleDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_dev_mode)]
    }
}

pub async fn toggle_dev_mode(mut state: AppState, _action: Action) -> Result<AppState, AppError> {
    info!("Toggle dev mode");

    if state.dev_mode != DevMode::Off {
        state.dev_mode = DevMode::Off;
    } else {
        // We don't preserve if user had autologin enabled
        // So we just put it back to default (reload profile if you want to enable autologin again)
        state.dev_mode = DevMode::On;
    }

    state.current_user_prompt = None;

    Ok(state)
}
