pub mod actions;
pub mod dragon_dynamic_profile;
pub mod ferris_static_profile;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// DevMode is a simple enum to enable dev mode for developers to test the app.
#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/DevMode.ts")]
pub enum DevMode {
    On,
    #[default]
    Off,
    OnWithAutologin,
}

#[typetag::serde(name = "dev_mode")]
impl FeatTrait for DevMode {}

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
