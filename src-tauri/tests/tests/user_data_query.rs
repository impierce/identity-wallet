use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState};

#[tokio::test]
#[serial_test::serial]
async fn test_credential_search_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let query = json_example::<AppState>("tests/fixtures/states/two_credentials_query.json");
    assert_state_update(state, vec![action], vec![Some(query)]).await;

}

#[tokio::test]
#[serial_test::serial]
async fn test_credential_sort_query() {
    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_redirect_me.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search.json");
    let query = json_example::<AppState>("tests/fixtures/states/two_credentials_search_query.json");
    assert_state_update(state, vec![action], vec![Some(query)]).await;
}

// There should also be a test checking the search & sort function on the connections.
// But first the connection handling needs to be finished for this.
