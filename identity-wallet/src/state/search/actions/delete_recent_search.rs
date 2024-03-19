use crate::reducer;
use crate::state::search::reducers::delete_recent_search::delete_recent_search;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::search_query::QueryTarget;

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DeleteRecentSearch.ts")]
pub struct DeleteRecentSearch {
    pub delete_target: QueryTarget,
    pub search_term: String,
}

#[typetag::serde(name = "[Search] Delete Recent")]
impl ActionTrait for DeleteRecentSearch {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(delete_recent_search)]
    }
}
