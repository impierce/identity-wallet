use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_credential_search_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_sort_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_sort_query.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_search_sort_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_sort.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_sort_query.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_search_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_search.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sort_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_sort.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_search_sort_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_search_sort.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_search_sort.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}
