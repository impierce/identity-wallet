use crate::common::test_managers;
use identity_wallet::oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;
use identity_wallet::oid4vci::credential_offer::CredentialOffer;
use identity_wallet::persistence::ASSETS_DIR;
use identity_wallet::state::core_utils::CoreUtils;
use identity_wallet::state::qr_code::reducers::read_credential_offer::read_credential_offer;
use identity_wallet::state::AppState;
use identity_wallet::{
    oid4vci::credential_format_profiles::{
        w3c_verifiable_credentials::jwt_vc_json, CredentialFormats, Parameters, WithParameters,
    },
    state::qr_code::actions::qrcode_scanned::QrCodeScanned,
};

use oid4vc::oid4vci::credential_issuer::credential_configurations_supported::CredentialConfigurationsSupportedObject;
use oid4vc::oid4vci::credential_offer::CredentialOfferParameters;
use serde_json::json;
use std::sync::Arc;
use tempfile::TempDir;
use url::Url;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
#[serial_test::serial]
async fn download_credential_logo() {
    *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().into_path();

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/offer/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(CredentialOfferParameters {
            credential_issuer: mock_server.uri().parse().unwrap(),
            credential_configuration_ids: vec!["UniversityDegreeCredential".to_string()],
            grants: None,
        }))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/openid-credential-issuer"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(CredentialIssuerMetadata {
                credential_issuer: "https://server.example.com".parse().unwrap(),
                credential_endpoint: Url::parse("https://server.example.com/credential").unwrap(),
                credential_configurations_supported: vec![(
                    "UniversityDegreeCredential".to_string(),
                    CredentialConfigurationsSupportedObject {
                        credential_format: CredentialFormats::<WithParameters>::JwtVcJson(Parameters {
                            parameters: (
                                jwt_vc_json::CredentialDefinition {
                                    type_: vec![
                                        "VerifiableCredential".to_string(),
                                        "UniversityDegreeCredential".to_string(),
                                    ],
                                    credential_subject: Default::default(),
                                },
                                None,
                            )
                                .into(),
                        }),
                        scope: Some("UniversityDegreeCredential".to_string()),
                        cryptographic_binding_methods_supported: vec!["did".to_string()],
                        credential_signing_alg_values_supported: vec!["ES256K".to_string()],
                        proof_types_supported: Default::default(),
                        display: vec![json!({
                            "name": "University Credential",
                            "locale": "en-US",
                            "logo": {
                                "url": format!("{}/logo/credential.svg", &mock_server.uri()),
                                "alternative_text": "a square logo of a university"
                            },
                            "background_color": "#12107c",
                            "text_color": "#FFFFFF"
                        })],
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            }),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/logo/credential.svg"))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(include_bytes!("../res/planet.svg").to_vec(), "image/svg+xml"),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]),
            ..Default::default()
        },
        ..AppState::default()
    };

    assert!(read_credential_offer(
        app_state,
        Arc::new(QrCodeScanned {
            form_urlencoded: CredentialOffer::CredentialOfferUri(
                format!("{}/offer/1", &mock_server.uri()).parse().unwrap()
            )
            .to_string()
        }),
    )
    .await
    .is_ok());
}

#[tokio::test]
#[serial_test::serial]
async fn download_issuer_logo() {
    *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().into_path();

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/offer/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(CredentialOfferParameters {
            credential_issuer: mock_server.uri().parse().unwrap(),
            credential_configuration_ids: vec!["UniversityDegreeCredential".to_string()],
            grants: None,
        }))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/openid-credential-issuer"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(CredentialIssuerMetadata {
                credential_issuer: "https://server.example.com".parse().unwrap(),
                credential_endpoint: Url::parse("https://server.example.com/credential").unwrap(),
                credential_configurations_supported: vec![(
                    "UniversityDegreeCredential".to_string(),
                    CredentialConfigurationsSupportedObject {
                        credential_format: CredentialFormats::<WithParameters>::JwtVcJson(Parameters {
                            parameters: (
                                jwt_vc_json::CredentialDefinition {
                                    type_: vec![
                                        "VerifiableCredential".to_string(),
                                        "UniversityDegreeCredential".to_string(),
                                    ],
                                    credential_subject: Default::default(),
                                },
                                None,
                            )
                                .into(),
                        }),
                        scope: Some("UniversityDegreeCredential".to_string()),
                        cryptographic_binding_methods_supported: vec!["did".to_string()],
                        credential_signing_alg_values_supported: vec!["ES256K".to_string()],
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
                display: Some(vec![json!({
                    "client_name": "University",
                    "logo_uri": format!("{}/logo/issuer.png", &mock_server.uri()),
                })]),
                ..Default::default()
            }),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/logo/issuer.png"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(include_bytes!("../res/unime.png").to_vec(), "image/png"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]),
            ..Default::default()
        },
        ..AppState::default()
    };

    read_credential_offer(
        app_state,
        Arc::new(QrCodeScanned {
            form_urlencoded: CredentialOffer::CredentialOfferUri(
                format!("{}/offer/1", &mock_server.uri()).parse().unwrap(),
            )
            .to_string(),
        }),
    )
    .await
    .unwrap();
}

#[tokio::test]
#[serial_test::serial]
async fn no_download_when_no_logo_in_metadata() {
    *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().into_path();

    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/offer/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(CredentialOfferParameters {
            credential_issuer: mock_server.uri().parse().unwrap(),
            credential_configuration_ids: vec!["UniversityDegreeCredential".to_string()],
            grants: None,
        }))
        .expect(1)
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/.well-known/openid-credential-issuer"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(CredentialIssuerMetadata {
                credential_issuer: "https://server.example.com".parse().unwrap(),
                credential_endpoint: Url::parse("https://server.example.com/credential").unwrap(),
                credential_configurations_supported: vec![(
                    "UniversityDegreeCredential".to_string(),
                    CredentialConfigurationsSupportedObject {
                        credential_format: CredentialFormats::<WithParameters>::JwtVcJson(Parameters {
                            parameters: (
                                jwt_vc_json::CredentialDefinition {
                                    type_: vec![
                                        "VerifiableCredential".to_string(),
                                        "UniversityDegreeCredential".to_string(),
                                    ],
                                    credential_subject: Default::default(),
                                },
                                None,
                            )
                                .into(),
                        }),
                        scope: Some("UniversityDegreeCredential".to_string()),
                        cryptographic_binding_methods_supported: vec!["did".to_string()],
                        credential_signing_alg_values_supported: vec!["ES256K".to_string()],
                        ..Default::default()
                    },
                )]
                .into_iter()
                .collect(),
                ..Default::default()
            }),
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // TODO: assert that function download_asset() is never called (through spy?)

    let app_state = AppState {
        core_utils: CoreUtils {
            managers: test_managers(vec![]),
            ..Default::default()
        },
        ..AppState::default()
    };

    assert!(read_credential_offer(
        app_state,
        Arc::new(QrCodeScanned {
            form_urlencoded: CredentialOffer::CredentialOfferUri(
                format!("{}/offer/1", &mock_server.uri()).parse().unwrap(),
            )
            .to_string()
        }),
    )
    .await
    .is_ok());
}
