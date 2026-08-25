use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        connections::reducers::handle_siopv2_authorization_request::handle_siopv2_authorization_request,
        profile_settings::reducers::update_sorting_preference::sort_connections,
        qr_code::reducers::read_authorization_request::read_authorization_request,
        qr_code::reducers::read_credential_offer::read_credential_offer, Reducer,
    },
};

use serde::{Deserialize, Serialize};

/// Action to accept a connection request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![
            reducer!(handle_siopv2_authorization_request),
            reducer!(read_authorization_request),
            reducer!(read_credential_offer),
            reducer!(sort_connections),
        ]
    }
}
