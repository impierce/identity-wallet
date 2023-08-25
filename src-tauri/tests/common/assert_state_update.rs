use identity_wallet::{
    state::{actions::Action, persistence::save_state, AppState, TransferState},
    STATE_FILE, STRONGHOLD,
};
use serde_json::json;
use tauri::Manager;
use tempfile::NamedTempFile;

/// Asserts that the state is updated as expected after the given action is handled.
pub async fn assert_state_update(
    current_state: AppState,
    actions: Vec<Action>,
    expected_states: Vec<Option<TransferState>>,
) {
    // Save the current state to the state file.
    save_state(TransferState::from(&current_state)).await.unwrap();

    // Initialize the app with the given state and action handler.
    let app = tauri::test::mock_builder()
        .manage(current_state)
        .invoke_handler(tauri::generate_handler![identity_wallet::command::handle_action])
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
            let TransferState {
                active_profile,
                locale,
                credentials,
                current_user_prompt,
                ..
            } = TransferState::from(app.app_handle().state::<AppState>().inner());

            let TransferState {
                active_profile: expected_active_profile,
                locale: expected_locale,
                credentials: expected_credentials,
                current_user_prompt: expected_current_user_prompt,
                ..
            } = expected_state;

            match (active_profile.as_ref(), expected_active_profile.as_ref()) {
                (Some(active_profile), Some(expected_active_profile)) => {
                    assert_eq!(active_profile.display_name, expected_active_profile.display_name);
                }
                _ => assert_eq!(active_profile, *expected_active_profile),
            }
            assert_eq!(locale, *expected_locale);
            assert_eq!(credentials, *expected_credentials);
            assert_eq!(current_user_prompt, *expected_current_user_prompt);
        }
    }
}

pub fn setup_state_file() {
    let path = NamedTempFile::new().unwrap().into_temp_path();
    *STATE_FILE.lock().unwrap() = path.as_os_str().into();
}

pub fn setup_stronghold() {
    let path = NamedTempFile::new().unwrap().into_temp_path();
    *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
}
