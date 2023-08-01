mod load_dev_profile;
mod qr_code_scanned;

use std::sync::Arc;

use crate::{
    crypto::stronghold::create_new_stronghold,
    state::{actions::Action, AppState, TransferState},
    PROVIDER_MANAGER, STATE_FILE, STRONGHOLD,
};
use did_key::{generate, Ed25519KeyPair};
use oid4vc_manager::{methods::key_method::KeySubject, ProviderManager};
use serde_json::json;
use tauri::Manager;
use tempfile::NamedTempFile;

/// Asserts that the state is updated as expected after the given action is handled.
pub fn assert_state_update(current_state: AppState, actions: Vec<Action>, expected_states: Vec<Option<TransferState>>) {
    // Initialize the app with the given state and action handler.
    let app = tauri::test::mock_builder()
        .manage(current_state)
        .invoke_handler(tauri::generate_handler![crate::command::handle_action])
        .build(tauri::generate_context!())
        .unwrap();

    let window = tauri::WindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

    for (action, expected_state) in actions.iter().zip(expected_states.iter()) {
        // Assert that the action is handled successfully.
        tauri::test::assert_ipc_response(
            &window,
            tauri::InvokePayload {
                cmd: "handle_action".into(),
                callback: tauri::api::ipc::CallbackFn(0),
                error: tauri::api::ipc::CallbackFn(1),
                inner: json!({ "action": action }),
            },
            Ok(()),
        );

        // Assert that the state is updated as expected.
        if let Some(expected_state) = expected_state {
            assert_eq!(
                &TransferState::from(app.app_handle().state::<AppState>().inner()),
                expected_state,
            );
        }
    }
}

pub fn setup_state_file() {
    let path = NamedTempFile::new().unwrap().into_temp_path();
    *STATE_FILE.lock().unwrap() = path.as_os_str().into();
}

pub async fn setup_stronghold() {
    let path = NamedTempFile::new().unwrap().into_temp_path();
    *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
    create_new_stronghold("my-password").await.unwrap();
}

pub async fn setup_provider_manager() {
    let subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(
        "this-is-a-very-UNSAFE-secret-key".as_bytes(),
    )));
    let provider_manager = ProviderManager::new([Arc::new(subject)]).unwrap();
    PROVIDER_MANAGER.lock().await.replace(provider_manager);
}
