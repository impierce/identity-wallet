pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use oid4vc_core::Subject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use ts_rs::TS;

use self::reducers::authorization::ConnectionRequest;

pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// The inner state of the application managed by Tauri.
#[derive(Default, Serialize, Deserialize, Derivative, TS)]
#[derivative(Debug)]
#[ts(export)]
pub struct AppState {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub managers: tauri::async_runtime::Mutex<Managers>,
    pub active_profile: Mutex<Option<Profile>>,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub active_connection_request: Mutex<Option<ConnectionRequest>>,
    pub locale: Mutex<Locale>,
    pub credentials: Mutex<Vec<DisplayCredential>>,
    pub current_user_prompt: Mutex<Option<CurrentUserPrompt>>,
    pub debug_messages: Mutex<Vec<String>>,
    #[ts(type = "object")]
    pub user_journey: Mutex<Option<serde_json::Value>>,
    pub connections: Mutex<Vec<Connection>>,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

/// A profile of the current user.
#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Profile {
    pub name: String,
    pub picture: Option<String>,
    pub theme: Option<String>,
    pub primary_did: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub client_name: String,
    pub url: String,
    pub logo_uri: Option<String>,
    pub verified: bool,
    pub first_connected: String,
    pub last_connected: String,
}
