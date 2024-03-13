use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        dev_mode::{
            actions::dev_profile::{DevProfile, ProfileType},
            reducers::{dragon_dynamic_profile::load_dragon_profile, ferris_static_profile::load_ferris_profile},
        },
        AppState,
    },
};

use log::info;

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
