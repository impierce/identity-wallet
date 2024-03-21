use crate::reducer;
use crate::state::search::reducers::add_recent_search::add_recent_search;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/AddRecentSearch.ts")]
pub struct AddRecentSearch {
    pub search_hit: String,
}

#[typetag::serde(name = "[Search] Add Recent Search")]
impl ActionTrait for AddRecentSearch {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(add_recent_search)]
    }
}
