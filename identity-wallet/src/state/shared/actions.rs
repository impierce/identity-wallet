use crate::{reducer, state::{actions::{ActionTrait, Reducer}, connections::reducers::read_authorization_request, credentials::reducers::read_credential_offer}};
use ts_rs::TS;

/// Actions

/// Action to scan a QR code.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/QrCodeScanned.ts")]
pub struct QrCodeScanned {
    pub form_urlencoded: String,
}

#[typetag::serde(name = "[QR Code] Scanned")]
impl ActionTrait for QrCodeScanned {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(read_authorization_request), reducer!(read_credential_offer)]
    }
}
