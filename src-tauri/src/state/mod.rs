pub mod actions;
pub mod persistence;
pub mod reducers;

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use ts_rs::TS;

/// The inner state of the application managed by Tauri.
#[derive(Default)]
pub struct AppState {
    pub active_profile: Mutex<Option<Profile>>,
    pub locale: Mutex<String>,
}

/// A representation of the current state which is used for serialization.
#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct TransferState {
    pub active_profile: Option<Profile>,
    pub locale: String,
}

impl From<&AppState> for TransferState {
    fn from(state: &AppState) -> TransferState {
        TransferState {
            active_profile: state.active_profile.lock().unwrap().clone(),
            locale: (*state.locale.lock().unwrap()).to_string(),
        }
    }
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub struct Profile {
    pub display_name: String,
    pub primary_did: String,
}
