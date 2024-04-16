use crate::reducer;
use crate::state::credentials::reducers::update_credential_metadata::update_credential_metadata;
use crate::state::profile_settings::reducers::update_sorting_preference::sort_credentials;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to update the credential metadata.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateCredentialMetadata.ts")]
pub struct UpdateCredentialMetadata {
    #[ts(type = "string")]
    pub id: uuid::Uuid,
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub is_favorite: Option<bool>,
}

#[typetag::serde(name = "[Credential Metadata] Update")]
impl ActionTrait for UpdateCredentialMetadata {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_credential_metadata), reducer!(sort_credentials)]
    }
}
