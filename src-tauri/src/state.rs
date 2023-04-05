use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// #[derive(Clone)]
pub struct AppState {
    pub status: Mutex<StateStatus>,
    pub active_profile: Mutex<Option<Profile>>,
}

impl AppState {
    pub fn new(status: StateStatus, active_profile: Option<Profile>) -> Self {
        AppState {
            status: Mutex::new(status),
            active_profile: Mutex::new(active_profile),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, TS)]
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
}

impl From<AppState> for TransferState {
    fn from(state: AppState) -> TransferState {
        TransferState {
            status: *state.status.lock().unwrap(),
            active_profile: state.active_profile.lock().unwrap().clone(),
        }
    }
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            status: *state.status.lock().unwrap(),
            active_profile: state.active_profile.lock().unwrap().clone(),
        }
    }
}

#[derive(Clone, Serialize, Debug, Deserialize, TS)]
#[ts(export)]
pub struct Profile {
    pub display_name: String,
    pub primary_did: String,
}
