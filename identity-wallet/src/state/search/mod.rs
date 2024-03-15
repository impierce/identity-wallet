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
    pub recents: Vec<String>,
}

#[typetag::serde(name = "UserDataQueryResults")]
impl FeatTrait for SearchResults {}
