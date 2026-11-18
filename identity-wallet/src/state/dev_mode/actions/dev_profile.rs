use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        dev_mode::reducers::load_dev_profile::load_dev_profile,
        profile_settings::reducers::update_sorting_preference::{sort_connections, sort_credentials},
        Reducer,
    },
};

use ts_rs::TS;

/// Action to load a dev profile.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(load_dev_profile),
            reducer!(sort_credentials),
            reducer!(sort_connections),
        ]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/ProfileType.ts")]
pub enum ProfileType {
    /// Dev profile with predefined static data
    Ferris,
}
