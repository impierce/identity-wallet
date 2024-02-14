use ts_rs::TS;
use crate::{reducer, state::{actions::{ActionTrait, Reducer}, 
    dev_mode::reducers::{load_dev_profile, set_dev_mode}}};

/// Actions

/// Action to set the dev mode to the given value.

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SetDevMode.ts")]
pub struct SetDevMode {
    pub enabled: bool,
}

#[typetag::serde(name = "[DEV] Set dev mode")]
impl ActionTrait for SetDevMode {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(set_dev_mode)]
    }
}

/// Action to load the dev profile.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/LoadDevProfile.ts")]
pub struct LoadDevProfile;

#[typetag::serde(name = "[DEV] Load profile")]
impl ActionTrait for LoadDevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile)]
    }
}
