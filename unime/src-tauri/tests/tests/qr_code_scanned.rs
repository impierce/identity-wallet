use crate::common::{
    assert_state_update::{assert_state_update, setup_state_file},
    test_managers,
};
use crate::common::{json_example, json_example_with};
use identity_wallet::state::profile_settings::AppTheme;
use identity_wallet::state::{
    actions::Action,
    core_utils::CoreUtils,
    credentials::VerifiableCredentialRecord,
    profile_settings::{Profile, ProfileSettings},
    AppState, AppStateContainer,
};

use identity_wallet::oid4vc_core::Subject as _;
use jsonwebtoken::Algorithm;
use oid4vci::credential_format_profiles::CredentialFormats;
use serde_json::json;
use tokio::sync::Mutex;

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_handle_siopv2_authorization_request() {
    setup_state_file();

    let managers = test_managers(vec![]).await;
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: AppTheme::System,
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 = json_example::<AppState>("tests/fixtures/states/accept_connection.json");
    let state2 = json_example::<AppState>("tests/fixtures/states/did_redirect_me.json");
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_id_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_connect_accept.json");

    let container = AppStateContainer(Mutex::new(AppState {
        core_utils: CoreUtils {
            managers,
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: active_profile.clone(),
            ..Default::default()
        },
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

    // Wallet keys are generated for every test run, so the credential is bound to this wallet's DID here.
    let managers = test_managers(vec![]).await;
    let holder_did = {
        let managers = managers.lock().await;
        managers
            .identity_manager
            .as_ref()
            .unwrap()
            .subject
            .identifier("did:key", Algorithm::EdDSA)
            .await
            .unwrap()
    };

    let mut verifiable_credential_record = VerifiableCredentialRecord::try_new(
        CredentialFormats::JwtVcJson(()),
        json!(bound_to_holder(
            "eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RqWXpmNkd1UVJraDFYczlHcUJIU3JKVU01S3VxcGNKMXVjV0E3cmdINXBoI3o2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.ETqRaVMxFZQLN8OmngL1IPGAA2xH9Nsir9vRvJTLLBOJbnGuPdvcMQkN720MQuk9LWmsqNMBrUQegIuJ9IQLBg",
            "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
            &holder_did,
        )),
        vec![]
    ).unwrap();
    verifiable_credential_record.display_credential.display_name = "PersonalInformation".to_string();
    let credential_id = verifiable_credential_record.display_credential.id.clone();

    managers
        .lock()
        .await
        .stronghold_manager
        .as_ref()
        .unwrap()
        .insert(
            credential_id.parse().unwrap(),
            json!(verifiable_credential_record).to_string().as_bytes().to_vec(),
        )
        .unwrap();

    let credentials = vec![verifiable_credential_record.display_credential.clone()];
    let replacements = [
        ("HOLDER_DID", holder_did.as_str()),
        ("CREDENTIAL_ID", credential_id.as_str()),
    ];
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: AppTheme::System,
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state1 =
        json_example_with::<AppState>("tests/fixtures/states/credential_accept_connection.json", &replacements);
    let state2 = json_example_with::<AppState>("tests/fixtures/states/credential_share_credential.json", &replacements);
    let state3 = json_example_with::<AppState>("tests/fixtures/states/credential_redirect_me.json", &replacements);
    let action1 = json_example::<Action>("tests/fixtures/actions/qr_scanned_vp_token.json");
    let action2 = json_example::<Action>("tests/fixtures/actions/authenticate_connect_accept.json");
    let action3 = json_example_with::<Action>("tests/fixtures/actions/authenticate_cred_selected.json", &replacements);

    let container = AppStateContainer(Mutex::new(AppState {
        core_utils: CoreUtils {
            managers,
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: active_profile.clone(),
            ..Default::default()
        },
        credentials: credentials.clone(),
        ..AppState::default()
    }));

    assert_state_update(
        // Initial state.
        container,
        // A QR code was scanned containing a OID4VP authorization request.
        vec![action1, action2, action3],
        // The state is updated with a new user prompt containing the uuid's of the candidate verifiable credentials.
        vec![Some(state1), Some(state2), Some(state3)],
    )
    .await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_invalid_qr_code_error() {
    setup_state_file();

    let managers = test_managers(vec![]).await;
    let active_profile = Some(Profile {
        name: "Ferris".to_string(),
        picture: Some("&#129408".to_string()),
        theme: AppTheme::System,
    });

    // Deserializing the Appstates and Actions from the accompanying json files.
    let state = json_example::<AppState>("tests/fixtures/states/invalid_payload_error.json");
    let action = json_example::<Action>("tests/fixtures/actions/qr_scanned_invalid_payload.json");

    let container = AppStateContainer(Mutex::new(AppState {
        core_utils: CoreUtils {
            managers,
            ..Default::default()
        },
        profile_settings: ProfileSettings {
            profile: active_profile.clone(),
            ..Default::default()
        },
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

/// `jwt` with the holder DID `from` replaced by `to` in its claims. Its signature is no longer valid, which the wallet
/// does not check again when presenting a stored credential.
fn bound_to_holder(jwt: &str, from: &str, to: &str) -> String {
    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};

    let mut parts: Vec<String> = jwt.split('.').map(ToString::to_string).collect();
    let claims = String::from_utf8(URL_SAFE_NO_PAD.decode(&parts[1]).unwrap()).unwrap();
    parts[1] = URL_SAFE_NO_PAD.encode(claims.replace(from, to));
    parts.join(".")
}
