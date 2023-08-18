use crate::common::{
    assert_state_update::{assert_state_update, setup_provider_manager, setup_state_file, setup_stronghold},
    TEST_PASSWORD,
};
use identity_wallet::{
    crypto::stronghold::StrongholdManager,
    state::{
        actions::{Action, ActionType},
        user_prompt::{CredentialOffer as CredentialOfferPrompt, CurrentUserPrompt, CurrentUserPromptType, Selection},
        AppState, Managers, Profile, TransferState,
    },
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
use uuid::Uuid;

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_read_credential_offer() {
    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            })),
            ..AppState::default()
        },
        // A QR code is scanned containing a credential offer.
        vec![Action {
            r#type: ActionType::QrCodeScanned,
            payload: Some(json!({
                "form_urlencoded": "openid-credential-offer://?credential_offer=%7B%22credential_issuer%22%3A%22http%3A%2F%2F192.168.1.127%3A9090%2F%22%2C%22credentials%22%3A%5B%7B%22format%22%3A%22jwt_vc_json%22%2C%22credential_definition%22%3A%7B%22type%22%3A%5B%22VerifiableCredential%22%2C%22PersonalInformation%22%5D%7D%7D%5D%2C%22grants%22%3A%7B%22urn%3Aietf%3Aparams%3Aoauth%3Agrant-type%3Apre-authorized_code%22%3A%7B%22pre-authorized_code%22%3A%220YI5DXtuCltKyNa5%22%2C%22user_pin_required%22%3Afalse%7D%7D%7D"
            })),
        }],
        // The state is updated with a new user prompt containing the credential offer.
        vec![Some(TransferState {
            active_profile: Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            current_user_prompt: Some(CurrentUserPrompt::CredentialOffer(CredentialOfferPrompt {
                r#type: CurrentUserPromptType::CredentialOffer,
                credential_offer: serde_json::to_value(&CredentialOffer {
                    credential_issuer: "http://192.168.1.127:9090".parse().unwrap(),
                    credentials: vec![CredentialsObject::ByValue(
                        CredentialFormats::<WithParameters>::JwtVcJson(Parameters {
                            format: JwtVcJson,
                            parameters: (
                                jwt_vc_json::CredentialDefinition {
                                    type_: vec!["VerifiableCredential".to_string(), "PersonalInformation".to_string()],
                                    credential_subject: None,
                                },
                                None,
                            )
                                .into(),
                        }),
                    )],
                    grants: Some(Grants {
                        authorization_code: None,
                        pre_authorized_code: Some(PreAuthorizedCode {
                            pre_authorized_code: "0YI5DXtuCltKyNa5".to_string(),
                            ..PreAuthorizedCode::default()
                        }),
                    }),
                })
                .unwrap(),
            })),
            ..TransferState::default()
        })],
    );
}

#[tokio::test]
#[serial_test::serial]
async fn test_qr_code_scanned_read_authorization_request() {
    setup_state_file();
    setup_stronghold();
    setup_provider_manager().await;

    let credential = CredentialFormats::<WithCredential>::JwtVcJson(Credential {
        format: JwtVcJson,
        credential: json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa3RqWXpmNkd1UVJraDFYczlHcUJIU3JKVU01S3VxcGNKMXVjV0E3cmdINXBoI3o2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWt0all6ZjZHdVFSa2gxWHM5R3FCSFNySlVNNUt1cXBjSjF1Y1dBN3JnSDVwaCIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.ETqRaVMxFZQLN8OmngL1IPGAA2xH9Nsir9vRvJTLLBOJbnGuPdvcMQkN720MQuk9LWmsqNMBrUQegIuJ9IQLBg")
    });

    let uuid = Uuid::new_v4();

    let stronghold_manager = StrongholdManager::create(TEST_PASSWORD).unwrap();
    stronghold_manager
        .insert(uuid, json!(credential).to_string().as_bytes().to_vec())
        .unwrap();

    assert_state_update(
        // Initial state.
        AppState {
            active_profile: Mutex::new(Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            })),
            managers: Mutex::new(Managers {
                stronghold_manager: Some(stronghold_manager),
                ..Managers::default()
            }),
            ..AppState::default()
        },
        // A QR code was scanned containing a authorization request.
        vec![Action {
            r#type: ActionType::QrCodeScanned,
            payload: Some(json!({
                "form_urlencoded": "siopv2://idtoken?response_type=id_token+vp_token&client_id=did%3Akey%3Az6Mkm9yeuZK7inXBNjnNH3vAs9uUjqfy3mfNoKBKsKBrv8Tb&scope=openid&presentation_definition=%7B%22id%22%3A%22Verifiable+Presentation+request+for+sign-on%22%2C%22input_descriptors%22%3A%5B%7B%22id%22%3A%22Request+for+Ferris%27s+Verifiable+Credential%22%2C%22constraints%22%3A%7B%22fields%22%3A%5B%7B%22path%22%3A%5B%22%24.vc.type%22%5D%2C%22filter%22%3A%7B%22type%22%3A%22array%22%2C%22contains%22%3A%7B%22const%22%3A%22PersonalInformation%22%7D%7D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.givenName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.familyName%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.email%22%5D%7D%2C%7B%22path%22%3A%5B%22%24.vc.credentialSubject.birthdate%22%5D%7D%5D%7D%7D%5D%7D&redirect_uri=https%3A%2F%2Fexample.com&nonce=nonce"
            })),
        }],
        // The state is updated with a new user prompt containing the uuid's of the candidate verifiable credentials.
        vec![Some(TransferState {
            active_profile: Some(Profile {
                display_name: "Ferris Crabman".to_string(),
                primary_did: "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY".to_string(),
            }),
            current_user_prompt: Some(CurrentUserPrompt::Selection(Selection {
                r#type: CurrentUserPromptType::SelectCredentials,
                options: vec![uuid.to_string()],
            })),
            ..TransferState::default()
        })],
    )
}
