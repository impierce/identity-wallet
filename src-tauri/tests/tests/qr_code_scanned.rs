use crate::common::json_example;
use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file, setup_stronghold},
    test_managers,
};
use did_key::{generate, Ed25519KeyPair};
use identity_wallet::{
    state::{actions::Action, AppState, Profile},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use jsonwebtoken::{Algorithm, Header};
use oid4vc_core::{jwt, Subject};
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::credential_format_profiles::{Credential, CredentialFormats, Parameters, WithCredential, WithParameters};
use oid4vci::credential_offer::{CredentialOffer, CredentialsObject, Grants, PreAuthorizedCode};
use oid4vci::{
    credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::{self, JwtVcJson},
    VerifiableCredentialJwt,
};
use serde_json::json;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[tokio::test]
#[serial_test::serial]
async fn test() {
    setup_state_file();
    setup_stronghold();

    // Create a new issuer.
    let issuer = KeySubject::from_keypair(
        generate::<Ed25519KeyPair>(Some(
            "this-is-a-very-UNSAFE-issuer-secret-key".as_bytes().try_into().unwrap(),
        )),
        None,
    );
    let issuer_did = issuer.identifier().unwrap();

    let mut managers = test_managers(vec![]);
    let primary_did = managers
        .lock()
        .await
        .identity_manager
        .as_ref()
        .unwrap()
        .subject
        .identifier()
        .unwrap();

    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: Some("system".to_string()),
        primary_did: primary_did.clone(),
    });

    // Create a new verifiable credential.
    let verifiable_credential = VerifiableCredentialJwt::builder()
        .sub(&primary_did)
        .iss(&issuer_did)
        .iat(0)
        .exp(9999999999i64)
        .verifiable_credential(serde_json::json!({
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://www.w3.org/2018/credentials/examples/v1"
            ],
            "type": [
                "VerifiableCredential",
                "PersonalInformation"
            ],
            "issuanceDate": "2022-01-01T00:00:00Z",
            "issuer": issuer_did,
            "credentialSubject": {
            "id": primary_did,
            "givenName": "Ferris",
            "familyName": "Crabman",
            "email": "ferris.crabman@crabmail.com",
            "birthdate": "1985-05-21"
            }
        }))
        .build()
        .unwrap();

    let jwt = jwt::encode(
        Arc::new(issuer),
        Header {
            alg: Algorithm::EdDSA,
            ..Default::default()
        },
        &verifiable_credential,
    )
    .unwrap();

    let verifiable_credential_record =
        VerifiableCredentialRecord::from(CredentialFormats::<WithCredential>::JwtVcJson(Credential {
            format: JwtVcJson,
            credential: json!(jwt),
        }));

    let key: Uuid = verifiable_credential_record
        .display_credential
        .id
        .parse()
        .expect("invalid uuid");

    managers
        .lock()
        .await
        .stronghold_manager
        .as_mut()
        .unwrap()
        .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
        .unwrap();

    assert_state_update(
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            ..AppState::default()
        },
        vec![Action {
            r#type: ActionType::QrCodeScanned,
            payload: Some(json!({
                "form_urlencoded": "siopv2://idtoken?response_type=vp_token&client_id=did%3Akey%3Az6MksT1XVJXVo8CTpiq9Qit67bP8bSpFBmSKHLTsbXyCAn7T&redirect_uri=http%3A%2F%2F0.0.0.0%3A3000%2Fsiopv2%2Fauth&presentation_definition=%7B%22id%22%3A%22Verifiable+Presentation+request+for+sign-on%22%2C%22input_descriptors%22%3A%5B%7B%22id%22%3A%22Request+for+Ferris%27s+Verifiable+Credential%22%2C%22constraints%22%3A%7B%22fields%22%3A%5B%7B%22path%22%3A%5B%22%24.vc.type%22%5D%2C%22filter%22%3A%7B%22type%22%3A%22array%22%2C%22contains%22%3A%7B%22const%22%3A%22PersonalInformation%22%7D%7D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.givenName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.familyName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.email%22%5D%7D%5D%7D%7D%5D%7D&response_mode=direct_post&nonce=n-0S6_WzA2Mj&client_metadata=%7B%22subject_syntax_types_supported%22%3A%5B%22did%3Akey%22%5D%2C%22id_token_signing_alg_values_supported%22%3A%5B%22EdDSA%22%5D%7D",
            })),
        }],
        vec![Some(TransferState::default())],
    ).await;
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

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/pf_credential_offer.json");
    let action = json_example::<Action>("tests/fixtures/actions/qr_scanned_openid_cred.json");
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

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/pf_accept_connect.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/pf_with_did_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_id_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_connect_accept.json");
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            ..AppState::default()
        },
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

    // Deserializing the Transferstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/pf_with_person_info_share_cred.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/pf_with_person_info_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_vp_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_cred_selected.json");
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(active_profile.clone()),
            managers,
            credentials: Mutex::new(credentials.clone()),
            ..AppState::default()
        },
        // A QR code was scanned containing a OID4VP authorization request.
        vec![action1, action2],
        // The state is updated with a new user prompt containing the uuid's of the candidate verifiable credentials.
        vec![Some(state1), Some(state2)],
    )
    .await;
}
