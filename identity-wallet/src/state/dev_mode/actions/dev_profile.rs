use crate::{
    reducer,
    state::{actions::ActionTrait, dev_mode::reducers::dev_action::dev_action, Reducer},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// shenron: this should be refactored to just dev_actions containing enum: static_ferris, ngdil { enum: Connect, Share, Receive }, selv { enum: ", ", "}.

/// Action with enum to perform Dev mode action
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/DevAction.ts")]
pub struct DevAction {
    pub action: DevActionType,
}

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/DevActionType.ts")]
pub enum DevActionType {
    DeleteProfile,
    EmptyProfile,
    LoadStaticFerris,
    NGDIL { step: OID4VCSteps, auto_confirm: bool },
    Selv { step: OID4VCSteps, auto_confirm: bool },
}

#[derive(Serialize, Deserialize, Debug, TS, Clone, PartialEq, Eq)]
#[ts(export, export_to = "bindings/dev/OID4VCStep.ts")]
pub enum OID4VCSteps {
    Connect,
    Share,
    Receive,
    All,
}

#[typetag::serde(name = "[DEV] Action")]
impl ActionTrait for DevAction {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(dev_action)]
    }
}
