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

pub async fn unlock_storage(state: AppState) -> Result<AppState, AppError> {
    command::reduce(
        state,
        Arc::new(UnlockStorage {
            password: PASSWORD.to_string(),
        }),
    )
    .await
}
