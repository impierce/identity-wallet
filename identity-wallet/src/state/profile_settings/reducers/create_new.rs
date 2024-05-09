use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        core_utils::IdentityManager,
        profile_settings::{actions::create_new::CreateNew, Profile, ProfileSettings},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
    stronghold::StrongholdManager,
    subject::subject,
};

use log::info;
use oid4vc::oid4vc_core::Subject as _;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_identity(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateNew {
        name,
        picture,
        theme,
        password,
    }) = listen::<CreateNew>(action)
    {
        let mut state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let default_did_method = state.profile_settings.default_did_method.as_str();

        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager =
            ProviderManager::new(subject.clone(), default_did_method).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone(), default_did_method).map_err(OID4VCWalletError)?;

        let profile_settings = ProfileSettings {
            profile: Some(Profile {
                name,
                picture: Some(picture),
                theme,
                primary_did: subject
                    .identifier(default_did_method)
                    .await
                    .map_err(OID4VCSubjectIdentifierError)?,
            }),
            ..state.profile_settings
        };

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        state.profile_settings = profile_settings;
        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
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
