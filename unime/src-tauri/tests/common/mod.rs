use did_key::{from_existing_key, Ed25519KeyPair};
use identity_wallet::oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use identity_wallet::oid4vci::Wallet;
use identity_wallet::{
    stronghold::StrongholdManager,
    state::{IdentityManager, Managers},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub mod assert_state_update;
pub mod extensions;

pub const TEST_PASSWORD: &str = "sup3rSecr3t";

pub fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}

pub fn test_managers(
    verifiable_credential_records: Vec<VerifiableCredentialRecord>,
) -> Arc<tauri::async_runtime::Mutex<Managers>> {
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

    Arc::new(tauri::async_runtime::Mutex::new(Managers {
        stronghold_manager: Some(stronghold_manager),
        identity_manager: Some(IdentityManager {
            subject,
            provider_manager,
            wallet,
        }),
    }))
}
