use crate::state::connections::Connections;
use crate::state::core_utils::helpers::download_logo;
use crate::state::core_utils::IdentityManager;
use crate::state::credentials::reducers::self_issue_credential::SubjectWrapper;
use crate::state::credentials::Sha256Hasher;
use crate::stronghold::StrongholdManager;
use crate::subject::Subject;
use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ActiveFlow, Oid4vciStage,
        },
        credentials::actions::credentials_selected::CredentialsSelected,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use chrono::{Duration, Utc};
use identity_core::common::Object as IotaObject;
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_iota::credential::{EnvelopedVc, VcDataUrl};
use identity_iota::did::CoreDID;
use log::{debug, info, warn};
use oid4vc::oid4vc_core::types::string_or_object::StringOrObject;
use oid4vc::oid4vc_core::utils::jwt::get_unverified_jwt_claims;
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::oid4vc_core::{jwt, Sign, Subject as _};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use oid4vc::oid4vp::token::vp_token::Presentations;
use oid4vc::oid4vp::token::vp_token_validator::DecodedPresentations;
use oid4vc::oid4vp::{
    authorization_request::ClientId,
    dcql::dcql_query::{CredentialQuery, Format},
    oid4vp::OID4VP,
    token::{
        verifiable_presentation_jwt::VerifiablePresentationJwt, vp_token::VpToken, vp_token_builder::VpTokenBuilder,
    },
};

use identity_credential::{credential::Jwt, presentation::Presentation};
use identity_iota::core::Url;
use jsonwebtoken::Algorithm;
use jsonwebtoken::Header;
use sd_jwt::{KeyBindingJwtBuilder, RequiredKeyBinding, SdJwt};
use serde_json::Value;
use std::str::FromStr as _;
use std::sync::Arc;
use uuid::Uuid;

// Sends the authorization response including the verifiable credentials.
#[tracing::instrument(skip_all, err)]
pub async fn handle_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(credential_uuids) = listen::<CredentialsSelected>(action)
        .and_then(|payload| (!payload.is_interactive).then_some(payload.credential_uuids))
    {
        info!(
            "Responding to OID4VP authorization request with {} credential(s)",
            credential_uuids.len()
        );

        let state_guard = state.core_utils.managers.lock().await;

        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;
        let identity_manager = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?;
        let provider_manager = &identity_manager.provider_manager;

        let oid4vp_authorization_request = match state.core_utils.active_flow.clone() {
            Some(ActiveFlow::Oid4vp {
                authorization_request, ..
            }) => authorization_request,
            _ => {
                return Err(AppError::Error("Expected OID4VP Authorization Request".to_string()));
            }
        };

        let (vp_token_payload, history_credentials) = build_oid4vp_vp_token_and_history_credentials(
            &state,
            stronghold_manager,
            identity_manager,
            credential_uuids,
        )
        .await?;

        let response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        debug!("Generated OID4VP authorization response: {response:?}");

        #[cfg(not(feature = "test_utils"))]
        if provider_manager.send_response(&response).await.is_err() {
            warn!("Failed to send OID4VP authorization response to verifier");
            return Err(SendAuthorizationResponseError);
        }
        info!("OID4VP presentation response successfully sent to verifier");

        let mut connections = state.connections;
        let mut history = state.history;

        update_history_and_connections(
            &oid4vp_authorization_request,
            history_credentials,
            &mut connections,
            &mut history,
        )
        .await?;

        drop(state_guard);
        return Ok(AppState {
            connections,
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            history,
            ..state
        });
    }

    Ok(state)
}

// TODO: move this struct as it is now generic
#[derive(Debug, Clone)]
pub struct ClientMetadata {
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub connection_url: String,
    pub redirect_uri: Option<String>,
    pub client_id: String,
}

/// Strips the OID4VP Client Identifier Prefix (e.g. `decentralized_identifier:`) to get the bare identifier.
pub fn strip_client_id_prefix(client_id: &str) -> String {
    ClientId::from_str(client_id)
        .map(|client_id| client_id.identifier().to_string())
        .unwrap_or_else(|_| client_id.to_string())
}

// TODO: move this functionality to the oid4vc-manager crate.
// TODO: this fn is nearly an exact copy of the fn `get_siopv2_client_name_and_logo_uri`, is there a simple way to put this into one generic helper?
/// Returns (client_name, logo_uri, connection_url, client_id)
pub async fn get_oid4vp_client_metadata(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> Result<ClientMetadata, AppError> {
    let redirect_uri = oid4vp_authorization_request.body.uri.uri().clone();
    // Inner workings of `origin()` and `ascii_serialization()` are slightly unusual and basically return a "null" string when the operation failed.
    let origin = redirect_uri.origin().ascii_serialization();
    let connection_url = if origin == "null" {
        redirect_uri.as_str()
    } else {
        origin.as_str()
    };

    let client_id = strip_client_id_prefix(&oid4vp_authorization_request.body.client_id);

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(match &oid4vp_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name, logo_uri, ..
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
            let mut logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            if let Some(logo_uri_str) = logo_uri.clone() {
                if download_logo(&logo_uri_str).await.is_none() {
                    // If the logo download fails, we don't throw an error.
                    logo_uri = None;
                }
            } else {
                warn!("No logo URI found");
            }

            Ok(ClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.to_string(),
                client_id: client_id.clone(),
                redirect_uri: Some(redirect_uri.to_string()),
            })
        }
        // TODO: support `client_metadata_uri`
        ClientMetadataResource::ClientMetadataUri(_) => Err(Error("Client metadata URI not supported".to_string())),
    }
    // Otherwise use the connection_url as the client_name.
    .unwrap_or_else(|_| ClientMetadata {
        client_name: connection_url.to_string(),
        logo_uri: None,
        connection_url: connection_url.to_string(),
        client_id,
        redirect_uri: None,
    }))
}

#[tracing::instrument(skip_all, err)]
pub async fn build_oid4vp_vp_token_and_history_credentials(
    state: &AppState,
    stronghold_manager: &StrongholdManager,
    identity_manager: &IdentityManager,
    credential_uuids: Vec<Uuid>,
) -> Result<(VpToken, Vec<HistoryCredential>), AppError> {
    let oid4vp_authorization_request = match state.core_utils.active_flow.clone() {
        Some(ActiveFlow::Oid4vp {
            authorization_request, ..
        }) => authorization_request,
        Some(ActiveFlow::Oid4vciOffer {
            stage: Oid4vciStage::InteractiveAuthorization {
                authorization_request, ..
            },
            ..
        }) => authorization_request,
        _ => {
            return Err(AppError::Error("Expected OID4VP Authorization Request".to_string()));
        }
    };

    let mut history_credentials = vec![];
    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let available_credentials_map: std::collections::HashMap<String, _> = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|record| (record.display_credential.id.clone(), record))
        .collect();

    // TODO: Optimize credential selection so that evaluate_credential_query does not need to be called twice.
    let mut selected_verifiable_credentials: Vec<(CredentialQuery, serde_json::Value)> = Vec::new();
    for requested_credential_query in &dcql_query.credentials {
        for user_selected_uuid in &credential_uuids {
            let user_selected_uuid_str = user_selected_uuid.to_string();
            if let Some(verifiable_credential_record) = available_credentials_map.get(&user_selected_uuid_str) {
                let credential_data = if verifiable_credential_record.display_credential.format
                    == CredentialFormats::DcSdJwt(())
                {
                    // Handle SD-JWTs to get disclosed claims
                    let sd_jwt_vc_string = verifiable_credential_record
                        .verifiable_credential
                        .as_str()
                        .ok_or(AppError::InvalidCredentialFormatError)?
                        .to_string();

                    let sd_jwt_vc = sd_jwt_vc_string
                        .parse::<identity_credential::sd_jwt_vc::SdJwtVc>()
                        .map_err(|e| AppError::Error(format!("Failed to parse stored SD-JWT VC: {e}")))?;

                    let disclosed_object = sd_jwt_vc
                        .into_disclosed_object(&Sha256Hasher::new())
                        .map_err(|e| AppError::Error(format!("Failed to get disclosed object from SD-JWT VC: {e}")))?;
                    serde_json::json!(disclosed_object)
                } else if verifiable_credential_record.display_credential.format == CredentialFormats::JwtVcJson(()) {
                    get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
                        .unwrap_or_default()
                        .get("vc")
                        .cloned()
                        .unwrap_or_else(|| {
                            log::debug!(
                                "JWT-VC-JSON missing 'vc' claim or it's not a valid JSON value for credential ID: {}",
                                verifiable_credential_record.display_credential.id
                            );
                            serde_json::json!({})
                        })
                } else {
                    log::warn!(
                        "Unsupported format {:?} for evaluation. Attempting to get unverified JWT claims for credential ID: {}",
                        verifiable_credential_record.display_credential.format,
                        verifiable_credential_record.display_credential.id
                    );
                    get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential).unwrap_or_default()
                };

                let Some(object) = credential_data.as_object() else {
                    warn!(
                        "Credential data is not a JSON object for credential ID: {}",
                        verifiable_credential_record.display_credential.id
                    );
                    continue;
                };
                let Ok(presentations) = DecodedPresentations::try_new(vec![object.clone()]) else {
                    warn!(
                        "Failed to create DecodedPresentations for credential ID: {}",
                        verifiable_credential_record.display_credential.id
                    );
                    continue;
                };
                let credential_query_satisfied = oid4vc::oid4vp::dcql_evaluation::evaluate_credential_query(
                    requested_credential_query,
                    &presentations,
                );

                if credential_query_satisfied {
                    selected_verifiable_credentials.push((
                        requested_credential_query.clone(),
                        verifiable_credential_record.verifiable_credential.clone(),
                    ));
                    history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
                    break;
                }
            }
        }
    }

    if selected_verifiable_credentials.is_empty() {
        return Err(AppError::Error(
            "No credentials selected or found to fulfill the request".to_string(),
        ));
    }

    let did_method = state
        .profile_settings
        .preferred_did_methods
        .first()
        .ok_or(AppError::Error("Default DID method is missing".to_string()))?;

    let algorithm = identity_manager
        .provider_manager
        .get_matching_signing_algorithm(&oid4vp_authorization_request)
        .await
        .unwrap_or(
            *identity_manager
                .provider_manager
                .provider
                .supported_signing_algorithms
                .first()
                .ok_or_else(|| {
                    AppError::Error("Provider manager does not contain any supported signing algorithms".to_string())
                })?,
        );

    let vp_token_payload = get_vp_token(
        selected_verifiable_credentials,
        did_method,
        &identity_manager.subject,
        &oid4vp_authorization_request,
        algorithm,
    )
    .await?;

    Ok((vp_token_payload, history_credentials))
}

#[tracing::instrument(skip_all)]
pub async fn update_history_and_connections(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
    history_credentials: Vec<HistoryCredential>,
    connections: &mut Connections,
    history: &mut Vec<HistoryEvent>,
) -> Result<(), AppError> {
    let ClientMetadata {
        client_name,
        logo_uri,
        connection_url,
        client_id,
        ..
    } = get_oid4vp_client_metadata(oid4vp_authorization_request).await?;

    let did = CoreDID::parse(client_id).ok();

    let previously_connected = connections.contains(connection_url.as_str(), &client_name);
    let connection = connections.update_or_insert(&connection_url, &client_name, did);

    let file_name = match logo_uri {
        Some(logo_uri) => hash(logo_uri.as_str()),
        None => "_".to_string(),
    };
    persist_asset(&file_name, &connection.id).ok();

    // History
    if !previously_connected {
        // Only add a `ConnectionAdded` event if the connection was not previously connected.
        history.push(HistoryEvent {
            connection_name: connection.name.clone(),
            event_type: EventType::ConnectionAdded,
            connection_id: connection.id.clone(),
            date: connection.last_interacted.clone(),
            credentials: vec![],
        });
    }
    history.push(HistoryEvent {
        connection_name: connection.name.clone(),
        event_type: EventType::CredentialsShared,
        connection_id: connection.id.clone(),
        date: connection.last_interacted.clone(),
        credentials: history_credentials,
    });

    Ok(())
}

async fn get_vp_token(
    selected_verifiable_credentials: Vec<(CredentialQuery, Value)>,
    did_method: &str,
    subject_manager: &Arc<Subject>,
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
    signing_algorithm: Algorithm,
) -> Result<VpToken, AppError> {
    let verifier_audience = oid4vp_authorization_request.body.client_id.to_string();
    let required_nonce = oid4vp_authorization_request.body.extension.nonce.clone();

    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let mut builder = VpTokenBuilder::builder_dcql_query(dcql_query.clone());

    let key_id = subject_manager
        .key_id(did_method, signing_algorithm)
        .await
        .ok_or_else(|| AppError::Error(format!("Failed to get signing method ID for DID method {did_method}")))?;

    let full_did = subject_manager
        .identifier(did_method, signing_algorithm)
        .await
        .map_err(|e| AppError::Error(format!("Failed to get DID identifier: {e}")))?;

    let holder_url: Url =
        Url::parse(&full_did).map_err(|e| AppError::Error(format!("Failed to parse DID as URL: {e}")))?;

    // TODO: Move most of this logic to `openid4vc` crates.
    for (credential_query_from_dcql, vc_value) in selected_verifiable_credentials {
        let credential_query_id = credential_query_from_dcql.id.clone();
        let format_from_query = credential_query_from_dcql.format;

        let presentation_format_item = match format_from_query {
            Format::JwtVcJson => {
                let raw_vc_jwt_string = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string();

                let vc_jwt: Jwt = raw_vc_jwt_string.into();

                let presentation = Presentation::builder(holder_url.clone(), IotaObject::default())
                    .credential(vc_jwt)
                    .build()
                    .map_err(AppError::PresentationBuilderError)?;

                let verifiable_presentation_jwt = VerifiablePresentationJwt::builder()
                    .iss(full_did.clone())
                    .sub(full_did.clone())
                    .aud(verifier_audience.to_string())
                    .nonce(required_nonce.to_string())
                    .iat(Utc::now().timestamp())
                    .exp((Utc::now() + Duration::minutes(10)).timestamp())
                    .verifiable_presentation(presentation)
                    .build()
                    .map_err(|e| AppError::Error(format!("Failed to build VerifiablePresentationJwt: {e}")))?;

                let jwt_header = Header {
                    alg: signing_algorithm,
                    kid: Some(key_id.to_string()),
                    typ: Some("JWT".to_string()),
                    ..Default::default()
                };

                let signed_vc_presentation_jwt_string = jwt::encode(
                    subject_manager.clone(),
                    jwt_header,
                    &verifiable_presentation_jwt,
                    did_method,
                )
                .await
                .map_err(|e| AppError::Error(format!("Failed to sign VP JWT: {e}")))?;

                StringOrObject::from(signed_vc_presentation_jwt_string)
            }
            Format::DcSdJwt => {
                let sd_jwt_vc = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string()
                    .parse::<SdJwtVc>()
                    .map_err(|err| AppError::Error(format!("Failed to parse SD-JWT VC: {err}")))?;

                let subject_wrapper = SubjectWrapper {
                    subject: subject_manager.clone(),
                    preferred_did_method: did_method.to_string(),
                };

                let Some(RequiredKeyBinding::Kid(cnf_kid)) = sd_jwt_vc.claims().cnf.as_ref() else {
                    return Err(AppError::Error("Unsupported `cnf` claim in SD-JWT VC".to_string()));
                };

                let cnf_jwk = subject_manager
                    .resolve_public_key(cnf_kid)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to resolve `cnf` key from DID URL: {e}")))?;

                let algorithm = cnf_jwk
                    .alg()
                    .ok_or_else(|| AppError::Error("JWK missing `alg` parameter".to_string()))?;

                let key_binding_jwt = KeyBindingJwtBuilder::new()
                    .iat(Utc::now().timestamp())
                    .aud(verifier_audience.to_string())
                    .nonce(required_nonce.to_string())
                    .finish(&sd_jwt_vc, &Sha256Hasher::new(), algorithm, &subject_wrapper)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to build KeyBindingJwt: {e}")))?;

                let (mut sd_jwt_vc, _) = sd_jwt_vc
                    .into_presentation(&Sha256Hasher::new())
                    .map_err(|err| AppError::Error(format!("Failed to create SD-JWT presentation: {err}")))?
                    // TODO: Conceal claims
                    .finish();

                sd_jwt_vc.attach_key_binding_jwt(key_binding_jwt);

                StringOrObject::from(sd_jwt_vc.to_string())
            }
            Format::VcSdJwt => {
                let vcdm2_sd_jwt = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .to_string();

                let vcdm2_sd_jwt = vcdm2_sd_jwt
                    .parse::<SdJwt>()
                    .map_err(|err| AppError::Error(format!("Failed to parse VCDM 2.0 SD-JWT: {err}")))?;

                let id = VcDataUrl::parse(&format!("data:application/vc+sd-jwt,{vcdm2_sd_jwt}"))
                    .map_err(|e| AppError::Error(format!("Failed to parse VcDataUrl: {e}")))?;
                let enveloped_credential = EnvelopedVc::new(id);

                let mut properties = IotaObject::default();
                properties.insert("iss".to_string(), full_did.clone().into());
                properties.insert("aud".to_string(), verifier_audience.clone().into());
                properties.insert("nonce".to_string(), required_nonce.clone().into());
                properties.insert("iat".to_string(), Utc::now().timestamp().into());
                properties.insert(
                    "exp".to_string(),
                    (Utc::now() + Duration::minutes(10)).timestamp().into(),
                );

                let presentation = Presentation::builder(holder_url.clone(), properties)
                    .credential(enveloped_credential)
                    .build_v2()
                    .map_err(|e| AppError::Error(format!("Failed to build Presentation: {e}")))?;

                let Some(RequiredKeyBinding::Kid(cnf_kid)) = vcdm2_sd_jwt.claims().cnf.as_ref() else {
                    return Err(AppError::Error(
                        "Unsupported `cnf` claim in VCDM 2.0 SD-JWT".to_string(),
                    ));
                };

                let cnf_jwk = subject_manager
                    .resolve_public_key(cnf_kid)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to resolve `cnf` key from DID URL: {e}")))?;

                let algorithm = cnf_jwk
                    .alg()
                    .ok_or_else(|| AppError::Error("JWK missing `alg` parameter".to_string()))?;

                let jwt_header = Header {
                    alg: Algorithm::from_str(algorithm).unwrap(),
                    kid: Some(cnf_kid.clone()),
                    typ: Some("JWT".to_string()),
                    ..Default::default()
                };

                let signed_vcdm2_sd_jwt_presentation_jwt_string =
                    jwt::encode(subject_manager.clone(), jwt_header, &presentation, did_method)
                        .await
                        .map_err(|e| AppError::Error(format!("Failed to sign VP JWT: {e}")))?;

                StringOrObject::from(signed_vcdm2_sd_jwt_presentation_jwt_string)
            }
            _ => {
                return Err(AppError::InvalidCredentialFormatError);
            }
        };

        let presentations = Presentations::try_new(vec![presentation_format_item])
            .map_err(|e| AppError::Error(format!("Failed to create presentations: {e}")))?;

        builder = builder.add_presentations(credential_query_id, presentations);
    }

    // Build and validate the VP token
    builder
        .build()
        .map_err(|e| AppError::Error(format!("Failed to build VpToken: {e:?}",)))
}
