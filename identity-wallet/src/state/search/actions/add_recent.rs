use super::search_query::QueryTarget;
use crate::reducer;
use crate::state::search::reducers::add_recent::add_recent;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/AddRecent.ts")]
pub struct AddRecent {
    pub target: QueryTarget,
    pub search_hit: String,
}

#[typetag::serde(name = "[Search] Add Recent")]
impl ActionTrait for AddRecent {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(add_recent)]
    }
}
