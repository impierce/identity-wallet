use crate::{
    reducer,
    state::{
        actions::ActionTrait,
        connections::reducers::handle_siopv2_authorization_request::handle_siopv2_authorization_request, Reducer,
    },
};

use serde::{Deserialize, Serialize};

/// Action to accept a connection request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_siopv2_authorization_request)]
    }
}
