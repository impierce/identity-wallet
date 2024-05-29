use crate::common::assert_state_update::{assert_state_update, setup_state_file, setup_stronghold};
use crate::common::json_example;
use identity_wallet::state::{actions::Action, AppState, AppStateContainer};
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_credentials_sorting_identical_issue_dates_reverse() {
    // This test proves there is no random behavior when sorting credentials with identical issue dates.
    // The order of the credentials will simply remain the same.
    // Yet, the order is still reversable when this is clicked by the user.

    setup_state_file();
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_sort_issue.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/credential_sort_issue_reverse.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates.json");
    let expected_state2 =
        json_example::<AppState>("tests/fixtures/states/four_credentials_sort_identical_issue_dates_reverse.json");
    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action, action2],
        vec![Some(expected_state), Some(expected_state2)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_connections_sorting_unicode_name_az_reverse() {
    setup_state_file();
    setup_stronghold();

    // Example of unicode characters, and how they are understood differently by the computer.
    // This example is also used in the json files
    // let no_unicode = "sécréter";
    // let unicode = "sécréter";
    // assert_ne!(no_unicode, unicode);
    // assert_ne!(no_unicode.len(), unicode.len());
    // The above is directly written as unicode characters.
    // In unicode sequence it is "s\u{00e9}cr\u{00e9}ter"

    let state = json_example::<AppState>("tests/fixtures/states/five_connections_unicode_random_alphabets.json");
    let action = json_example::<Action>("tests/fixtures/actions/connection_sort_name_az.json");
    let expected_state =
        json_example::<AppState>("tests/fixtures/states/five_connections_sort_unicode_random_alphabets_name_az.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}
