use crate::reducer;
use crate::state::actions::ActionTrait;
use crate::state::actions::Reducer;
use crate::state::reducers::reset_state;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}
