use crate::{
    error::AppError,
    state::{
        actions::{listen, Action},
        dev_mode::{
            actions::dev_profile::{DevProfile, ProfileType},
            reducers::ferris_static_profile::load_ferris_profile,
        },
        AppState,
    },
};

#[tracing::instrument(skip_all, err)]
pub async fn load_dev_profile(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(dev_profile) = listen::<DevProfile>(action) {
        log::info!("Loading {:?} dev profile", dev_profile.profile);
        // All dev profiles need to use the same fixed password, so the frontend can automatically unlock storage.
        match dev_profile.profile {
            ProfileType::Ferris => return load_ferris_profile().await,
        }
    }

    Ok(state)
}
