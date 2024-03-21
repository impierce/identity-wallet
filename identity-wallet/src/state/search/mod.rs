pub mod actions;
pub mod reducers;

use super::FeatTrait;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, Derivative, TS, Clone, PartialEq, Debug)]
#[ts(export)]
#[serde(default)]
pub struct SearchResults {
    pub current: Vec<String>,
    pub recents_credentials: Vec<String>,
    pub recents_connections: Vec<String>,
}

#[typetag::serde(name = "SearchResults")]
impl FeatTrait for SearchResults {}
