use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        profile_settings::{
            reducers::update_sorting_preference::{sort_connections, sort_credentials, update_sorting_preference},
            ConnectionSortMethod, CredentialSortMethod,
        },
        Reducer,
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone, Default)]
#[ts(export, export_to = "bindings/actions/UpdateSortingPreference.ts")]
pub struct UpdateSortingPreference {
    #[ts(optional)]
    pub credential_sorting: Option<CredentialSortMethod>,
    #[ts(optional)]
    pub connection_sorting: Option<ConnectionSortMethod>,
    #[ts(optional)]
    pub reverse: Option<bool>,
}

#[typetag::serde(name = "[Settings] Update sorting preference")]
impl ActionTrait for UpdateSortingPreference {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(update_sorting_preference),
            reducer!(sort_credentials),
            reducer!(sort_connections),
        ]
    }
}
