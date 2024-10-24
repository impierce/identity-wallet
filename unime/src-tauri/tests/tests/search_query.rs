use crate::common::assert_state_update::{assert_state_update, setup_state_file};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_credential_search_query() {
    setup_state_file();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
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
async fn test_credential_search_query_relevance_order() {
    setup_state_file();

    let state = json_example::<AppState>("tests/fixtures/states/shenron_six_credentials_two_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_letter.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/shenron_six_credentials_two_connections_search.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_add_recent_search() {
    setup_state_file();

    // Add recent search with recent search still in the current field
    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_add_recent_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_recent_search.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;

    // Add recent search with recent search not in the current field
    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_add_recent_search.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/two_credentials_recent_search_no_current.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_add_existing_recent_search_does_not_create_duplicate() {
    setup_state_file();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_recent_search.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_add_recent_search.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state.clone())),
        vec![action],
        vec![Some(state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_add_existing_recent_search_back_on_top() {
    setup_state_file();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_two_recent_searches.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_add_recent_search.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/two_credentials_existing_recent_search_back_on_top.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_delete_recent() {
    setup_state_file();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_delete_recent.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_delete_recent.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}
