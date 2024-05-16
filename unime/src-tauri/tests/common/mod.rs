pub mod assert_state_update;
pub mod extensions;

use did_manager::SecretManager;
use identity_wallet::oid4vc_manager::ProviderManager;
use identity_wallet::oid4vci::Wallet;
use identity_wallet::state::credentials::VerifiableCredentialRecord;
use identity_wallet::subject::Subject;
use identity_wallet::{
    state::core_utils::{IdentityManager, Managers},
    stronghold::StrongholdManager,
};

use log::debug;
use rand::distributions::DistString;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

pub const TEST_PASSWORD: &str = "sup3rSecr3t";

pub fn json_example<T>(path: &str) -> T
where
    T: DeserializeOwned,
{
    let file_path = Path::new(path);
    let file = File::open(file_path).expect("file does not exist");
    serde_json::from_reader::<_, T>(file).expect("could not parse json")
}

pub async fn test_managers(
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

    let subject: Arc<Subject> = Arc::new(Subject {
        stronghold_manager: stronghold_manager.clone(),
        secret_manager: SecretManager::generate(
            random_stronghold_path().to_string_lossy().into_owned(),
            TEST_PASSWORD.to_string(),
        )
        .await
        .unwrap(),
    });

    let provider_manager = ProviderManager::new(subject.clone(), "did:key").unwrap();
    let wallet: Wallet = Wallet::new(subject.clone(), "did:key").unwrap();

    Arc::new(tauri::async_runtime::Mutex::new(Managers {
        stronghold_manager: Some(stronghold_manager),
        identity_manager: Some(IdentityManager {
            subject,
            provider_manager,
            wallet,
        }),
    }))
}

pub fn random_stronghold_path() -> std::path::PathBuf {
    let mut file = std::env::temp_dir();
    file.push("test_strongholds");
    file.push(rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 32));
    file.set_extension("stronghold");
    debug!("Stronghold path: {:?}", file);
    file.to_owned()
}
