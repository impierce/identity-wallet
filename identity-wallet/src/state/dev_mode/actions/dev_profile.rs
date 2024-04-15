use crate::{
    reducer,
    state::{actions::ActionTrait, dev_mode::reducers::load_dev_profile::load_dev_profile, profile_settings::reducers::update_sorting_preference::{sort_connections, sort_credentials}, Reducer},
};

use ts_rs::TS;

/// Action to load a dev profile.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevProfile.ts")]
pub struct DevProfile {
    pub profile: ProfileType,
    pub execute_step: Option<ProfileSteps>,
    pub reset_profile: bool,
}

#[typetag::serde(name = "[DEV] Load DEV profile")]
impl ActionTrait for DevProfile {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(load_dev_profile), reducer!(sort_credentials), reducer!(sort_connections)]
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/ProfileType.ts")]
pub enum ProfileType {
    /// Dev profile with predefined static data
    Ferris,

    /// Dev profile that can dynamically interact with the NGDIL demo by executing `ProfileSteps`
    Dragon,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone, PartialEq, Eq, PartialOrd)]
#[ts(export, export_to = "bindings/dev/ProfileSteps.ts")]
pub enum ProfileSteps {
    /// Step 1
    #[serde(rename = "Create profile")]
    CreateProfile,
    /// Step 2
    #[serde(rename = "Add credentials")]
    AddCredentials,
    /// Step 3
    #[serde(rename = "Accept credentials")]
    AcceptCredentials,
    /// Step 4
    #[serde(rename = "Add connection")]
    AddConnection,
    /// Step 5
    #[serde(rename = "Accept connection")]
    AcceptConnection,
    /// Step 6
    #[serde(rename = "Add presentation")]
    AddPresentation,
    /// Step 7
    #[serde(rename = "Share credentials")]
    ShareCredentails,
    /// Step 8
    #[serde(rename = "Add future engineer")]
    AddFutureEngineer,
    /// Step 9
    #[serde(rename = "Complete flow")]
    CompleteFlow,
}
