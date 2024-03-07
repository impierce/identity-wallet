use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::{json_example, test_managers};
use identity_wallet::state::profile_settings::AppTheme;
use identity_wallet::state::{
    actions::Action,
    core_utils::CoreUtils,
    profile_settings::{Profile, ProfileSettings},
    AppState, AppStateContainer,
};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_create_new() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the AppStates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/no_profile_redirect_welcome.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/create_new.json");

    assert_state_update(
        // Initial state.
        AppStateContainer::default(),
        vec![
            // Get the initial state.
            action1, // Create a new profile.
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

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/password_required.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/unlock_storage.json");

    let container = AppStateContainer(Mutex::new(AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]),
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some(AppTheme::System),
                primary_did: "did:example:placeholder".to_string(),
            }),
            ..Default::default()
        },
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        vec![
            // Get the initial state.
            action1, // Unlock the storage.
            action2,
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

#[tokio::test]
#[serial_test::serial]
async fn test_get_state_unlock_storage_invalid_password() {
    setup_state_file();
    setup_stronghold();

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/password_required.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/password_required_incorrect_password_error.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/get_state.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/unlock_storage_incorrect_password.json");

    let container = AppStateContainer(Mutex::new(AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]),
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: Some(Profile {
                name: "Ferris Crabman".to_string(),
                picture: Some("&#129408".to_string()),
                theme: Some(AppTheme::System),
                primary_did: "did:example:placeholder".to_string(),
            }),
            ..Default::default()
        },
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        vec![
            // Get the initial state.
            action1, // Unlock the storage.
            action2,
        ],
        vec![
            // The storage is locked, so the user is prompted to unlock it.
            Some(state1),
            // An incorrect password error is added to the state.
            Some(state2),
        ],
    )
    .await;
}
