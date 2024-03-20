use crate::reducer;
use crate::state::search::reducers::user_data_query::{connection_search, credential_search};
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/SearchQuery.ts")]
pub struct SearchQuery {
    pub target: QueryTarget,
    pub search_term: String
}

#[typetag::serde(name = "[Search] Query")]
impl ActionTrait for SearchQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_search), reducer!(connection_search)]
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum QueryTarget {
    Credentials,
    Connections,
}

// Sorting functionality will be moved elsewhere in future PR

// #[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
// #[ts(export)]
// pub enum SortMethod {
//     NameAZ,
//     IssuanceNewOld,
//     AddedNewOld,
//     FirstInteractedNewOld,
//     LastInteractedNewOld,
// }
