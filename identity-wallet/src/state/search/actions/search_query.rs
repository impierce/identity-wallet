use crate::reducer;
use crate::state::search::reducers::search_query::credential_search;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SearchQuery.ts")]
pub struct SearchQuery {
    pub search_term: String,
}

#[typetag::serde(name = "[Search] Query")]
impl ActionTrait for SearchQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_search)]
    }
}
