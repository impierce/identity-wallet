use crate::state::core_utils::helpers::get_unverified_jwt_claims;
use crate::state::credentials::reducers::self_issue_credential::SubjectWrapper;
use crate::state::credentials::Sha256Hasher;
use crate::subject::Subject;
use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ConnectionRequest,
        },
        credentials::actions::credentials_selected::CredentialsSelected,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use chrono::{Duration, Utc};
use identity_core::common::Object as IotaObject;
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_iota::did::CoreDID;
use log::{info, warn};
use oid4vc::oid4vc_core::types::string_or_object::StringOrObject;
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::oid4vc_core::{jwt, Sign, Subject as _};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use oid4vc::oid4vp::token::vp_token::Presentations;
use oid4vc::oid4vp::token::vp_token_validator::DecodedPresentations;
use oid4vc::oid4vp::{
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
use sd_jwt::{KeyBindingJwtBuilder, RequiredKeyBinding};
use serde_json::Value;
use std::sync::Arc;

// Sends the authorization response including the verifiable credentials.
pub async fn handle_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("handle_presentation_request");

    if let Some(credential_uuids) = listen::<CredentialsSelected>(action).map(|payload| payload.credential_uuids) {
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

        let oid4vp_authorization_request = if let ConnectionRequest::OID4VP(oid4vp_authorization_request) =
            serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap()
        {
            oid4vp_authorization_request
        } else {
            return Err(AppError::Error("Expected OID4VP Authorization Request".to_string()));
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
                            .ok_or(AppError::InvalidCredentialFormatError("Failed to get SD-JWT VC as string from Verifiable Credential Record".to_string()))?
                            .to_string();

                        let sd_jwt_vc = sd_jwt_vc_string
                            .parse::<identity_credential::sd_jwt_vc::SdJwtVc>()
                            .map_err(|e| AppError::Error(format!("Failed to parse stored SD-JWT VC: {e}")))?;

                        let disclosed_object = sd_jwt_vc.into_disclosed_object(&Sha256Hasher::new()).map_err(|e| {
                            AppError::Error(format!("Failed to get disclosed object from SD-JWT VC: {e}"))
                        })?;
                        serde_json::json!(disclosed_object)
                    } else if verifiable_credential_record.display_credential.format == CredentialFormats::JwtVcJson(())
                    {
                        get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
                            .unwrap_or_default()
                            .get("vc")
                            .cloned()
                            .unwrap_or_else(|| {
                                log::debug!(
                                    "JWT-VC-JSON missing 'vc' claim or it's not a valid JSON value: {:?}",
                                    verifiable_credential_record.verifiable_credential
                                );
                                serde_json::json!({})
                            })
                    } else {
                        log::warn!(
                            "Unsupported format {:?} for evaluation. Attempting to get unverified JWT claims.",
                            verifiable_credential_record.display_credential.format
                        );
                        get_unverified_jwt_claims(&verifiable_credential_record.verifiable_credential)
                            .unwrap_or_default()
                    };

                    let Some(object) = credential_data.as_object() else {
                        warn!("Credential data is not a JSON object: {credential_data}");
                        continue;
                    };
                    let Ok(presentations) = DecodedPresentations::try_new(vec![object.clone()]) else {
                        warn!("Failed to create DecodedPresentations for credential data: {credential_data}");
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
                        AppError::Error(
                            "Provider manager does not contain any supported signing algorithms".to_string(),
                        )
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

        let response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        info!("response generated: {response:?}");

        #[cfg(not(feature = "test_utils"))]
        if provider_manager.send_response(&response).await.is_err() {
            info!("failed to send response");
            return Err(SendAuthorizationResponseError);
        }
        info!("response successfully sent");

        let OID4VPClientMetadata {
            client_name,
            logo_uri,
            connection_url,
            client_id,
        } = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

        let did = CoreDID::parse(client_id).ok();

        let mut connections = state.connections;
        let previously_connected = connections.contains(connection_url.as_str(), &client_name);
        let connection = connections.update_or_insert(&connection_url, &client_name, did);

        let file_name = match logo_uri {
            Some(logo_uri) => hash(logo_uri.as_str()),
            None => "_".to_string(),
        };
        persist_asset(&file_name, &connection.id).ok();

        // History
        let mut history = state.history;
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

pub struct OID4VPClientMetadata {
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub connection_url: String,
    pub client_id: String,
}

// TODO: move this functionality to the oid4vc-manager crate.
/// Returns (client_name, logo_uri, connection_url, client_id)
pub fn get_oid4vp_client_name_and_logo_uri(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> OID4VPClientMetadata {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = oid4vp_authorization_request.body.uri.uri().clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    let client_id = oid4vp_authorization_request.body.client_id.clone();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    match &oid4vp_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name,
            logo_uri,
            extension: _,
            other: _,
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
            let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            Some(OID4VPClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.to_string(),
                client_id: client_id.clone(),
            })
        }
        // TODO: support `client_metadata_uri`
        ClientMetadataResource::ClientMetadataUri(_) => None,
    }
    // Otherwise use the connection_url as the client_name.
    .unwrap_or(OID4VPClientMetadata {
        client_name: connection_url.to_string(),
        logo_uri: None,
        connection_url: connection_url.to_string(),
        client_id,
    })
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

    for (credential_query_from_dcql, vc_value) in selected_verifiable_credentials {
        let credential_query_id = credential_query_from_dcql.id.clone();
        let format_from_query = credential_query_from_dcql.format;

        let presentation_format_item = match format_from_query {
            Format::JwtVcJson => {
                let raw_vc_jwt_string = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError("Failed to get VC JWT as string from Verifiable Credential Record".to_string()))?
                    .to_string();

                let vc_jwt: Jwt = raw_vc_jwt_string.into();

                let full_did = subject_manager
                    .identifier(did_method, signing_algorithm)
                    .await
                    .map_err(|e| AppError::Error(format!("Failed to get DID identifier: {e}")))?;

                let holder_url: Url =
                    Url::parse(&full_did).map_err(|e| AppError::Error(format!("Failed to parse DID as URL: {e}")))?;

                let presentation = Presentation::builder(holder_url, IotaObject::default())
                    .credential(vc_jwt)
                    .build()
                    .map_err(AppError::PresentationBuilderError)?;

                let verifiable_presentation_jwt = VerifiablePresentationJwt::builder()
                    .iss(full_did.clone())
                    .sub(full_did)
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
            // TODO: Support `vc+sd-jwt` format
            Format::DcSdJwt => {
                let sd_jwt_vc = vc_value
                    .as_str()
                    .ok_or(AppError::InvalidCredentialFormatError("Failed to get SD-JWT VC as string from Verifiable Credential Record".to_string()))?
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
            _ => {
                return Err(AppError::InvalidCredentialFormatError("Unsupported credential format".to_string()));
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
