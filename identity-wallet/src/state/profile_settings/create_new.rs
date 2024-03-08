use crate::{
    error::AppError::{self, *},
    reducer,
    state::{
        actions::{listen, Action, ActionTrait},
        core_utils::IdentityManager,
        profile_settings::{Profile, ProfileSettings},
        user_prompt::CurrentUserPrompt,
        AppState, AppTheme, Reducer,
    },
    stronghold::StrongholdManager,
};

use did_key::{from_existing_key, Ed25519KeyPair};
use log::info;
use oid4vc::{
    oid4vc_core::Subject,
    oid4vc_manager::{methods::key_method::KeySubject, ProviderManager},
    oid4vci::Wallet,
};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::sync::Arc;
use ts_rs::TS;

/// Action to create a new profile.
#[derive(Serialize, Deserialize, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CreateNew.ts")]
pub struct CreateNew {
    pub name: String,
    pub picture: String,
    pub theme: AppTheme,
    pub password: String,
}

impl std::fmt::Debug for CreateNew {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateNew")
            .field("name", &self.name)
            .field("picture", &self.picture)
            .field("theme", &self.theme)
            .field("password", &"*****")
            .finish()
    }
}

#[typetag::serde(name = "[DID] Create new")]
impl ActionTrait for CreateNew {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(initialize_stronghold), reducer!(create_identity)]
    }
}

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
async fn create_identity(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateNew {
        name, picture, theme, ..
    }) = listen::<CreateNew>(action)
    {
        let mut state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

        let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
        let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

        let provider_manager = ProviderManager::new([subject.clone()]).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone());

        let profile_settings = ProfileSettings {
            profile: Some(Profile {
                name,
                picture: Some(picture),
                theme: Some(theme),
                primary_did: subject.identifier().map_err(OID4VCSubjectIdentifierError)?,
            }),
            ..Default::default()
        };

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        let current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

        drop(state_guard);

        return Ok(AppState {
            profile_settings,
            current_user_prompt,
            ..state
        })
    }

    Ok(state)
}

pub async fn initialize_stronghold(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<CreateNew>(action).map(|payload| payload.password) {
        state
            .core_utils
            .managers
            .lock()
            .await
            .stronghold_manager
            .replace(Arc::new(
                StrongholdManager::create(&password).map_err(StrongholdCreationError)?,
            ));

        info!("stronghold initialized");
    }

    Ok(state)
}
