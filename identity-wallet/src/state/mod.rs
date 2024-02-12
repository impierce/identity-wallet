pub mod features;
pub mod actions;
pub mod reducers;
pub mod persistence;
pub mod user_prompt;

use self::reducers::authorization::ConnectionRequest;
use self::features::profile::Profile;
use crate::{
    crypto::stronghold::StrongholdManager, state::user_prompt::CurrentUserPrompt,
    verifiable_credential_record::DisplayCredential,
};
use derivative::Derivative;
use downcast_rs::{impl_downcast, DowncastSync};
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};
use strum::EnumString;
use ts_rs::TS;
use dyn_clone::DynClone;

#[derive(Default, Debug)]
pub struct AppStateContainer(pub tokio::sync::Mutex<AppState>);

impl AppStateContainer{
    pub async fn insert_extension (self, key: &str, extension: Box<dyn FeatTrait>) -> Self
    {
        self.0.lock().await.extensions.insert(key.to_string(), extension);
        self
    }
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
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    #[serde(skip)]
    #[derivative(Debug = "ignore")]
    pub active_connection_request: Option<ConnectionRequest>,
    pub locale: Locale,
    pub credentials: Vec<DisplayCredential>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    #[ts(type = "object | null")]
    pub user_journey: Option<serde_json::Value>,
    pub connections: Vec<Connection>,
    pub user_data_query: Vec<String>,
    ////
    pub profile: Option<Profile>,
    #[ts(skip)]
    pub extensions: std::collections::HashMap<String, Box<dyn FeatTrait>>,
    pub dev_mode_enabled: bool,
}

impl AppState{
    pub fn insert_extension (mut self, key: &str, extension: Box<dyn FeatTrait>) -> Self
    {
        self.extensions.insert(key.to_string(), extension);
        self
    }
}

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

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default, EnumString)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Locale {
    #[default]
    En,
    De,
    Nl,
}

#[derive(Clone, Serialize, Debug, Deserialize, TS, PartialEq, Default)]
#[ts(export)]
#[serde(default)]
pub struct Connection {
    pub id: String,
    pub client_name: String,
    pub url: String,
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

// Trait which each field of the appstate has to implement.
#[typetag::serde(tag = "feat_state_type")]
pub trait FeatTrait: Send + Sync + std::fmt::Debug + DynClone + DowncastSync{}
dyn_clone::clone_trait_object!(FeatTrait);
impl_downcast!(sync FeatTrait);

// Testing extensions

#[derive(Debug, Serialize, Deserialize, TS, PartialEq, Default, Clone)]
pub struct CustomExtension {
    pub name: String,
    pub value: String,
}

#[typetag::serde(name = "customextension")]
impl FeatTrait for CustomExtension {}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    //use tokio::runtime::Runtime;

    #[test]
    fn test_app_state_serialize() {
        let state = AppState {
            locale: Locale::En,
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            user_journey: None,
            connections: vec![],
            ..Default::default()
        }
        .add_feat_state("profile", Box::new(Profile {
            name: "John Doe".to_string(),
            picture: None,
            theme: None,
            primary_did: "did:example:123".to_string(),
        }));

        //let mut serialized = String::new();
        //let rt = Runtime::new().unwrap();
        //rt.block_on(async {
            let serialized = serde_json::to_string_pretty(&state).unwrap();
        //});

        // AppState is serialized without the `managers` and `active_connection_request` fields.
        // Probably a basic json file instead of the indoc! is cleaner.
        assert_eq!(
            serialized,
            indoc! {
            r#"{
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
                  "user_data_query": [],
                  "feat_states": {
                    "profile": {
                      "feat_state_type": "profile",
                      "name": "John Doe",
                      "picture": null,
                      "theme": null,
                      "primary_did": "did:example:123"
                    }
                  },
                  "extensions": {}
                }"#}
        );
    }
}
