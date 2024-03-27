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
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/shenron_six_credentials_two_connections.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_search_letter.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/shenron_six_credentials_two_connections_search.json");

    assert_state_update(
        AppStateContainer(Mutex::new(state)),
        vec![action],
        vec![Some(expected_state)],
    )
    .await;
}

// Results for search term "b"

// "32353832-3634-6530-3635-663766373865", = Name: "Volunteer Badge", Issuer: "NGDIL"
// "65343639-6535-6236-3939-313463653461", = Name: "Business Innovation & Interdisciplinair Samenwerken", = Issuer: "Koning Willem I College"
// "30306664-6633-3766-3063-393735663837", = Name: "Future Engineer Certificate", Issuer: "Koning Willem I College"
// "37323764-3935-3531-3636-386334326265", = Name: "School Course Certificate", Issuer: "NGDIL"
// "39313132-3661-6238-3462-393936663735", = Name: "Higher Education Information Literacy Level 1", Issuer: "NGDIL"
// "65313633-6666-3135-6464-636630373861"  = Name: "National ID", Issuer: "NGDIL"

// Results for search term "d"

// "32353832-3634-6530-3635-663766373865", = Name: Volunteer Badge, issuer: NGDIL
// "39313132-3661-6238-3462-393936663735", = Name: Higher Education Information Literacy Level 1, issuer: NGDIL
// "65313633-6666-3135-6464-636630373861", = Name: National ID, issuer: NGDIL
// "65343639-6535-6236-3939-313463653461", = Name: Business Innovation & Interdisciplinair Samenwerken, issuer: NGDIL
// "37323764-3935-3531-3636-386334326265", = Name: School Course Certificate, issuer: NGDIL
// "30306664-6633-3766-3063-393735663837"  = Name: Future Engineer Certificate, issuer: Koning Willem I College


#[tokio::test]
#[serial_test::serial]
async fn test_credential_add_recent_search() {
    setup_state_file();
    setup_stronghold();

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
    setup_stronghold();

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
    setup_stronghold();

    let state = json_example::<AppState>("tests/fixtures/states/two_credentials_two_recent_searches.json");
    let action = json_example::<Action>("tests/fixtures/actions/credential_add_recent_search.json");
    let expected_state = json_example::<AppState>("tests/fixtures/states/two_credentials_existing_recent_search_back_on_top.json");

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
    setup_stronghold();

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

#[test]
fn tets() {
    let v = vec![1, 2, 3];
    let panics = std::panic::catch_unwind(|| v[99]).is_err();
    assert!(panics);
    println!("Hello, World");
}
