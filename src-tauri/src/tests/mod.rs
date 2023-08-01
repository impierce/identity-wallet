mod load_dev_profile;

use crate::state::{actions::Action, AppState, TransferState};
use serde_json::json;
use tauri::Manager;

/// Asserts that the state is updated as expected after the given action is handled.
pub fn assert_state_update(current_state: AppState, action: Action, expected_state: TransferState) {
    // Initialize the app with the given state and action handler.
    let app = tauri::test::mock_builder()
        .manage(current_state)
        .invoke_handler(tauri::generate_handler![crate::command::handle_action])
        .build(tauri::generate_context!())
        .unwrap();

    let window = tauri::WindowBuilder::new(&app, "main", Default::default())
        .build()
        .unwrap();

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
    assert_eq!(
        TransferState::from(app.app_handle().state::<AppState>().inner()),
        expected_state,
    );
}
