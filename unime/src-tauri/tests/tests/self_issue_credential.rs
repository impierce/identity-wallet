use std::sync::Arc;

use identity_wallet::state::{
    core_utils::CoreUtils,
    credentials::{
        actions::self_issue_credential::{SelfIssueCredential, SelfIssuedCredentialType},
        reducers::self_issue_credential::self_issue_credential,
    },
    AppState,
};
use jsonwebtoken::Algorithm;
use serde_json::json;

use crate::common::test_managers;

#[tokio::test]
#[serial_test::serial]
async fn test() {
    // Set up AppState
    let mut app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]).await,
            ..Default::default()
        },
        ..AppState::default()
    };

    // Set up did_jwk
    let managers = app_state.core_utils.managers.lock().await;
    let subject = &managers.identity_manager.as_ref().unwrap().subject;
    let did_jwk = subject.identifier("did:jwk", Algorithm::EdDSA).await.unwrap();

    app_state.dids.insert("did:jwk".to_string(), did_jwk);

    drop(managers);

    let action = Arc::new(SelfIssueCredential {
        _type: SelfIssuedCredentialType::Profile,
        data: json!({
            "test": 123,
            "tester": 456
        }),
    });

    let result = self_issue_credential(app_state, action).await.unwrap();
    println!("\n\nResult:\n{:?}\n", result);

    // Assert AppState
    assert!(!result.credentials.is_empty());

    // Assert Stronghold
}
