use identity_wallet::{
    state::{actions::Action, persistence::save_state, AppState, AppStateContainer, CustomExtension, profile::redux::Profile},
    STATE_FILE, STRONGHOLD,
};
use serde_json::json;
use tauri::Manager;
use tempfile::NamedTempFile;

/// Asserts that the state is updated as expected after the given action is handled.
pub async fn assert_state_update(
    current_state: AppStateContainer,
    actions: Vec<Action>,
    expected_states: Vec<Option<AppState>>,
) {
    {
        let current_state = current_state.0.lock().await;
        
        // Save the current state to the state file.
        save_state(&current_state).await.unwrap();
    }

    // Initialize the app with the given state and action handler.
    let app = tauri::test::mock_builder()
        .manage(current_state)
        .invoke_handler(tauri::generate_handler![unime::tauri_command::handle_action])
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
            let container = app.app_handle().state::<AppStateContainer>().inner();
            let mut guard = container.0.lock().await;

            let AppState {
                locale,
                credentials,
                current_user_prompt,
                user_data_query,
                debug_messages,
                feat_states,
                extensions,
                ..
            } = &mut *guard;

            let AppState {
                locale: expected_locale,
                credentials: expected_credentials,
                current_user_prompt: expected_current_user_prompt,
                user_data_query: expected_user_data_query,
                debug_messages: expected_debug_messages,
                feat_states: expected_feat_states,
                extensions: expected_extensions,
                ..
            } = expected_state;

            let active_profile = feat_states.get("profile"); 
            let expected_active_profile = expected_feat_states.get("profile");

            assert_eq!(active_profile.is_none(), expected_active_profile.is_none());
            if (active_profile.is_some()) && (expected_active_profile.is_some()) {
                let active_profile = active_profile.unwrap().clone().downcast::<Profile>().unwrap();
                let expected_active_profile = expected_active_profile.unwrap().clone().downcast::<Profile>().unwrap();
                assert_eq!(active_profile.name, expected_active_profile.name);
                assert_eq!(active_profile.picture, expected_active_profile.picture);
                assert_eq!(active_profile.theme, expected_active_profile.theme);
            }

            // match (active_profile, expected_active_profile) {
            //     (Some(active_profile), Some(expected_active_profile)) => {
            //         assert_eq!(active_profile.name, expected_active_profile.name);
            //     }
            //     (active_profile, expected_active_profile) => assert_eq!(active_profile, expected_active_profile),
            // }

            assert_eq!(locale, expected_locale);
            assert_eq!(credentials, expected_credentials);

            debug_messages.iter().zip(expected_debug_messages.iter()).for_each(
                |(debug_message, expected_debug_message)| {
                    assert_eq!(
                        debug_message.split_once(' ').unwrap().1,
                        expected_debug_message.split_once(' ').unwrap().1
                    );
                },
            );

            assert_eq!(debug_messages.len(), expected_debug_messages.len());
            assert_eq!(current_user_prompt, expected_current_user_prompt);
            assert_eq!(user_data_query, expected_user_data_query);
            if (extensions.len() != 0) || (expected_extensions.len() != 0) {
                assert_eq!(extensions.get("test").unwrap().clone().downcast::<CustomExtension>().unwrap(), expected_extensions.get("test").unwrap().clone().downcast::<CustomExtension>().unwrap());
            }
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
