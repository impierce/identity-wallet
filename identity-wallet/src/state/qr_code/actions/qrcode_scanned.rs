use crate::state::actions::ActionTrait;
use crate::state::qr_code::reducers::read_authorization_request::read_authorization_request;
use crate::state::qr_code::reducers::read_credential_offer::read_credential_offer;
use crate::{reducer, state::Reducer};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to handle the scanning of a QR code.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
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
