use crate::{
    error::AppError::{self, *},
    http_client::get_http_client,
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::{download_logo, normalize_connection_url},
            ActiveFlow, CoreUtils, Oid4vciStage,
        },
        did::validate_linked_verifiable_presentations::{
            validate_linked_verifiable_presentations, LinkedVerifiableCredentialData,
        },
        qr_code::actions::qrcode_scanned::QrCodeScanned,
        user_prompt::{ClientMetadata, ConnectionData, CurrentUserPrompt},
        AppState,
    },
};
use identity_iota::did::CoreDID;
use log::{info, warn};
use oid4vc::siopv2::siopv2::SIOPv2;
use oid4vc::{
    oid4vc_core::{
        authorization_request::{AuthorizationRequest, Object},
        client_metadata::ClientMetadataResource,
    },
    oid4vci::credential_offer::CredentialOffer,
};
use oid4vc::{oid4vci::credential_offer::CredentialOfferParameters, oid4vp::oid4vp::OID4VP};
use serde_json::Value;

/// The kind of request encoded in a scanned QR-code.
///
/// SIOPv2 and OID4VP requests are classified through `AuthorizationRequest::from_generic`, while
/// OID4VCI credential offers use their own URL scheme and are parsed directly from the raw string.
#[derive(Debug, Clone)]
enum ParsedQrCode {
    Siopv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    Oid4vp(Box<AuthorizationRequest<Object<OID4VP>>>),
    Oid4vci(Box<CredentialOfferParameters>),
}

/// Sets the `AcceptConnection` prompt; the following `ConnectionAccepted` action routes to the next reducer depending on the `ActiveFlow` set here.
/// 1. Read and parse the QR-code to a URL.
/// 2. Retrieve the connection data to display on the "Accept connection" screen.
/// 3. Init the `ActiveFlow` enum with the rest of the retrieved data.
pub async fn accept_connection(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(qr_code_scanned) = listen::<QrCodeScanned>(action).map(|payload| payload.form_urlencoded) {
        let parsed_qr_code = parse_qr_code(&state, qr_code_scanned).await?;
        info!("QR code parsed as: {parsed_qr_code:?}");

        let (client_metadata, active_flow) =
            get_client_metadata_init_active_flow(&state, parsed_qr_code.clone()).await?;
        info!("Retrieved client metadata: {client_metadata:?}");
        info!("Initializing active flow: {active_flow:?}");

        let did = client_metadata.client_id.to_string();
        let connection_data = state
            .connections
            .0
            .iter()
            // TODO: currently we only match against the DID, but if any display info changes with what we stored we plan to notify the user of the diffs.
            .find(|conn| conn.did == did)
            .map(|connection| {
                let interactions = state
                    .history
                    .iter()
                    .filter(|event| event.connection_id == connection.id)
                    .cloned()
                    .collect();
                ConnectionData {
                    first_interacted_at: connection.first_interacted.clone(),
                    last_interacted_at: connection.last_interacted.clone(),
                    interactions,
                }
            });

        let url = url::Url::parse(&client_metadata.connection_url).map_err(|_| {
            Error(format!(
                "`connection_url` could not be parsed to URL: `{:?}`",
                client_metadata.connection_url.clone()
            ))
        })?;

        let state_guard = state.core_utils.managers.lock().await;
        let subject = state_guard
            .identity_manager
            .as_ref()
            .ok_or(AppError::MissingManagerError("identity"))?
            .subject
            .clone();

        info!(
            "Checking domain linkage for DID: {did} and URL: {}",
            client_metadata.connection_url
        );

        let domain_validation = {
            #[cfg(not(feature = "test_utils"))]
            {
                use crate::state::did::validate_domain_linkage::validate_domain_linkage;

                let resolver = subject.resolver().await;

                Box::new(validate_domain_linkage(resolver.as_ref(), url, &did).await)
            }
            #[cfg(feature = "test_utils")]
            {
                // Skip validation during tests

                use crate::state::did::validate_domain_linkage::{ValidationResult, ValidationStatus};
                Box::new(ValidationResult {
                    status: ValidationStatus::default(),
                    url,
                    name: None,
                    logo_uri: None,
                    issuance_date: None,
                    message: None,
                })
            }
        };

        info!("Domain validation result: {domain_validation:?}");

        let linked_verifiable_presentations = match validate_linked_verifiable_presentations(&subject, &did)
            .await
            .into_iter()
            .flatten()
            .collect::<Vec<LinkedVerifiableCredentialData>>()
        {
            vec if !vec.is_empty() => Some(vec),
            _ => None,
        };

        info!("Linked verifiable presentations: {linked_verifiable_presentations:?}");

        drop(state_guard);

        let current_user_prompt = Some(CurrentUserPrompt::AcceptConnection {
            client_metadata,
            connection_data,
            domain_validation,
            linked_verifiable_presentations,
            ecosystems: None, // TODO: impl this
        });

        info!("Setting current user prompt to: {current_user_prompt:?}");

        Ok(AppState {
            current_user_prompt,
            core_utils: CoreUtils {
                active_flow: Some(active_flow),
                ..state.core_utils
            },
            ..state
        })
    } else {
        Ok(state)
    }
}

// Helpers

// OID4VCI credential offers are handled by a dedicated reducer, so they're
// parsed directly here rather than through `provider_manager.validate_request`.
async fn parse_qr_code(state: &AppState, qr_code_scanned: String) -> Result<ParsedQrCode, AppError> {
    let state_guard = state.core_utils.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    if let Ok(credential_offer) = qr_code_scanned.parse::<CredentialOffer>() {
        let credential_offer: CredentialOfferParameters = match credential_offer {
            CredentialOffer::CredentialOffer(credential_offer) => *credential_offer,
            CredentialOffer::CredentialOfferUri(credential_offer_uri) => wallet
                .get_credential_offer(credential_offer_uri)
                .await
                .map_err(GetCredentialOfferError)?,
        };

        return Ok(ParsedQrCode::Oid4vci(Box::new(credential_offer)));
    }

    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let generic_authorization_request = provider_manager
        .validate_request(qr_code_scanned.clone())
        .await
        .map_err(|_| InvalidQRCodeError(qr_code_scanned.clone()))?;

    if let Result::Ok(siopv2_authorization_request) =
        AuthorizationRequest::<Object<SIOPv2>>::from_generic(&generic_authorization_request)
    {
        Ok(ParsedQrCode::Siopv2(Box::new(siopv2_authorization_request)))
    } else if let Result::Ok(oid4vp_authorization_request) =
        AuthorizationRequest::<Object<OID4VP>>::from_generic(&generic_authorization_request)
    {
        Ok(ParsedQrCode::Oid4vp(Box::new(oid4vp_authorization_request)))
    } else {
        Err(InvalidAuthorizationRequest(Box::new(generic_authorization_request)))
    }
}

/// This function retrieves the client metadata and initializes the active flow based on the parsed QR code.
/// For SIOPv2 the next reducer will be `handle_siopv2_authorization_request`.
/// For OID4VP the next reducer will be `read_oid4vp_authorization_request`.
/// For OID4VCI the next reducer will be `read_credential_offer`.
async fn get_client_metadata_init_active_flow(
    state: &AppState,
    parsed_qr_code: ParsedQrCode,
) -> Result<(ClientMetadata, ActiveFlow), AppError> {
    match parsed_qr_code {
        ParsedQrCode::Siopv2(siopv2_authorization_request) => {
            let client_metadata = get_siopv2_client_metadata(&siopv2_authorization_request).await?;
            let active_flow = ActiveFlow::Siopv2 {
                authorization_request: siopv2_authorization_request,
            };
            Ok((client_metadata, active_flow))
        }
        ParsedQrCode::Oid4vp(oid4vp_authorization_request) => {
            let client_metadata = get_oid4vp_client_metadata(&oid4vp_authorization_request).await?;
            let active_flow = ActiveFlow::Oid4vp {
                authorization_request: oid4vp_authorization_request,
                is_interactive: false,
            };
            Ok((client_metadata, active_flow))
        }
        ParsedQrCode::Oid4vci(credential_offer) => {
            let client_metadata = get_oid4vci_client_metadata(state, &credential_offer).await?;
            let active_flow = ActiveFlow::Oid4vciOffer {
                stage: Oid4vciStage::OfferReceived,
                logo_uri: client_metadata.logo_uri.clone(),
                credential_offer,
            };
            Ok((client_metadata, active_flow))
        }
    }
}

async fn get_siopv2_client_metadata(
    siopv2_authorization_request: &AuthorizationRequest<Object<SIOPv2>>,
) -> Result<ClientMetadata, AppError> {
    let redirect_uri = siopv2_authorization_request.body.uri.uri().clone();
    let connection_url = normalize_connection_url(&redirect_uri);

    let client_id = strip_client_id_prefix(&siopv2_authorization_request.body.client_id);
    let client_id =
        CoreDID::parse(&client_id).map_err(|e| AppError::Error(format!("Failed to parse client_id as DID: {e}")))?;

    Ok(match &siopv2_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name, logo_uri, ..
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or_else(|| connection_url.clone());
            let mut logo_uri = logo_uri.as_ref().map(ToString::to_string);

            if let Some(logo_uri_str) = &logo_uri {
                if download_logo(logo_uri_str).await.is_none() {
                    logo_uri = None;
                }
            } else {
                warn!("No logo URI found");
            }

            ClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.clone(),
                client_id: client_id.clone(),
                redirect_uri: Some(redirect_uri.to_string()),
            }
        }
        ClientMetadataResource::ClientMetadataUri(_) => {
            return Err(Error("Client metadata URI not supported".to_string()));
        }
    })
}

pub(crate) async fn get_oid4vp_client_metadata(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> Result<ClientMetadata, AppError> {
    let redirect_uri = oid4vp_authorization_request.body.uri.uri().clone();
    let connection_url = normalize_connection_url(&redirect_uri);
    let client_id = CoreDID::parse(strip_client_id_prefix(&oid4vp_authorization_request.body.client_id))
        .map_err(|error| AppError::Error(format!("Failed to parse client_id as DID: {error}")))?;

    Ok(match &oid4vp_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name, logo_uri, ..
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or_else(|| connection_url.clone());
            let mut logo_uri = logo_uri.as_ref().map(ToString::to_string);

            if let Some(logo_uri_str) = &logo_uri {
                if download_logo(logo_uri_str).await.is_none() {
                    logo_uri = None;
                }
            } else {
                warn!("No logo URI found");
            }

            ClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.clone(),
                client_id,
                redirect_uri: Some(redirect_uri.to_string()),
            }
        }
        ClientMetadataResource::ClientMetadataUri(_) => {
            return Err(Error("Client metadata URI not supported".to_string()));
        }
    })
}

async fn get_oid4vci_client_metadata(
    state: &AppState,
    credential_offer: &CredentialOfferParameters,
) -> Result<ClientMetadata, AppError> {
    let state_guard = state.core_utils.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    let credential_issuer_url = credential_offer.credential_issuer.clone();
    let connection_url = normalize_connection_url(&credential_issuer_url);

    info!("credential issuer url: {credential_issuer_url:?}");
    info!("connection url: {connection_url:?}");

    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .ok();

    let display = credential_issuer_metadata
        .as_ref()
        .and_then(|metadata| metadata.display.as_ref()?.first().cloned());

    let (issuer_name, logo_uri) = match display {
        Some(display) => {
            let issuer_name = display["name"]
                .as_str()
                .map(ToString::to_string)
                .unwrap_or_else(|| connection_url.clone());
            let mut logo_uri = display["logo"]["uri"].as_str().map(ToString::to_string);

            if let Some(logo_uri_str) = &logo_uri {
                if download_logo(logo_uri_str).await.is_none() {
                    logo_uri = None;
                }
            } else {
                warn!("No logo URI found");
            }

            (issuer_name, logo_uri)
        }
        None => (connection_url.clone(), None),
    };

    // This fetching of the DID document means that our OID4VCI implementation only accepts did:web's as client IDs.
    // Read more about this design decision in ADR 0001.
    let did_doc = get_http_client()
        .await
        .get(format!(
            "{}/.well-known/did.json",
            credential_issuer_url.to_string().trim_end_matches('/')
        ))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let client_id = did_doc
        .get("id")
        .and_then(Value::as_str)
        .ok_or(AppError::DidParseError)?
        .to_string();
    let client_id =
        CoreDID::parse(&client_id).map_err(|e| AppError::Error(format!("Failed to parse client_id as DID: {e}")))?;

    Ok(ClientMetadata {
        client_name: issuer_name,
        redirect_uri: Some(credential_issuer_url.to_string()),
        connection_url,
        logo_uri,
        client_id,
    })
}

fn strip_client_id_prefix(client_id: &str) -> String {
    use oid4vc::oid4vp::authorization_request::ClientId;
    use std::str::FromStr as _;

    ClientId::from_str(client_id)
        .map(|client_id| client_id.identifier().to_string())
        .unwrap_or_else(|_| client_id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{hash, ASSETS_DIR, STRONGHOLD};
    use crate::state::core_utils::{IdentityManager, Managers};
    use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
    use crate::stronghold::StrongholdManager;
    use crate::subject::subject;

    use oid4vc::oid4vc_manager::ProviderManager;
    use oid4vc::oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;
    use oid4vc::oid4vci::credential_offer::CredentialConfigurationIds;
    use oid4vc::oid4vci::Wallet;

    use serde_json::json;
    use std::sync::Arc;
    use tempfile::{NamedTempFile, TempDir};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    const ISSUER_NAME: &str = "University";
    const ISSUER_DID: &str = "did:web:example.com";
    const CLIENT_DID: &str = "did:key:z6Mkm9yeuZK7inXBNjnNH3vAs9uUjqfy3mfNoKBKsKBrv8Tb";
    const CLIENT_NAME: &str = "Example Relying Party";

    async fn test_state() -> AppState {
        let stronghold_path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = stronghold_path.as_os_str().into();

        let password = "sup3rSecr3t".to_string();
        let stronghold_manager = Arc::new(StrongholdManager::create(&password).unwrap());
        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap();
        let wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .unwrap();

        AppState {
            core_utils: CoreUtils {
                managers: Arc::new(tauri::async_runtime::Mutex::new(Managers {
                    stronghold_manager: Some(stronghold_manager),
                    identity_manager: Some(IdentityManager {
                        subject,
                        provider_manager,
                        wallet,
                    }),
                })),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn credential_offer(credential_issuer: &str) -> CredentialOfferParameters {
        CredentialOfferParameters {
            credential_issuer: credential_issuer.parse().unwrap(),
            credential_configuration_ids: CredentialConfigurationIds::try_new(vec![
                "UniversityDegreeCredential".to_string()
            ])
            .unwrap(),
            grants: None,
        }
    }

    async fn mount_did_document(mock_server: &MockServer) {
        Mock::given(method("GET"))
            .and(path("/.well-known/did.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "id": ISSUER_DID })))
            .expect(1)
            .mount(mock_server)
            .await;
    }

    async fn mount_issuer_metadata(mock_server: &MockServer, display: Option<Vec<serde_json::Value>>) {
        Mock::given(method("GET"))
            .and(path("/.well-known/openid-credential-issuer"))
            .respond_with(ResponseTemplate::new(200).set_body_json(CredentialIssuerMetadata {
                credential_issuer: mock_server.uri().parse().unwrap(),
                credential_endpoint: format!("{}/credential", mock_server.uri()).parse().unwrap(),
                display,
                ..Default::default()
            }))
            .expect(1)
            .mount(mock_server)
            .await;
    }

    async fn mount_logo(mock_server: &MockServer) -> String {
        Mock::given(method("GET"))
            .and(path("/logo/client.png"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0x89, 0x50, 0x4E, 0x47], "image/png"))
            .expect(1)
            .mount(mock_server)
            .await;
        format!("{}/logo/client.png", mock_server.uri())
    }

    fn siopv2_request(client_metadata: serde_json::Value) -> String {
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", CLIENT_DID)
            .append_pair("client_metadata", &client_metadata.to_string())
            .append_pair("nonce", "nonce")
            .append_pair("redirect_uri", "https://example.com/")
            .append_pair("response_type", "id_token")
            .append_pair("scope", "openid")
            .finish();
        format!("openid://?{query}")
    }

    fn oid4vp_request(client_metadata: serde_json::Value) -> String {
        let dcql_query = json!({
            "credentials": [{
                "id": "CredentialQuery",
                "format": "jwt_vc_json",
                "meta": { "type_values": [["VerifiableCredential"], ["PersonalInformation"]] },
                "claims": [{ "path": ["credentialSubject", "givenName"] }],
            }],
        });
        let query = url::form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", CLIENT_DID)
            .append_pair("client_metadata", &client_metadata.to_string())
            .append_pair("dcql_query", &dcql_query.to_string())
            .append_pair("nonce", "nonce")
            .append_pair("redirect_uri", "https://example.com/")
            .append_pair("response_mode", "direct_post")
            .append_pair("response_type", "vp_token")
            .finish();
        format!("openid://?{query}")
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn issuer_logo_is_downloaded_and_client_metadata_is_built_from_the_issuer_display() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        let issuer_logo_uri = format!("{}/logo/issuer.png", mock_server.uri());

        mount_issuer_metadata(
            &mock_server,
            Some(vec![json!({
                "name": ISSUER_NAME,
                "logo": { "uri": issuer_logo_uri },
            })]),
        )
        .await;
        mount_did_document(&mock_server).await;

        Mock::given(method("GET"))
            .and(path("/logo/issuer.png"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(vec![0x89, 0x50, 0x4E, 0x47], "image/png"))
            .expect(1)
            .mount(&mock_server)
            .await;

        let state = test_state().await;
        let client_metadata = get_oid4vci_client_metadata(&state, &credential_offer(&mock_server.uri()))
            .await
            .unwrap();

        assert_eq!(client_metadata.client_name, ISSUER_NAME);
        assert_eq!(client_metadata.logo_uri.as_deref(), Some(issuer_logo_uri.as_str()));
        assert_eq!(client_metadata.connection_url, mock_server.uri());
        assert_eq!(client_metadata.client_id.to_string(), ISSUER_DID);

        // Logos are downloaded into `assets/tmp` under the hash of their URI.
        let downloaded_logo = ASSETS_DIR
            .lock()
            .unwrap()
            .join("tmp")
            .join(format!("{}.png", hash(&issuer_logo_uri)));
        assert!(downloaded_logo.exists());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn scanning_an_unparsable_qr_code_is_rejected() {
        let state = test_state().await;

        let result = accept_connection(
            state,
            Arc::new(QrCodeScanned {
                form_urlencoded: "invalid payload".to_string(),
            }),
        )
        .await;

        assert!(matches!(result, Err(InvalidQRCodeError(payload)) if payload == "invalid payload"));
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn the_issuer_url_is_used_as_name_when_the_metadata_has_no_display() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        mount_issuer_metadata(&mock_server, None).await;
        mount_did_document(&mock_server).await;

        let state = test_state().await;
        let client_metadata = get_oid4vci_client_metadata(&state, &credential_offer(&mock_server.uri()))
            .await
            .unwrap();

        assert_eq!(client_metadata.client_name, mock_server.uri());
        assert_eq!(client_metadata.logo_uri, None);

        // The `assets/tmp` folder is only created by a download, so its absence proves none was attempted.
        assert!(!ASSETS_DIR.lock().unwrap().join("tmp").exists());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn siopv2_request_initializes_the_siopv2_flow_and_downloads_the_client_logo() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        let logo_uri = mount_logo(&mock_server).await;

        let state = test_state().await;
        let parsed = parse_qr_code(
            &state,
            siopv2_request(json!({
                "client_name": CLIENT_NAME,
                "logo_uri": logo_uri,
                "id_token_signed_response_alg": "EdDSA",
                "subject_syntax_types_supported": ["did:key"],
            })),
        )
        .await
        .unwrap();
        assert!(matches!(parsed, ParsedQrCode::Siopv2(_)));

        let (client_metadata, active_flow) = get_client_metadata_init_active_flow(&state, parsed).await.unwrap();

        assert_eq!(client_metadata.client_name, CLIENT_NAME);
        assert_eq!(client_metadata.logo_uri.as_deref(), Some(logo_uri.as_str()));
        assert_eq!(client_metadata.connection_url, "https://example.com");
        assert_eq!(client_metadata.redirect_uri.as_deref(), Some("https://example.com/"));
        assert_eq!(client_metadata.client_id.to_string(), CLIENT_DID);
        assert!(matches!(active_flow, ActiveFlow::Siopv2 { .. }));

        let downloaded_logo = ASSETS_DIR
            .lock()
            .unwrap()
            .join("tmp")
            .join(format!("{}.png", hash(&logo_uri)));
        assert!(downloaded_logo.exists());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn siopv2_request_without_client_name_falls_back_to_the_connection_url() {
        let state = test_state().await;
        let parsed = parse_qr_code(
            &state,
            siopv2_request(json!({
                "id_token_signed_response_alg": "EdDSA",
                "subject_syntax_types_supported": ["did:key"],
            })),
        )
        .await
        .unwrap();

        let (client_metadata, _) = get_client_metadata_init_active_flow(&state, parsed).await.unwrap();

        assert_eq!(client_metadata.client_name, "https://example.com");
        assert_eq!(client_metadata.logo_uri, None);
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn oid4vp_request_initializes_the_oid4vp_flow_and_downloads_the_client_logo() {
        *ASSETS_DIR.lock().unwrap() = TempDir::new().unwrap().keep();

        let mock_server = MockServer::start().await;
        let logo_uri = mount_logo(&mock_server).await;

        let state = test_state().await;
        let parsed = parse_qr_code(
            &state,
            oid4vp_request(json!({
                "client_name": CLIENT_NAME,
                "logo_uri": logo_uri,
                "vp_formats_supported": { "jwt_vc_json": { "alg_values": ["EdDSA"] } },
                "subject_syntax_types_supported": ["did:key"],
            })),
        )
        .await
        .unwrap();
        assert!(matches!(parsed, ParsedQrCode::Oid4vp(_)));

        let (client_metadata, active_flow) = get_client_metadata_init_active_flow(&state, parsed).await.unwrap();

        assert_eq!(client_metadata.client_name, CLIENT_NAME);
        assert_eq!(client_metadata.logo_uri.as_deref(), Some(logo_uri.as_str()));
        assert_eq!(client_metadata.connection_url, "https://example.com");
        assert_eq!(client_metadata.client_id.to_string(), CLIENT_DID);
        // OID4VP flows started from a QR-code are never part of an interactive OID4VCI authorization.
        assert!(matches!(
            active_flow,
            ActiveFlow::Oid4vp {
                is_interactive: false,
                ..
            }
        ));

        let downloaded_logo = ASSETS_DIR
            .lock()
            .unwrap()
            .join("tmp")
            .join(format!("{}.png", hash(&logo_uri)));
        assert!(downloaded_logo.exists());
    }
}
