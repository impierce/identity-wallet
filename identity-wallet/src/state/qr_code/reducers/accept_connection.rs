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
        user_prompt::{ClientMetadata, ConnectionData, CurrentUserPrompt, EcosystemProfile},
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
use openid_federation::{FederationClient, ReqwestHttpClient};
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

                Box::new(validate_domain_linkage(resolver.as_ref(), url.clone(), &did).await)
            }
            #[cfg(feature = "test_utils")]
            {
                // Skip validation during tests

                use crate::state::did::validate_domain_linkage::{ValidationResult, ValidationStatus};
                Box::new(ValidationResult {
                    status: ValidationStatus::default(),
                    url: url.clone(),
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

        let ecosystems = fetch_ecosystems(&url).await;

        let current_user_prompt = Some(CurrentUserPrompt::AcceptConnection {
            client_metadata,
            connection_data,
            domain_validation,
            linked_verifiable_presentations,
            ecosystems,
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

/// Discovers every OpenID Federation trust chain reachable from `entity_id` which is equal to the `client_metadata.connection_url`.
/// Then it fetches the ecosystem profile hosted by each chain's trust anchor. Returns `None` if no trust chains are found
/// or none of their trust anchors expose an ecosystem profile.
async fn fetch_ecosystems(entity_id: &url::Url) -> Option<Vec<EcosystemProfile>> {
    // In the future this must be more flexible to support other types of clients.
    let federation_client = FederationClient::with_http_client(ReqwestHttpClient::with_client(get_http_client().await));

    let trust_chains = match federation_client.discover_all_trust_chains(entity_id).await {
        Ok(trust_chains) => trust_chains,
        Err(e) => {
            warn!("Failed to discover trust chains for entity ID {entity_id}: {e}");
            return None;
        }
    };

    let mut ecosystems = Vec::new();
    for trust_chain in &trust_chains {
        let Ok((trust_anchor_id, _)) = trust_chain.trust_anchor_entity_id_and_configuration() else {
            warn!(
                "Failed to get trust anchor entity ID from trust chain, which should not be possible: {trust_chain:?}"
            );
            continue;
        };

        if let Some(ecosystem_profile) = fetch_ecosystem_profile(&trust_anchor_id).await {
            ecosystems.push(ecosystem_profile);
        }
    }

    info!("Fetched ecosystems for entity ID {entity_id}: {ecosystems:?}");

    match ecosystems {
        vec if !vec.is_empty() => Some(vec),
        _ => None,
    }
}

/// Fetches the ecosystem profile hosted by a trust anchor at its `/public/ecosystem-profile` endpoint.
async fn fetch_ecosystem_profile(trust_anchor_entity_id: &url::Url) -> Option<EcosystemProfile> {
    let ecosystem_profile_url = match trust_anchor_entity_id.join("/public/ecosystem-profile") {
        Ok(url) => url,
        Err(e) => {
            warn!("Failed to build ecosystem profile URL for trust anchor {trust_anchor_entity_id}: {e}");
            return None;
        }
    };

    let response = match get_http_client().await.get(ecosystem_profile_url.clone()).send().await {
        Ok(response) => response,
        Err(e) => {
            warn!("Failed to fetch ecosystem profile from {ecosystem_profile_url}: {e}");
            return None;
        }
    };

    if !response.status().is_success() {
        warn!(
            "Trust anchor {trust_anchor_entity_id} responded with status {} for ecosystem profile",
            response.status()
        );
        return None;
    }

    match response.json::<EcosystemProfile>().await {
        Ok(ecosystem_profile) => Some(ecosystem_profile),
        Err(e) => {
            warn!("Failed to parse ecosystem profile from {ecosystem_profile_url}: {e}");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::{hash, ASSETS_DIR, STRONGHOLD};
    use crate::state::core_utils::{IdentityManager, Managers};
    use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
    use crate::stronghold::StrongholdManager;
    use crate::subject::subject;

    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use oid4vc::oid4vc_manager::ProviderManager;
    use oid4vc::oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;
    use oid4vc::oid4vci::credential_offer::CredentialConfigurationIds;
    use oid4vc::oid4vci::Wallet;
    use openid_federation::{
        expires_in, EntityConfiguration, EntityMetadata, FederationEntityMetadata, Jwk, JwkSet, SubordinateStatement,
    };

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

    // A single symmetric key shared by every entity in these tests; only used to make the JWTs verifiable, not to assert on trust.
    fn federation_test_key() -> Jwk {
        Jwk {
            kty: "oct".to_string(),
            use_: Some("sig".to_string()),
            key_ops: None,
            alg: Some("HS256".to_string()),
            kid: Some("test-key-1".to_string()),
            x5u: None,
            x5c: None,
            x5t: None,
            x5t_s256: None,
            n: None,
            e: None,
            d: None,
            p: None,
            q: None,
            dp: None,
            dq: None,
            qi: None,
            crv: None,
            x: None,
            y: None,
            k: Some("dGVzdF9zZWNyZXRfa2V5".to_string()), // base64 encoded "test_secret_key"
        }
    }

    fn encode_entity_statement<T: serde::Serialize>(claims: &T) -> String {
        let mut header = Header::new(Algorithm::HS256);
        header.typ = Some("entity-statement+jwt".to_string());
        header.kid = Some("test-key-1".to_string());
        encode(&header, claims, &EncodingKey::from_secret(b"test_secret_key")).unwrap()
    }

    /// Mounts a self-signed OpenID Federation Entity Configuration for `entity_id` on its own mock server.
    /// An empty `authority_hints` marks the entity as a trust anchor (the chain's terminal node).
    async fn mount_entity_configuration(
        mock_server: &MockServer,
        entity_id: &url::Url,
        organization_name: &str,
        authority_hints: Vec<url::Url>,
    ) {
        let mut jwks = JwkSet::new();
        jwks.add_key(federation_test_key());

        let mut metadata = EntityMetadata::new();
        metadata.federation_entity = Some(FederationEntityMetadata {
            organization_name: Some(organization_name.to_string()),
            homepage_uri: None,
            policy_uri: None,
            logo_uri: None,
            contacts: None,
            federation_fetch_endpoint: Some(entity_id.join("/federation-fetch").unwrap()),
            federation_list_endpoint: None,
            federation_resolve_endpoint: None,
            federation_trust_mark_status_endpoint: None,
            federation_historical_keys_endpoint: None,
        });

        let mut config = EntityConfiguration::new(
            entity_id.clone(),
            jwks,
            expires_in(chrono::Duration::hours(1)),
            chrono::Utc::now().timestamp(),
        )
        .with_metadata(metadata);
        if !authority_hints.is_empty() {
            config = config.with_authority_hints(authority_hints);
        }

        Mock::given(method("GET"))
            .and(path("/.well-known/openid-federation"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(encode_entity_statement(&config))
                    .insert_header("content-type", "application/entity-statement+jwt"),
            )
            .mount(mock_server)
            .await;
    }

    /// Mounts the Subordinate Statement a trust anchor issues about `subject_entity_id` on its `/federation-fetch` endpoint.
    async fn mount_subordinate_statement(
        trust_anchor_server: &MockServer,
        trust_anchor_id: &url::Url,
        subject_entity_id: &url::Url,
    ) {
        let mut jwks = JwkSet::new();
        jwks.add_key(federation_test_key());

        let statement = SubordinateStatement::new(
            trust_anchor_id.clone(),
            subject_entity_id.clone(),
            expires_in(chrono::Duration::hours(1)),
            chrono::Utc::now().timestamp(),
            jwks,
        );

        Mock::given(method("GET"))
            .and(path("/federation-fetch"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(encode_entity_statement(&statement))
                    .insert_header("content-type", "application/entity-statement+jwt"),
            )
            .mount(trust_anchor_server)
            .await;
    }

    /// Mounts the `/public/ecosystem-profile` endpoint hosted by a trust anchor.
    async fn mount_ecosystem_profile(trust_anchor_server: &MockServer, name: &str) {
        Mock::given(method("GET"))
            .and(path("/public/ecosystem-profile"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "logoUri": format!("{}/logo.png", trust_anchor_server.uri()),
                "name": name,
                "description": null,
                "ecosystemLeader": {
                    "logoUri": format!("{}/logo.png", trust_anchor_server.uri()),
                    "name": name,
                    "description": null,
                    "domain": trust_anchor_server.uri(),
                },
                "memberCount": 1,
                "members": [{
                    "logoUri": format!("{}/logo.png", trust_anchor_server.uri()),
                    "name": name,
                    "description": null,
                    "domain": trust_anchor_server.uri(),
                }],
            })))
            .mount(trust_anchor_server)
            .await;
    }

    #[tokio::test]
    async fn fetch_ecosystems_returns_none_when_the_entity_is_not_part_of_any_federation() {
        let mock_server = MockServer::start().await;
        let entity_id: url::Url = mock_server.uri().parse().unwrap();

        // No `.well-known/openid-federation` is mounted, so the entity configuration fetch fails.
        let ecosystems = fetch_ecosystems(&entity_id).await;

        assert_eq!(ecosystems, None);
    }

    #[tokio::test]
    async fn fetch_ecosystems_returns_a_single_ecosystem_profile_for_one_trust_anchor() {
        let leaf_server = MockServer::start().await;
        let trust_anchor_server = MockServer::start().await;

        let leaf_id: url::Url = leaf_server.uri().parse().unwrap();
        let trust_anchor_id: url::Url = trust_anchor_server.uri().parse().unwrap();

        mount_entity_configuration(&leaf_server, &leaf_id, "Leaf", vec![trust_anchor_id.clone()]).await;
        mount_entity_configuration(&trust_anchor_server, &trust_anchor_id, "Trust Anchor", vec![]).await;
        mount_subordinate_statement(&trust_anchor_server, &trust_anchor_id, &leaf_id).await;
        mount_ecosystem_profile(&trust_anchor_server, "Ecosystem One").await;

        let ecosystems = fetch_ecosystems(&leaf_id).await.unwrap();

        assert_eq!(ecosystems.len(), 1);
        assert_eq!(ecosystems[0].name, "Ecosystem One");
    }

    #[tokio::test]
    async fn fetch_ecosystems_returns_an_ecosystem_profile_per_trust_anchor() {
        let leaf_server = MockServer::start().await;
        let trust_anchor_one_server = MockServer::start().await;
        let trust_anchor_two_server = MockServer::start().await;

        let leaf_id: url::Url = leaf_server.uri().parse().unwrap();
        let trust_anchor_one_id: url::Url = trust_anchor_one_server.uri().parse().unwrap();
        let trust_anchor_two_id: url::Url = trust_anchor_two_server.uri().parse().unwrap();

        mount_entity_configuration(
            &leaf_server,
            &leaf_id,
            "Leaf",
            vec![trust_anchor_one_id.clone(), trust_anchor_two_id.clone()],
        )
        .await;
        mount_entity_configuration(
            &trust_anchor_one_server,
            &trust_anchor_one_id,
            "Trust Anchor One",
            vec![],
        )
        .await;
        mount_entity_configuration(
            &trust_anchor_two_server,
            &trust_anchor_two_id,
            "Trust Anchor Two",
            vec![],
        )
        .await;
        mount_subordinate_statement(&trust_anchor_one_server, &trust_anchor_one_id, &leaf_id).await;
        mount_subordinate_statement(&trust_anchor_two_server, &trust_anchor_two_id, &leaf_id).await;
        mount_ecosystem_profile(&trust_anchor_one_server, "Ecosystem One").await;
        mount_ecosystem_profile(&trust_anchor_two_server, "Ecosystem Two").await;

        let ecosystems = fetch_ecosystems(&leaf_id).await.unwrap();

        assert_eq!(ecosystems.len(), 2);
        let names: std::collections::HashSet<_> = ecosystems.iter().map(|profile| profile.name.as_str()).collect();
        assert!(names.contains("Ecosystem One"));
        assert!(names.contains("Ecosystem Two"));
    }
}
