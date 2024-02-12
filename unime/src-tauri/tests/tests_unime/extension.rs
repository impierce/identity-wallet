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
    let state = AppStateContainer::default().insert_extension("test", Box::new(CustomExtension{ name: "test".to_string(), value: "test".to_string() })).await;
    let state2 = json_example::<AppState>("tests/fixtures/states/test_extension.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/test_extension.json");

    dbg!(&state);
    dbg!(&state2);
    dbg!(&action1);
 
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
