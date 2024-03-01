use super::dragon_dynamic_profile::*;
use crate::{
    command,
    error::AppError,
    state::{
        actions::{listen, Action},
        common::actions::UnlockStorage,
        dev_mode::{actions::DevProfile, ferris_static_profile::load_ferris_profile, DevMode, ProfileType},
        AppState,
    },
};

use log::info;
use std::sync::Arc;

pub(super) const PASSWORD: &str = "sup3rSecr3t";

pub async fn load_dev_profile(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("Load dev profile: {:?}", action);

    if let Some(dev_profile) = listen::<DevProfile>(action) {
        // All dev profiles need to use the const PASSWORD so it can automatically unlock storage.
        match dev_profile.profile {
            ProfileType::Ferris => return load_ferris_profile().await,
            ProfileType::Dragon => return load_dragon_profile(state, dev_profile).await,
        }
    }

    Ok(state)
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

pub async fn unlock_storage(state: AppState) -> Result<AppState, AppError> {
    command::reduce(
        state,
        Arc::new(UnlockStorage {
            password: PASSWORD.to_string(),
        }),
    )
    .await
}
