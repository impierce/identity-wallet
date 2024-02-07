use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState};
use identity_wallet::state::{AppStateContainer, CustomExtension};

#[tokio::test]
#[serial_test::serial]
async fn test_extension() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the AppStates and Actions from the accompanying json files.
    let state = AppStateContainer::default().add_extension("test", Box::new(CustomExtension{ name: "test".to_string(), value: "test".to_string() })).await;
    let state2 = json_example::<AppState>("tests/fixtures/states/redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/test_extension.json");

    dbg!(&state);
    dbg!(&action1);

    let guard = state.0.lock().await;

    let appstate = AppState {
        managers: guard.managers.clone(),
        active_profile: guard.active_profile.clone(),
        active_connection_request: serde_json::from_value(serde_json::json!(guard.active_connection_request))
            .unwrap(),
        locale: guard.locale.clone(),
        credentials: guard.credentials.clone(),
        current_user_prompt: guard.current_user_prompt.clone(),
        debug_messages: guard.debug_messages.clone(),
        user_journey: guard.user_journey.clone(),
        connections: guard.connections.clone(),
        user_data_query: guard.user_data_query.clone(),
        dev_mode_enabled: guard.dev_mode_enabled,
        extensions: guard.extensions.clone(),
    };

    drop(guard);

    dbg!(serde_json::to_string_pretty(&appstate).unwrap());
    assert_state_update(
        // Initial state.
        state,
        vec![
            // Test action
            action1,
        ],
        vec![
            // state including CustomExtension
            Some(state2),
        ],
    )
    .await;
}
