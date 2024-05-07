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
};

use did_manager::SecretManager;
use log::info;
use oid4vc::oid4vc_core::Subject;
use oid4vc::oid4vc_manager::ProviderManager;
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

/// Creates a new profile with a new DID (using the did:key method) and sets it as the active profile.
pub async fn create_identity(mut state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(CreateNew {
        name, picture, theme, ..
    }) = listen::<CreateNew>(action)
    {
        let mut state_guard = state.core_utils.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;

        let default_did_method = state.profile_settings.default_did_method.as_str();

        let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

        let client_path = crate::persistence::STRONGHOLD
            .lock()
            .unwrap()
            .to_str()
            .ok_or(anyhow::anyhow!("failed to get stronghold path"))
            .unwrap()
            .to_owned();
        let password = "sup3rSecr3t".to_owned();
        let subject = Arc::new(
            SecretManager::load(client_path, password, "key-0".to_owned())
                .await
                .unwrap(),
        );

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
