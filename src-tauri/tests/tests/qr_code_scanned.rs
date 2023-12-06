use crate::common::json_example;
use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file, setup_stronghold},
    test_managers,
};
use identity_wallet::state::AppStateContainer;
use identity_wallet::{
    state::{actions::Action, AppState, Profile},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::JwtVcJson;
use oid4vci::credential_format_profiles::CredentialFormats;
use oid4vci::credential_format_profiles::{Credential, WithCredential};
use serde_json::json;
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_read_credential_offer() {
    setup_state_file();
    setup_stronghold();

    let managers = test_managers(vec![]);
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: managers
            .lock()
            .await
            .identity_manager
            .as_ref()
            .unwrap()
            .subject
            .identifier()
            .unwrap(),
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/credential_offer.json");
    let action = json_example::<Action>("tests/fixtures/actions/qr_scanned_openid_cred.json");

    let container = AppStateContainer(Mutex::new(AppState {
        active_profile: active_profile.clone(),
        managers,
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        // A QR code is scanned containing a credential offer.
        vec![action],
        // The state is updated with a new user prompt containing the credential offer.
        vec![Some(state)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_handle_siopv2_authorization_request() {
    setup_state_file();
    setup_stronghold();

    let managers = test_managers(vec![]);
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: managers
            .lock()
            .await
            .identity_manager
            .as_ref()
            .unwrap()
            .subject
            .identifier()
            .unwrap(),
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/accept_connection.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/did_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_id_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_connect_accept.json");

    let container = AppStateContainer(Mutex::new(AppState {
        active_profile: active_profile.clone(),
        managers,
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        // A QR code was scanned containing a SIOPv2 authorization request.
        vec![action1, action2],
        // The state is updated with a new user prompt containing the client's metadata.
        vec![Some(state1), Some(state2)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_handle_oid4vp_authorization_request() {
    setup_state_file();
    setup_stronghold();

    let verifiable_credential_record = VerifiableCredentialRecord::from(CredentialFormats::<WithCredential>::JwtVcJson(Credential {
        format: JwtVcJson,
        credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RqWXpmNkd1UVJraDFYczlHcUJIU3JKVU01S3VxcGNKMXVjV0E3cmdINXBoI3o2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.ETqRaVMxFZQLN8OmngL1IPGAA2xH9Nsir9vRvJTLLBOJbnGuPdvcMQkN720MQuk9LWmsqNMBrUQegIuJ9IQLBg")
    }));

    let credentials = vec![verifiable_credential_record.display_credential.clone()];

    let managers = test_managers(vec![verifiable_credential_record]);
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: managers
            .lock()
            .await
            .identity_manager
            .as_ref()
            .unwrap()
            .subject
            .identifier()
            .unwrap(),
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/credential_share_credential.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/credential_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_vp_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_cred_selected.json");

    let container = AppStateContainer(Mutex::new(AppState {
        active_profile: active_profile.clone(),
        managers,
        credentials: credentials.clone(),
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        // A QR code was scanned containing a OID4VP authorization request.
        vec![action1, action2],
        // The state is updated with a new user prompt containing the uuid's of the candidate verifiable credentials.
        vec![Some(state1), Some(state2)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_invalid_qr_code_error() {
    setup_state_file();
    setup_stronghold();

    let managers = test_managers(vec![]);
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: managers
            .lock()
            .await
            .identity_manager
            .as_ref()
            .unwrap()
            .subject
            .identifier()
            .unwrap(),
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/invalid_payload_error.json");
    let action = json_example::<Action>("tests/fixtures/actions/qr_scanned_invalid_payload.json");

    let container = AppStateContainer(Mutex::new(AppState {
        active_profile: active_profile.clone(),
        managers,
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        // A QR code is scanned containing an invalid payload.
        vec![action],
        // An invalid payload error is added to the state.
        vec![Some(state)],
    )
    .await;
}
