use identity_wallet::oid4vc_core::Subject as _;
use identity_wallet::persistence::{STATE_FILE, STRONGHOLD};
use identity_wallet::state::credentials::DisplayCredential;
use identity_wallet::state::profile_settings::Locale;
use identity_wallet::state::user_prompt::CurrentUserPrompt;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use jsonwebtoken::Algorithm;
use oauth_tsl::status_list::StatusType;
use serde_json::json;
use std::path::Path;
use tauri::Manager;
use tempfile::TempDir;

/// Profile created with UniMe 0.18.0 through the regular onboarding, see `tests/fixtures/profiles/v0.18.0/README.md`.
const PROFILE_V0_18_0: &str = "tests/fixtures/profiles/v0.18.0";
const PROFILE_V0_18_0_PASSWORD: &str = "Fixture123!";

/// Copies the profile fixture into a temporary data directory, since every handled action saves the state again.
fn setup_profile_fixture(fixture_dir: &str) -> TempDir {
    let data_dir = tempfile::tempdir().unwrap();

    for file in ["state.json", "stronghold.bin"] {
        std::fs::copy(Path::new(fixture_dir).join(file), data_dir.path().join(file)).unwrap();
    }

    *STATE_FILE.lock().unwrap() = data_dir.path().join("state.json");
    *STRONGHOLD.lock().unwrap() = data_dir.path().join("stronghold.bin");

    data_dir
}

async fn handle_actions(actions: Vec<Action>) -> AppState {
    let app = tauri::test::mock_builder()
        .manage(AppStateContainer::default())
        .invoke_handler(tauri::generate_handler![unime::tauri_command::handle_action])
        .build(tauri::generate_context!())
        .unwrap();

    let window = tauri::webview::WebviewWindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    for action in actions {
        tauri::test::assert_ipc_response(
            &window,
            tauri::webview::InvokeRequest {
                cmd: "handle_action".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: url::Url::parse("http://localhost").unwrap(),
                body: json!({ "action": action }).into(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
            Ok(()),
        );
    }

    let container = app.app_handle().state::<AppStateContainer>().inner();
    let state = container.0.lock().await.clone();
    state
}

fn get_state() -> Action {
    serde_json::from_value(json!({ "type": "[App] Get state" })).unwrap()
}

fn unlock_storage(password: &str) -> Action {
    serde_json::from_value(json!({ "type": "[Storage] Unlock", "payload": { "password": password } })).unwrap()
}

fn credential<'a>(credentials: &'a [DisplayCredential], display_name: &str) -> &'a DisplayCredential {
    credentials
        .iter()
        .find(|credential| credential.display_name == display_name)
        .unwrap_or_else(|| panic!("credential `{display_name}` is missing"))
}

#[tokio::test]
#[serial_test::serial]
async fn test_profile_v0_18_0_asks_for_password() {
    let _data_dir = setup_profile_fixture(PROFILE_V0_18_0);

    let state = handle_actions(vec![get_state()]).await;

    assert_eq!(state.current_user_prompt, Some(CurrentUserPrompt::PasswordRequired));
    assert!(!state.is_unlocked);
    assert_eq!(state.profile_settings.profile.unwrap().name, "Fixture Holder");
}

#[tokio::test]
#[serial_test::serial]
async fn test_profile_v0_18_0_unlocks() {
    let _data_dir = setup_profile_fixture(PROFILE_V0_18_0);

    let state = handle_actions(vec![get_state(), unlock_storage(PROFILE_V0_18_0_PASSWORD)]).await;

    assert_eq!(
        state.current_user_prompt,
        Some(CurrentUserPrompt::Redirect {
            target: "me".to_string()
        })
    );
    assert!(state.is_unlocked);

    let profile_settings = &state.profile_settings;
    assert_eq!(profile_settings.profile.as_ref().unwrap().name, "Fixture Holder");
    assert_eq!(profile_settings.locale, Locale::en_US);

    // Credentials are read from the Stronghold vault on unlock.
    let credentials = &state.credentials;
    assert_eq!(credentials.len(), 5);

    let expected_credentials = [
        ("Dutch Driver's Licence", "Mira", StatusType::VALID),
        ("My first badge", "Mira", StatusType::VALID),
        ("Complete VC", "Mira", StatusType::INVALID),
        ("Full Badge", "Capella", StatusType::VALID),
        ("Testing Credential", "Capella", StatusType::VALID),
    ];
    for (display_name, issuer_name, status) in expected_credentials {
        let credential = credential(credentials, display_name);
        assert_eq!(credential.issuer_name, issuer_name);
        assert_eq!(credential.credential_status.as_ref().unwrap().status, status);
        assert!(!credential.display_claims.is_empty(), "`{display_name}` has no claims");

        let connection_id = credential.connection_id.as_ref().unwrap();
        let connection = state
            .connections
            .0
            .iter()
            .find(|connection| &connection.id == connection_id)
            .unwrap_or_else(|| panic!("connection of `{display_name}` is missing"));
        assert_eq!(connection.name, issuer_name);
    }

    let licence = credential(credentials, "Dutch Driver's Licence");
    assert_eq!(licence.data["credentialSubject"]["Licence number"], "FIXTURE0001");

    let expired_badge = credential(credentials, "My first badge");
    assert_eq!(expired_badge.data["validUntil"], "2026-10-01T00:00:00Z");

    let mut connections: Vec<_> = state
        .connections
        .0
        .iter()
        .map(|connection| (connection.name.as_str(), connection.did.as_str()))
        .collect();
    connections.sort();
    assert_eq!(
        connections,
        vec![
            ("Capella", "did:web:capella.dev2.impierce.com"),
            ("Mira", "did:web:mira.dev2.impierce.com"),
        ]
    );

    assert_eq!(state.history.len(), 7);

    // The keys in the Stronghold snapshot still produce the DIDs the profile was created with.
    let subject = state
        .core_utils
        .managers
        .lock()
        .await
        .identity_manager
        .as_ref()
        .unwrap()
        .subject
        .clone();
    for method in ["did:jwk", "did:key"] {
        assert_eq!(
            &subject.identifier(method, Algorithm::EdDSA).await.unwrap(),
            state.dids.get(method).unwrap(),
        );
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_profile_v0_18_0_rejects_incorrect_password() {
    let _data_dir = setup_profile_fixture(PROFILE_V0_18_0);

    let state = handle_actions(vec![get_state(), unlock_storage("incorrect")]).await;

    assert!(!state.is_unlocked);
    assert_eq!(state.current_user_prompt, Some(CurrentUserPrompt::PasswordRequired));
    assert!(state
        .debug_messages
        .iter()
        .any(|message| message.contains("Failed to load stronghold")));
}
