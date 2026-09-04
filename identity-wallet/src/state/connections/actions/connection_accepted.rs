use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        connections::reducers::handle_siopv2_authorization_request::handle_siopv2_authorization_request,
        profile_settings::reducers::update_sorting_preference::sort_connections,
        qr_code::reducers::read_authorization_request::read_oid4vp_authorization_request,
        qr_code::reducers::read_credential_offer::read_credential_offer, Reducer,
    },
};

use serde::{Deserialize, Serialize};

/// Action to accept a connection request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionAccepted;

// The first 3 reducers are executed in an OR/OR/OR manner, matching against the active flow, which is set after the QrCodeScanned action.
#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(handle_siopv2_authorization_request),
            reducer!(read_oid4vp_authorization_request),
            reducer!(read_credential_offer),
            reducer!(sort_connections),
        ]
    }
}
