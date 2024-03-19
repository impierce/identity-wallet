use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_credential_search_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state.clone())),
        vec![action.clone()],
        vec![Some(expected_state.clone())],
    )
    .await;

    // To check wether searching the same search_term isn't registered twice.
    assert_state_update(
        AppStateContainer(Mutex::new(expected_state.clone())),
        vec![action],
        vec![Some(expected_state.clone())],
    )
    .await;

    // Checking an extra search_term
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_similar.json");
    let expected_state_2 = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query_similar.json");
    assert_state_update(
        AppStateContainer(Mutex::new(expected_state.clone())),
        vec![action],
        vec![Some(expected_state_2)],
    )
    .await;

    // Checking deleting recent action
    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_delete_recent.json");
    let expected_state_2 = json_example::<AppState>("tests/fixtures/states/two_credentials_search_delete_recent.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state_2)],
    )
    .await;
}

// Sorting testing will be moved elsewhere in future PR

// #[tokio::test]
// #[serial_test::serial]
// async fn test_credential_sort_query() {
//     setup_state_file();
//     setup_stronghold();

//     let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
//     let action = json_example::<Action>("tests/fixtures/actions/credential_sort.json");
//     let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_sort_query.json");
//     assert_state_update(
//         AppStateContainer(Mutex::new(state)),
//         vec![action],
//         vec![Some(expected_state)],
//     )
//     .await;
// }

// #[tokio::test]
// #[serial_test::serial]
// async fn test_credential_search_sort_query() {
//     setup_state_file();
//     setup_stronghold();

//     let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me_query.json");
//     let action = json_example::<Action>("tests/fixtures/actions/credential_search_sort.json");
//     let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_search_sort_query.json");
//     assert_state_update(
//         AppStateContainer(Mutex::new(state)),
//         vec![action],
//         vec![Some(expected_state)],
//     )
//     .await;
// }

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
        vec![action.clone()],
        vec![Some(expected_state.clone())],
    )
    .await;

    // To check wether searching the same term isn't registered twice.
    assert_state_update(
        AppStateContainer(Mutex::new(expected_state.clone())),
        vec![action],
        vec![Some(expected_state.clone())],
    )
    .await;

    // Checking a similar search_term with only a lower/upper case difference.
    // In this case the search_term won't be registered twice either,
    //  but it will be inserted back on top and in lower/upper case style of the most recent search.
    let action = json_example::<Action>("tests/fixtures/actions/connection_search_case_difference.json");
    let expected_state_2 =
        json_example::<AppState>("tests/fixtures/states/three_connections_search_case_difference.json");
    assert_state_update(
        AppStateContainer(Mutex::new(expected_state)),
        vec![action],
        vec![Some(expected_state_2)],
    )
    .await;

    // Checking deleting recent action
    let state = json_example::<AppState>("tests/fixtures/states/three_connections_search.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_search_delete_recent.json");
    let expected_state_2 = json_example::<AppState>("tests/fixtures/states/three_connections_search_delete_recent.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state_2)],
    )
    .await;
}

// Sorting testing will be moved elsewhere in future PR

// #[tokio::test]
// #[serial_test::serial]
// async fn test_connections_sort_query() {
//     setup_state_file();
//     setup_stronghold();

//     let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
//     let action = json_example::<Action>("tests/fixtures/actions/connection_sort.json");
//     let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_sort.json");
//     assert_state_update(
//         AppStateContainer(Mutex::new(state)),
//         vec![action],
//         vec![Some(expected_state)],
//     )
//     .await;
// }

// #[tokio::test]
// #[serial_test::serial]
// async fn test_connections_search_sort_query() {
//     setup_state_file();
//     setup_stronghold();

//     let state = json_example::<AppState>("tests/fixtures/states/three_connections.json");
//     let action = json_example::<Action>("tests/fixtures/actions/connection_search_sort.json");
//     let expected_state = json_example::<AppState>("tests/fixtures/states/three_connections_search_sort.json");
//     assert_state_update(
//         AppStateContainer(Mutex::new(state)),
//         vec![action],
//         vec![Some(expected_state)],
//     )
//     .await;
// }
