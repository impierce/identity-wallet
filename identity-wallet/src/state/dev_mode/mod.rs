pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// DevMode is a simple enum to enable dev mode for developers to test the app.
#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq, Default)]
#[ts(export, export_to = "bindings/dev_mode/DevMode.ts")]
pub enum DevMode {
    On,
    #[default]
    Off,
    OnWithAutologin, // shenron: Used whenever a Dev profile is initiated, since these are initiated without having to onboard and set a password. I'll have to keep this for the Ferris profile.
}

#[typetag::serde(name = "dev_mode")]
impl FeatTrait for DevMode {}
