use crate::reducer;
use crate::state::credentials::reducers::send_credential_request::send_credential_request;
use crate::state::{actions::ActionTrait, Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to handle the selection of credential offers.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialOffersSelected.ts")]
pub struct CredentialOffersSelected {
    pub credential_configuration_ids: Vec<String>,
}

#[typetag::serde(name = "[Credential Offer] Selected")]
impl ActionTrait for CredentialOffersSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_credential_request)]
    }
}
