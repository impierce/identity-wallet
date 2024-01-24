use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::credential_offer::send_credential_request;

use ts_rs::TS;

#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialOffersSelected.ts")]
pub struct CredentialOffersSelected {
    #[ts(type = "Array<string>")]
    pub offer_indices: Vec<usize>,
}

#[typetag::serde(name = "[Credential Offer] Selected")]
impl ActionTrait for CredentialOffersSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(send_credential_request)]
    }
}
