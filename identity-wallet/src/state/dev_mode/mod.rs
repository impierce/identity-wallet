use super::FeatTrait;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

mod dragon_dynamic_profile;
mod ferris_static_profile;
pub mod load_dev_profile;
pub mod toggle_dev_mode;

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
