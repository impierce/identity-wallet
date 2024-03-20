use crate::{
    reducer,
    state::{
        actions::ActionTrait, profile_settings::{reducers::update_sorting_preference::update_sorting_preference, ConnectionSorting, CredentialSorting}, Reducer
    },
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateSortingPreference.ts")]
pub struct UpdateSortingPreference {
    pub credential_sorting: Option<CredentialSorting>,
    pub connection_sorting: Option<ConnectionSorting>
}

#[typetag::serde(name = "[Settings] Update Sorting Preference")]
impl ActionTrait for UpdateSortingPreference {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_sorting_preference)]
    }
}

