use std::sync::Mutex;

use serde::{Deserialize, Serialize};

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

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum StateStatus {
    Loading,
    Stable,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct TransferState {
    state: StateStatus,
}

impl From<AppState> for TransferState {
    fn from(state: AppState) -> TransferState {
        TransferState {
            state: *state.status.lock().unwrap(),
        }
    }
}
