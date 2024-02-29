use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::user_data_query::{connection_query, credential_query};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum QueryTarget {
    Credentials,
    Connections,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum SortMethod {
    NameAZ,
    IssuanceNewOld,
    AddedNewOld,
    FirstInteractedNewOld,
    LastInteractedNewOld,
}

/// Action to query user data.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UserDataQuery.ts")]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[ts(optional)]
    pub search_term: Option<String>,
    #[ts(optional)]
    pub sort_method: Option<SortMethod>,
    #[ts(optional)]
    pub sort_reverse: Option<bool>,
}

#[typetag::serde(name = "[User Data] Query")]
impl ActionTrait for UserDataQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_query), reducer!(connection_query)]
    }
}
