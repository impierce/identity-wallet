use std::sync::Arc;
use super::{ferris_static_profile::load_ferris_profile, dragon_dynamic_profile::load_dragon_profile};
use log::info;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::{command, error::AppError, reducer, state::{actions::{listen, Action, ActionTrait}, common::unlock_storage::UnlockStorage, AppState, Reducer}};

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/ProfileType.ts")]
pub enum ProfileType {
    /// Dev profile with predefined static data
    Ferris,

    /// Dev profile that can dynamically interact with the NGDIL demo by executing `ProfileSteps`
    Dragon,
}

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq, PartialOrd)]
#[ts(export, export_to = "bindings/dev/ProfileSteps.ts")]
pub enum ProfileSteps {
    /// Step 1
    CreateProfile,
    /// Step 2
    AddCredentials,
    /// Step 3
    AcceptCredentials,
    /// Step 4
    AddConnection,
    /// Step 5
    AcceptConnection,
    /// Step 6
    AddPresentation,
    /// Step 7
    ShareCredentails,
    /// Step 8
    AddFutureEngineer,
    /// Step 9
    CompleteFlow,
}

/// Action to load a dev profile.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
    pub execute_step: Option<ProfileSteps>,
}

pub(super) const PASSWORD: &str = "sup3rSecr3t";

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}

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

pub async fn unlock_storage(state: AppState) -> Result<AppState, AppError> {
    command::reduce(
        state,
        Arc::new(UnlockStorage {
            password: PASSWORD.to_string(),
        }),
    )
    .await
}
