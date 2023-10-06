use std::sync::Mutex;
use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::{test_managers, json_example};
use identity_wallet::state::Profile;
use identity_wallet::state::{
    actions::Action,
    AppState, TransferState,
};


#[tokio::test]
#[serial_test::serial]
async fn test_get_state_create_new() {
    setup_state_file();
    setup_stronghold();

    let state1 = json_example::<TransferState>("tests/tests/fixtures-get_state/states/create_new1.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures-get_state/states/create_new2.json");
    let action1 = json_example::<Action>("tests/tests/fixtures-get_state/actions/create_new1.json");
    let action2 = json_example::<Action>("tests/tests/fixtures-get_state/actions/create_new2.json");
    assert_state_update(
        // Initial state.
        AppState::default(),
        vec![
            // Get the initial state.
            action1,
            // Create a new profile.
            action2,
        ],
        vec![
            // There is no profile yet, so the user is redirected to the welcome page.
            Some(state1),
            // The profile was created, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_unlock_storage() {
    setup_state_file();
    setup_stronghold();

    
//    let appstate = json_example::<AppState>("tests/tests/fixtures-get_state/states/unlock_storage-app_state.json");
    let state1 = json_example::<TransferState>("tests/tests/fixtures-get_state/states/unlock_storage1.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures-get_state/states/unlock_storage2.json");
    let action1 = json_example::<Action>("tests/tests/fixtures-get_state/actions/unlock_storage1.json");
    let action2 = json_example::<Action>("tests/tests/fixtures-get_state/actions/unlock_storage2.json");
    assert_state_update(
        // Initial state.
        AppState {
            managers: test_managers(vec![]),
            active_profile: Mutex::new(Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some("system".to_string()),
                primary_did: "did:example:placeholder".to_string(),
            })),
            ..AppState::default()
        },
        vec![
            // Get the initial state.
            action1,
            // Unlock the storage.
            action2
        ],
        vec![
            // The storage is locked, so the user is prompted to unlock it.
            Some(state1),
            // The storage is unlocked, so the user is redirected to the profile page.
            Some(state2),
        ],
    )
    .await;
}
