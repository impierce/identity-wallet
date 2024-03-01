use super::{ProfileSteps, ProfileType};
use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        dev_mode::reducers::{load_dev_profile, toggle_dev_mode},
        Reducer,
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to toggle the DEV mode.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ToggleDevMode;

#[typetag::serde(name = "[DEV] Toggle DEV mode")]
impl ActionTrait for ToggleDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(toggle_dev_mode)]
    }
}

/// Action to load a dev profile.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
    pub execute_step: Option<ProfileSteps>,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
