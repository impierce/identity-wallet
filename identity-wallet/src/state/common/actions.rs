use crate::{reducer,
    state::{actions::{ActionTrait, Reducer},
    common::reducers::{cancel_user_flow, get_state, reset_state, unlock_storage}}
};
use std::fmt::Formatter;
use ts_rs::TS;

/// Action to retrieve the state from the storage.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/GetState.ts")]
pub struct GetState;

#[typetag::serde(name = "[App] Get state")]
impl ActionTrait for GetState {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(get_state)]
    }
}

/// Action to unlock the storage.
#[derive(serde::Serialize, serde::Deserialize, TS, Clone)]
#[ts(export, export_to = "bindings/actions/UnlockStorage.ts")]
pub struct UnlockStorage {
    pub password: String,
}

impl std::fmt::Debug for UnlockStorage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnlockStorage").field("password", &"*****").finish()
    }
}

#[typetag::serde(name = "[Storage] Unlock")]
impl ActionTrait for UnlockStorage {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(unlock_storage)]
    }
}

/// Action to cancel the user flow and redirect to the given target.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
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

/// Action to completely purge and reset the app state.
#[derive(serde::Serialize, serde::Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/Reset.ts")]
pub struct Reset;

#[typetag::serde(name = "[App] Reset")]
impl ActionTrait for Reset {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(reset_state)]
    }
}
