pub mod actions;
pub mod persistence;
pub mod reducers;
pub mod user_prompt;

use self::reducers::authorization::ConnectionRequest;
use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use oid4vc_core::Subject;
use oid4vc_manager::ProviderManager;
use oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};
use ts_rs::TS;

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

/// The inner state of the application managed by Tauri. When the state is serialized in order to be sent to the
/// frontend, the `managers` and `active_connection_request` fields are skipped.
#[derive(Default, Serialize, Deserialize, Derivative, TS)]
#[derivative(Debug)]
#[ts(export)]
#[serde(default)]
pub struct AppState {
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub managers: tauri::async_runtime::Mutex<Managers>,
    pub active_profile: Option<Profile>,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub active_connection_request: Option<ConnectionRequest>,
    pub locale: Locale,
    pub credentials: Vec<DisplayCredential>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    pub dev_mode_enabled: bool,
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    #[ts(type = "object | null")]
    pub user_journey: Option<serde_json::Value>,
    pub connections: Vec<Connection>,
    pub user_data_query: Vec<String>,
}

#[derive(Default)]
pub struct AppStateContainer(pub tokio::sync::Mutex<AppState>);

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
    pub first_interacted: String,
    pub last_interacted: String,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum QueryTarget {
    Credentials,
    Connections,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub enum SortMethod {
    NameAZ,
    IssuanceNewOld,
    AddedNewOld,
    FirstInteractedNewOld,
    LastInteractedNewOld,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq)]
#[ts(export)]
pub struct UserDataQuery {
    pub target: QueryTarget,
    #[serde(default)]
    pub search_term: Option<String>,
    #[serde(default)]
    pub sort_method: Option<SortMethod>,
    #[serde(default)]
    pub sort_reverse: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_app_state_serialize() {
        let state = AppState {
            active_profile: Some(Profile {
                name: "John Doe".to_string(),
                picture: None,
                theme: None,
                primary_did: "did:example:123".to_string(),
            }),
            locale: Locale::En,
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            user_journey: None,
            connections: vec![],
            ..Default::default()
        };

        let serialized = serde_json::to_string_pretty(&state).unwrap();

        // AppState is serialized without the `managers` and `active_connection_request` fields.
        // Probably a basic json file instead of the indoc! is cleaner.
        assert_eq!(
            serialized,
            indoc! {
            r#"{
                  "active_profile": {
                    "name": "John Doe",
                    "picture": null,
                    "theme": null,
                    "primary_did": "did:example:123"
                  },
                  "locale": "en",
                  "credentials": [],
                  "current_user_prompt": {
                    "type": "redirect",
                    "target": "me"
                  },
                  "dev_mode_enabled": false,
                  "debug_messages": [],
                  "user_journey": null,
                  "connections": [],
                  "user_data_query": []
                }"#}
        );
    }
}
