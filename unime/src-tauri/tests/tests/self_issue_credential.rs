use std::sync::Arc;

use identity_wallet::state::{core_utils::CoreUtils, credentials::{actions::self_issue_credential::{SelfIssueCredential, SelfIssuedCredentialType}, reducers::self_issue_credential::self_issue_credential}, AppState};

use crate::common::test_managers;

#[tokio::test]
#[serial_test::serial]
async fn test() {

    // Set up AppState
    let app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]).await,
            ..Default::default()
        },
        ..AppState::default()
    };

    let action = Arc::new(SelfIssueCredential {
        _type: SelfIssuedCredentialType::Profile,
        data: Default::default(),
    });

    println!("{:?}", app_state.core_utils);
    let result = self_issue_credential(app_state, action).await.unwrap();
    println!("{:?}", result);
    
    // Assert AppState
    assert!(!result.credentials.is_empty());

    // Assert Stronghold
}