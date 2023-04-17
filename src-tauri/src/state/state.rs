use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default)]
pub struct AppState {
    pub status: Mutex<StateStatus>,
    pub active_profile: Mutex<Option<Profile>>,
    pub locale: Mutex<String>,
}

// TODO: do we really need that StateStatus?
#[derive(Clone, Copy, Debug, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
#[ts(export)]
pub enum StateStatus {
    Loading,
    Stable,
}

impl Default for StateStatus {
    fn default() -> Self {
        StateStatus::Stable
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TransferState {
    pub status: StateStatus,
    pub active_profile: Option<Profile>,
    pub locale: String,
}

// TODO: only the AppState should have a default
impl Default for TransferState {
    fn default() -> Self {
        TransferState {
            status: StateStatus::Stable,
            active_profile: None,
            locale: "en".to_string(),
        }
    }
}

impl From<AppState> for TransferState {
    fn from(state: AppState) -> TransferState {
        TransferState {
            status: *state.status.lock().unwrap(),
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: (*state.locale.lock().unwrap()).to_string(),
        }
    }
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            status: *state.status.lock().unwrap(),
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: (*state.locale.lock().unwrap()).to_string(),
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub struct Profile {
    pub display_name: String,
    pub primary_did: String,
}
