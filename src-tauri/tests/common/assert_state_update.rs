use identity_wallet::{
    state::{actions::Action, persistence::save_state, AppState},
    STATE_FILE, STRONGHOLD,
};
use serde_json::json;
use tauri::Manager;
use tempfile::NamedTempFile;

/// Asserts that the state is updated as expected after the given action is handled.
pub async fn assert_state_update(
    current_state: AppState,
    actions: Vec<Action>,
    expected_states: Vec<Option<AppState>>,
) {
    // Save the current state to the state file.
    save_state(&current_state).await.unwrap();

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
            tauri::window::InvokeRequest {
                cmd: "handle_action".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                body: json!({ "action": action }).into(),
                headers: Default::default(),
            },
            Ok(()),
        );

        // Assert that the state is updated as expected.
        if let Some(expected_state) = expected_state {
            let AppState {
                active_profile,
                locale,
                credentials,
                current_user_prompt,
                debug_messages,
                ..
            } = app.app_handle().state::<AppState>().inner();

            let AppState {
                active_profile: expected_active_profile,
                locale: expected_locale,
                credentials: expected_credentials,
                current_user_prompt: expected_current_user_prompt,
                debug_messages: expected_debug_messages,
                ..
            } = expected_state;

            let active_profile = active_profile.lock().unwrap().clone();
            let expected_active_profile = expected_active_profile.lock().unwrap().clone();
            match (active_profile, expected_active_profile) {
                (Some(active_profile), Some(expected_active_profile)) => {
                    assert_eq!(active_profile.name, expected_active_profile.name);
                }
                (active_profile, expected_active_profile) => assert_eq!(active_profile, expected_active_profile),
            }
            assert_eq!(*locale.lock().unwrap(), *expected_locale.lock().unwrap());
            assert_eq!(*credentials.lock().unwrap(), *expected_credentials.lock().unwrap());
            assert_eq!(
                debug_messages.lock().unwrap().len(),
                expected_debug_messages.lock().unwrap().len()
            );
            assert_eq!(
                *current_user_prompt.lock().unwrap(),
                *expected_current_user_prompt.lock().unwrap()
            );
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
