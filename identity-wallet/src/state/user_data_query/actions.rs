use ts_rs::TS;

use crate::{reducer, state::{actions::{ActionTrait, Reducer}, user_data_query::reducers::{connection_query, credential_query}}};

use super::{QueryTarget, SortMethod};



#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UserDataQuery.ts")]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[ts(optional)]
    pub search_term: Option<String>,
    #[ts(optional)]
    pub sort_method: Option<SortMethod>,
    #[serde(default)]
    pub sort_reverse: bool,
}

#[typetag::serde(name = "[User Data] Query")]
impl ActionTrait for UserDataQuery {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(credential_query), reducer!(connection_query)]
    }
}
