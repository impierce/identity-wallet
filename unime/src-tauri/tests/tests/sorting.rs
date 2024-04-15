use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

// Test: Frontend shouldn't persist reverse setting on any of the sorting options which aren't selected.
//       Because the backend doesn't persist this either.

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_name_az_reverse() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_name_az_reverse.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_name_az_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_added_settings() {
    // This test only checks if the settings are updated correctly.
    // Currently we #[PartialEq(ignore)] the metadata.date_added field since it will fail many other tests.
    // The json files are static but some tests create new credentials and the date_added field will be added at this time.
    // Simultaneously, this and the following tests using "four_credentials_*.json" fixtures will show that sorting for both connections and credentials are checked at every update.

    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_added.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_added.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_issue() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_issue.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_issue.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_issue_reverse() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_issue_reverse.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_issue_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_identical_issue_dates() {
    // This tests proves there is no random behavior when sorting credentials with identical issue dates.
    // The order of the credentials will simply remain the same.

    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_issue.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_identical_issue_dates_reverse() {
    // This tests proves again there is no random behavior when sorting credentials with identical issue dates.
    // But the order is still reversable when this is clicked by the user.

    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_issue_reverse.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

// test initiate defaults

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_name_az_reverse() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_name_az_reverse.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_sort_name_az_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_first_interact() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_first_interact.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_sort_first_interact.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_first_interact_reverse() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_first_interact_reverse.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/three_connections_sort_first_interact_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_last_interact() {
    setup_state_file();
    setup_stronghold();

    // In fact, the {last_interacted, reverse: true} is the same as {first_interacted, reverse:false}.
    // We should make a decision based on the UX to choose which one to use (first_interacted and last_interacted, or, first_interacted with reverse button), since having both would be duplicate.

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_last_interact.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_sort_last_interact.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_name_az_random_alphabets() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_connections_random_alphabets.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_name_az.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/four_connections_random_alphabets_name_az.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}
