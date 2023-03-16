use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// #[derive(Clone)]
pub struct AppState {
    pub status: Mutex<StateStatus>,
}

impl AppState {
    pub fn new(status: StateStatus) -> Self {
        AppState {
            status: Mutex::new(status),
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

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TransferState {
    status: StateStatus,
}

impl From<AppState> for TransferState {
    fn from(state: AppState) -> TransferState {
        TransferState {
            status: *state.status.lock().unwrap(),
        }
    }
}
