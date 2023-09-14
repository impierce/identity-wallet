use crate::{
    crypto::stronghold::StrongholdManager,
    state::{
        actions::Action,
        user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Redirect},
        AppState, IdentityManager,
    },
};
use did_key::{from_existing_key, Ed25519KeyPair};
use log::info;
use oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use oid4vci::Wallet;
use std::sync::Arc;

pub async fn unlock_storage(state: &AppState, action: Action) -> anyhow::Result<()> {
    let mut state_guard = state.managers.lock().await;

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let password = payload["password"]
        .as_str()
        .ok_or(anyhow::anyhow!("unable to read password from json payload"))?;

    let stronghold_manager = Arc::new(StrongholdManager::load(password)?);

    let public_key = stronghold_manager.get_public_key()?;

    let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
    let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

    let provider_manager = ProviderManager::new([subject.clone()])?;
    let wallet: Wallet = Wallet::new(subject.clone());

    info!("loading credentials from stronghold");
    *state.credentials.lock().unwrap() = stronghold_manager
        .values()?
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

    state
        .current_user_prompt
        .lock()
        .unwrap()
        .replace(CurrentUserPrompt::Redirect(Redirect {
            r#type: CurrentUserPromptType::Redirect,
            target: "me".to_string(),
        }));

    Ok(())
}
