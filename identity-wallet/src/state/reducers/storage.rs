use crate::{
    stronghold::StrongholdManager,
    error::AppError::{self, *},
    state::{
        actions::{listen, Action, UnlockStorage},
        user_prompt::CurrentUserPrompt,
        AppState, IdentityManager,
    },
};
use did_key::{from_existing_key, Ed25519KeyPair};
use log::info;
use oid4vc::oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use oid4vc::oid4vci::Wallet;
use std::sync::Arc;

pub async fn unlock_storage(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(password) = listen::<UnlockStorage>(action).map(|payload| payload.password) {
        let mut state_guard = state.core_state.managers.lock().await;

        let stronghold_manager = Arc::new(StrongholdManager::load(&password).map_err(StrongholdLoadingError)?);

        let public_key = stronghold_manager.get_public_key().map_err(StrongholdPublicKeyError)?;

        let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
        let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

        let provider_manager = ProviderManager::new([subject.clone()]).map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(subject.clone());

        info!("loading credentials from stronghold");
        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        state_guard.stronghold_manager.replace(stronghold_manager);

        state_guard.identity_manager.replace(IdentityManager {
            subject,
            provider_manager,
            wallet,
        });

        info!("storage unlocked");

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            ..state
        });
    }

    Ok(state)
}
