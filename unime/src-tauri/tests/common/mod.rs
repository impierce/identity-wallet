pub mod assert_state_update;
pub mod extensions;

use did_manager::SecretManager;
use identity_wallet::oid4vc_manager::ProviderManager;
use identity_wallet::oid4vci::Wallet;
use identity_wallet::persistence::STRONGHOLD;
use identity_wallet::state::credentials::VerifiableCredentialRecord;
use identity_wallet::state::{AppStateContainer, SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
use identity_wallet::subject::Subject;
use identity_wallet::{
    state::core_utils::{IdentityManager, Managers},
    stronghold::StrongholdManager,
};
use tokio::sync::{Mutex, OnceCell};

use self::assert_state_update::setup_stronghold;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;

const KEY_ID: &str = "ed25519-0";
pub const TEST_PASSWORD: &str = "sup3rSecr3t";

/// The origin of the app's own pages, which mock IPC requests must come from: Tauri 2.12 rejects app commands from any
/// other origin. Windows and Android serve the app from `http://tauri.localhost` instead of the `tauri://` scheme.
pub fn app_origin() -> url::Url {
    let origin = if cfg!(any(windows, target_os = "android")) {
        "http://tauri.localhost"
    } else {
        "tauri://localhost"
    };
    origin.parse().unwrap()
}

/// Builds a mock app that manages `state` and handles `handle_action`.
///
/// The only `tauri::generate_context!()` in the test crate: on macOS every expansion defines the same
/// `_EMBED_INFO_PLIST` symbol, so a second one fails to compile.
pub fn mock_app(state: AppStateContainer) -> tauri::App<tauri::test::MockRuntime> {
    tauri::test::mock_builder()
        .manage(state)
        .invoke_handler(tauri::generate_handler![unime::tauri_command::handle_action])
        .build(tauri::generate_context!())
        .unwrap()
}

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
        secret_manager: Arc::new(Mutex::new(
            SecretManager::builder()
                .snapshot_path(&stronghold_snapshot_path)
                .with_ed25519_key(KEY_ID)
                .password(TEST_PASSWORD)
                .build()
                .await
                .unwrap(),
        )),
        resolver: OnceCell::new(),
    });

    let provider_manager = ProviderManager::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .unwrap();
    let wallet: Wallet = Wallet::new(
        subject.clone(),
        Vec::from(SUPPORTED_DID_METHODS),
        Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
    )
    .unwrap();

    Arc::new(tauri::async_runtime::Mutex::new(Managers {
        stronghold_manager: Some(stronghold_manager),
        identity_manager: Some(IdentityManager {
            subject,
            provider_manager,
            wallet,
        }),
    }))
}
