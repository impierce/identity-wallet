use did_key::{from_existing_key, Ed25519KeyPair};
use identity_wallet::{
    crypto::stronghold::StrongholdManager,
    state::{IdentityManager, Managers},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use oid4vci::Wallet;
use serde_json::json;
use std::sync::Arc;

pub mod assert_state_update;

pub const TEST_PASSWORD: &str = "sup3rSecr3t";

pub fn test_managers(
    verifiable_credential_records: Vec<VerifiableCredentialRecord>,
) -> tauri::async_runtime::Mutex<Managers> {
    let stronghold_manager = Arc::new(StrongholdManager::create(TEST_PASSWORD).unwrap());

    verifiable_credential_records
        .into_iter()
        .for_each(|verifiable_credential_record| {
            stronghold_manager
                .insert(
                    verifiable_credential_record.display_credential.id.parse().unwrap(),
                    json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
                )
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
            subject,
            provider_manager,
            wallet,
        }),
    })
}
