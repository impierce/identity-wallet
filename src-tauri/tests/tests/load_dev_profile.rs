use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState};

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/dev_load_profile.json");
    assert_state_update(AppState::default(), vec![action], vec![Some(state)]).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_load_dev_profile_twice() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/dev_load_profile.json");
    assert_state_update(
        AppState::default(),
        vec![
            // Load the profile twice.
            action.clone(),
            action,
        ],
        vec![
            // Each time the dev profile is loaded, the state should be the same.
            Some(state1),
            Some(state2),
        ],
    )
    .await;
}
