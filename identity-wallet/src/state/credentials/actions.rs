use crate::{reducer, state::{actions::{ActionTrait, Reducer}, connections::reducers::handle_oid4vp_authorization_request, credentials::reducers::{send_credential_request, update_credential_metadata}}};
use ts_rs::TS;

/// Actions

/// Action to update the credential metadata.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UpdateCredentialMetadata.ts")]
pub struct UpdateCredentialMetadata {
    #[ts(type = "string")]
    pub id: uuid::Uuid,
    #[ts(optional)]
    pub name: Option<String>,
    #[ts(optional)]
    pub icon: Option<String>,
    #[ts(optional)]
    pub color: Option<String>,
    #[ts(optional)]
    pub is_favorite: Option<bool>,
}

#[typetag::serde(name = "[Credential Metadata] Update")]
impl ActionTrait for UpdateCredentialMetadata {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(update_credential_metadata)]
    }
}

/// Action to authenticate the selected credentials.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialsSelected.ts")]
pub struct CredentialsSelected {
    #[ts(type = "Array<string>")]
    pub credential_uuids: Vec<uuid::Uuid>,
}

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_oid4vp_authorization_request)]
    }
}

/// Action to handle the selected credential offers.
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
