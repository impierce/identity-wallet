use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::authorization::handle_siopv2_authorization_request;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct ConnectionAccepted;

#[typetag::serde(name = "[Authenticate] Connection accepted")]
impl ActionTrait for ConnectionAccepted {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_siopv2_authorization_request)]
    }
}
