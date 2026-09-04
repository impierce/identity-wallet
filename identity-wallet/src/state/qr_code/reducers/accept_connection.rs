use crate::{
    error::AppError::{self, *},
    http_client::get_http_client,
    state::{
        actions::{listen, Action},
        core_utils::{helpers::download_logo, ActiveFlow, CoreUtils, Oid4vciStage},
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
    let origin = redirect_uri.origin().ascii_serialization();
    let connection_url = if origin == "null" {
        redirect_uri.to_string()
    } else {
        origin
    };

    // This means we only accept DID's as client IDs
    // TODO put this in a ADR along with the logging sensitive info decision
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

pub(crate) fn strip_client_id_prefix(client_id: &str) -> String {
    use oid4vc::oid4vp::authorization_request::ClientId;
    use std::str::FromStr as _;

    ClientId::from_str(client_id)
        .map(|client_id| client_id.identifier().to_string())
        .unwrap_or_else(|_| client_id.to_string())
}

pub(crate) async fn get_oid4vp_client_metadata(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> Result<ClientMetadata, AppError> {
    let redirect_uri = oid4vp_authorization_request.body.uri.uri().clone();
    let origin = redirect_uri.origin().ascii_serialization();
    let connection_url = if origin == "null" {
        redirect_uri.to_string()
    } else {
        origin
    };
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
    let origin = credential_issuer_url.origin().ascii_serialization();
    let connection_url = if origin == "null" {
        credential_issuer_url.to_string()
    } else {
        origin
    };

    info!("credential issuer url: {credential_issuer_url:?}");

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
                .unwrap_or_else(|| credential_issuer_url.to_string());
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
        None => (credential_issuer_url.to_string(), None),
    };

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

    // This means we only accept DID's as client IDs
    // TODO put this in a ADR along with the logging sensitive info decision
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
