use identity_wallet::state::FeatTrait;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod actions;
pub mod reducers;

// This module is soly for testing and demonstrating the extension system.
#[derive(Debug, Serialize, Deserialize, TS, PartialEq, Default, Clone)]
pub struct CustomExtension {
    pub name: String,
    pub value: String,
}

#[typetag::serde(name = "custom_extension")]
impl FeatTrait for CustomExtension {}
