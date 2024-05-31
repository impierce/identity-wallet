pub mod actions;
pub mod reducers;

use super::FeatTrait;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Serialize, Deserialize, TS, Clone, PartialEq, Debug)]
#[ts(export, export_to = "bindings/search/SearchResults.ts")]
#[serde(default)]
pub struct SearchResults {
    pub current: Vec<String>,
    /// Currently we only search credentials, therefore the recents list is called recent_credentials.
    pub recent_credentials: Vec<String>,
}

#[typetag::serde(name = "search_results")]
impl FeatTrait for SearchResults {}
