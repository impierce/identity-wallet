use std::sync::Arc;

use identity_wallet::state::{
    actions::Action,
    core_utils::CoreUtils,
    credentials::{
        actions::self_issue_credential::{SelfIssueCredential, SelfIssuedCredentialType},
        reducers::self_issue_credential::self_issue_credential,
    },
    profile_settings::{Profile, ProfileSettings},
    AppState,
};
use jsonwebtoken::Algorithm;

use crate::common::{json_example, test_managers};

#[tokio::test]
#[serial_test::serial]
async fn test() {
    // Set up AppState
    let mut app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]).await,
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: Some(Profile {
                name: "John Doe".to_string(),
                ..Default::default()
            }),
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

    // let action = json_example::<Action>("tests/fixtures/actions/self_issue_profile.json");
    let action2 = Arc::new(SelfIssueCredential {
        _type: SelfIssuedCredentialType::Profile,
        data: "{ \"first name\": \"John\", \"last name\": \"Doe\" }".to_string(),
    });

    let result = self_issue_credential(app_state, action2).await.unwrap();

    // Assert AppState
    assert!(!result.credentials.is_empty());

    // Assert Stronghold
}
