use did_key::{from_existing_key, Ed25519KeyPair};
use identity_wallet::{
    crypto::stronghold::StrongholdManager,
    state::{IdentityManager, Managers},
};
use oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use oid4vci::{
    credential_format_profiles::{CredentialFormats, WithCredential},
    Wallet,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub mod assert_state_update;

pub const TEST_PASSWORD: &str = "my-password";

pub fn test_managers(
    verifiable_credentials: Vec<(Uuid, CredentialFormats<WithCredential>)>,
) -> tauri::async_runtime::Mutex<Managers> {
    let stronghold_manager = Arc::new(StrongholdManager::create(TEST_PASSWORD).unwrap());

    verifiable_credentials
        .into_iter()
        .for_each(|(uuid, verifiable_credential)| {
            stronghold_manager
                .insert(uuid, json!(verifiable_credential).to_string().as_bytes().to_vec())
                .unwrap();
        });

    let public_key = stronghold_manager.get_public_key().unwrap();

    let keypair = from_existing_key::<Ed25519KeyPair>(public_key.as_slice(), None);
    let subject = Arc::new(KeySubject::from_keypair(keypair, Some(stronghold_manager.clone())));

    let provider_manager = ProviderManager::new([subject.clone()]).unwrap();
    let wallet: Wallet = Wallet::new(subject.clone());

    tauri::async_runtime::Mutex::new(Managers {
        stronghold_manager: Some(stronghold_manager),
        identity_manager: Some(IdentityManager {
            provider_manager,
            wallet,
        }),
    })
}
