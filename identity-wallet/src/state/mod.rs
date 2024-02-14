pub mod profile;
pub mod user_journey;
pub mod dev_mode;
pub mod boot;
pub mod credentials;
pub mod connections;
pub mod shared;
pub mod extension;
pub mod user_data_query;
//
pub mod actions;
pub mod persistence;
pub mod user_prompt;

use self::connections::reducers::ConnectionRequest;
use crate::state::connections::Connection;
use self::profile::Locale;
use self::profile::Profile;
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
use ts_rs::TS;
use dyn_clone::DynClone;

/// Trait which each field of the appstate has to implement.
#[typetag::serde(tag = "feat_state_type")]
pub trait FeatTrait: Send + Sync + std::fmt::Debug + DynClone + DowncastSync{}
dyn_clone::clone_trait_object!(FeatTrait);
impl_downcast!(sync FeatTrait);

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
    pub credentials: Vec<DisplayCredential>,
    pub current_user_prompt: Option<CurrentUserPrompt>,
    pub connections: Vec<Connection>,
    pub user_data_query: Vec<String>,
    /// Locale is a separate field from the profile only because of onboarding, 
    /// where the user needs to be able to choose the language before anything else.
    /// Locale and Profile are grouped together in the feature folder.
    pub locale: Locale,
    pub profile: Option<Profile>,
    #[ts(type = "object | null")]
    pub user_journey: Option<serde_json::Value>,
    /// Handled in command.rs, so no feature folder nor redux pattern needed.
    #[ts(type = "Array<string>")]
    pub debug_messages: VecDeque<String>,
    /// Extensions will bring in their own redux compliant code, in the unime folder.
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    //use tokio::runtime::Runtime;

    #[test]
    fn test_app_state_serialize() {
        let state = AppState {
            credentials: vec![],
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            debug_messages: Default::default(),
            connections: vec![],
            locale: Locale::En,
            profile: Some(Profile {
                name: "John Doe".to_string(),
                picture: None,
                theme: None,
                primary_did: "did:example:123".to_string(),
            }),
            user_journey: None,
            ..Default::default()
        };

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
                  "credentials": [],
                  "current_user_prompt": {
                    "type": "redirect",
                    "target": "me"
                  },
                  "connections": [],
                  "user_data_query": [],
                  "locale": "en",
                  "profile": {
                    "name": "John Doe",
                    "picture": null,
                    "theme": null,
                    "primary_did": "did:example:123"
                  },
                  "user_journey": null,
                  "debug_messages": [],
                  "extensions": {},
                  "dev_mode_enabled": false
                }"#}
        );
    }
}
