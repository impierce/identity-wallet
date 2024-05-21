pub mod assert_state_update;
pub mod extensions;

use did_manager::SecretManager;
use identity_wallet::oid4vc_manager::ProviderManager;
use identity_wallet::oid4vci::Wallet;
use identity_wallet::persistence::STRONGHOLD;
use identity_wallet::state::credentials::VerifiableCredentialRecord;
use identity_wallet::subject::Subject;
use identity_wallet::{
    state::core_utils::{IdentityManager, Managers},
    stronghold::StrongholdManager,
};

use self::assert_state_update::setup_stronghold;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

const KEY_ID: &str = "key-0";
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
    setup_stronghold();

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

    let stronghold_snapshot_path = STRONGHOLD.lock().unwrap().to_string_lossy().to_string();

    let subject: Arc<Subject> = Arc::new(Subject {
        stronghold_manager: stronghold_manager.clone(),
        secret_manager: SecretManager::load(
            stronghold_snapshot_path,
            TEST_PASSWORD.to_string(),
            KEY_ID.to_string(),
            None,
            None,
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
