use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file, setup_stronghold},
    test_managers,
};
use crate::common::json_example;
use identity_wallet::state::reducers::load_dev_profile::{DRIVERS_LICENSE_CREDENTIAL, PERSONAL_INFORMATION};
use identity_wallet::{
    state::{
        actions::{Action, ActionType},
        user_prompt::{
            AcceptConnection, CredentialOffer as CredentialOfferPrompt, CurrentUserPrompt, CurrentUserPromptType,
            Redirect, ShareCredentials,
        },
        AppState, Profile, TransferState,
    },
    verifiable_credential_record::VerifiableCredentialRecord,
};
use oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::{self, JwtVcJson};
use oid4vci::credential_format_profiles::{Credential, Parameters, WithCredential, WithParameters};
use oid4vci::credential_offer::{Grants, PreAuthorizedCode};
use oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_offer::{CredentialOffer, CredentialsObject},
};
use serde_json::json;
use std::sync::Mutex;


#[tokio::test]
#[serial_test::serial]
async fn pretty_print_test() {

    let verifiable_credential_record = VerifiableCredentialRecord::from(CredentialFormats::<WithCredential>::JwtVcJson(Credential {
        format: JwtVcJson,
        credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RqWXpmNkd1UVJraDFYczlHcUJIU3JKVU01S3VxcGNKMXVjV0E3cmdINXBoI3o2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.ETqRaVMxFZQLN8OmngL1IPGAA2xH9Nsir9vRvJTLLBOJbnGuPdvcMQkN720MQuk9LWmsqNMBrUQegIuJ9IQLBg")
    }));

    let credentials = vec![verifiable_credential_record.display_credential.clone()];

    let uuid = verifiable_credential_record.display_credential.id.clone();

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

    let test = TransferState {
        active_profile,
        credentials,
        current_user_prompt: Some(CurrentUserPrompt::Redirect(Redirect {
            r#type: CurrentUserPromptType::Redirect,
            target: "me".to_string(),
        })),
        ..TransferState::default()
    };
    

    let s = serde_json::to_string_pretty(&test).unwrap();
    println!("{}", s);
    let ds = serde_json::from_str::<TransferState>(&s).unwrap();
    println!("{:#?}", ds);
    let dss = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/handle_oid4vp_2.json");
    println!("{:#?}", dss);
}

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

//    let credential_issuer: url::Url = "http://192.168.1.127:9090".parse().unwrap();

    let state = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/read_cred.json");
    let action = json_example::<Action>("tests/tests/fixtures-qr_code_scanned/actions/read_cred.json");
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            ..AppState::default()
        },
        // A QR code is scanned containing a credential offer.
        vec![action],
        // The state is updated with a new user prompt containing the credential offer.
        vec![Some(state)],
    ).await;
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

    let state1 = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/handle_siopv2_1.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/handle_siopv2_2.json");
    let action1 = json_example::<Action>("tests/tests/fixtures-qr_code_scanned/actions/handle_siopv2_1.json");
    let action2 = json_example::<Action>("tests/tests/fixtures-qr_code_scanned/actions/handle_siopv2_2.json");
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            ..AppState::default()
        },
        // A QR code was scanned containing a SIOPv2 authorization request.
        vec![
            action1,
            action2
        ],
        // The state is updated with a new user prompt containing the client's metadata.
        vec![
            Some(state1),
            Some(state2),
        ],
    ).await;
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

//    let uuid = verifiable_credential_record.display_credential.id.clone();

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

    let state1 = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/handle_oid4vp_1.json");
    let state2 = json_example::<TransferState>("tests/tests/fixtures-qr_code_scanned/states/handle_oid4vp_2.json");
    let action1 = json_example::<Action>("tests/tests/fixtures-qr_code_scanned/actions/handle_oid4vp_1.json");
    let action2 = json_example::<Action>("tests/tests/fixtures-qr_code_scanned/actions/handle_oid4vp_2.json");
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            credentials: Mutex::new(credentials.clone()),
            ..AppState::default()
        },
        // A QR code was scanned containing a OID4VP authorization request.
        vec![
            action1,
            action2,
        ],
        // The state is updated with a new user prompt containing the uuid's of the candidate verifiable credentials.
        vec![
            Some(state1),
            Some(state2),
        ],
    ).await;
}
