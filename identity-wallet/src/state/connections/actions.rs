use crate::{reducer, state::{actions::{ActionTrait, Reducer}, connections::reducers::handle_siopv2_authorization_request}};
use ts_rs::TS;

/// Action to accept a connection request.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/ConnectionAccepted.ts")]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_siopv2_authorization_request)]
    }
}
