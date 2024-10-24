use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::IdentityManager,
        profile_settings::{actions::create_new::CreateNew, Profile, ProfileSettings},
        user_prompt::CurrentUserPrompt,
        AppState, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS,
    },
    stronghold::StrongholdManager,
    subject::subject,
};

use jsonwebtoken::Algorithm;
use log::info;
use oid4vc::oid4vc_core::Subject as _;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

/// Creates a new profile, produces (deterministic) DIDs and redirects to the main page.
pub async fn create_identity(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateNew {
        name,
        picture,
        theme,
        password,
    }) = listen::<CreateNew>(action)
    {
        info!("Creating new identity ...");
        let mut state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCWalletError)?;

        let mut dids = state.dids;

        let did_jwk = subject
            // TODO: make distinction between keys using the same DID Method but different algorithms.
            .identifier("did:jwk", Algorithm::EdDSA)
            .await
            .map_err(|e| Error(e.to_string()))?;
        dids.insert("did:jwk".to_string(), did_jwk);

        let did_key = subject
            // TODO: make distinction between keys using the same DID Method but different algorithms.
            .identifier("did:key", Algorithm::EdDSA)
            .await
            .map_err(|e| Error(e.to_string()))?;
        dids.insert("did:key".to_string(), did_key);

        let profile_settings = ProfileSettings {
            profile: Some(Profile {
                name,
                picture: Some(picture),
                theme,
            }),
            ..state.profile_settings
        };

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        drop(state_guard);
        return Ok(AppState {
            dids,
            profile_settings,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
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
        return Ok(state);
    }

    Ok(state)
}
