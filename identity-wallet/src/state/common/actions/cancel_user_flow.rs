use crate::{
    reducer,
    state::{actions::ActionTrait, common::reducers::cancel_user_flow::cancel_user_flow, Reducer},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to cancel the user flow and (optionally) redirect to a specific route.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CancelUserFlow.ts")]
pub struct CancelUserFlow {
    #[ts(optional)]
    pub redirect: Option<String>,
}

#[typetag::serde(name = "[User Flow] Cancel")]
impl ActionTrait for CancelUserFlow {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(cancel_user_flow)]
    }
}
