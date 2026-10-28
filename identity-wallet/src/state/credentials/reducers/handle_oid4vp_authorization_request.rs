use crate::state::connections::Connections;
use crate::state::core_utils::IdentityManager;
use crate::state::credentials::reducers::self_issue_credential::SubjectWrapper;
use crate::state::credentials::Sha256Hasher;
#[cfg(not(feature = "test_utils"))]
use crate::state::interaction_result::send_authorization_response;
use crate::stronghold::StrongholdManager;
use crate::subject::Subject;
use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ActiveFlow, CoreUtils, Oid4vciStage, PendingConnectionData,
        },
        credentials::{
            actions::credentials_selected::{CredentialSelection, CredentialsSelected},
            credential_query_matching::{credential_query_types_key, satisfies_credential_query},
        },
        interaction_result::{InteractionKind, InteractionOutcome, InteractionResult},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
};
use chrono::{Duration, Utc};
use identity_core::common::Object as IotaObject;
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_iota::credential::{EnvelopedVc, VcDataUrl};
use identity_iota::did::DID;
use log::{debug, info};
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc::oid4vc_core::types::string_or_object::StringOrObject;
use oid4vc::oid4vc_core::{jwt, Sign, Subject as _};
use oid4vc::oid4vp::token::vp_token::Presentations;
use oid4vc::oid4vp::{
    dcql::dcql_query::{CredentialQuery, Format},
    oid4vp::OID4VP,
    token::{
        verifiable_presentation_jwt::VerifiablePresentationJwt, vp_token::VpToken, vp_token_builder::VpTokenBuilder,
    },
};

use identity_credential::{credential::Jwt, presentation::Presentation};
use identity_iota::core::Url;
use identity_iota::did::DIDUrl;
use jsonwebtoken::Algorithm;
use jsonwebtoken::Header;
use sd_jwt::{KeyBindingJwtBuilder, RequiredKeyBinding, SdJwt};
use serde_json::Value;
use std::str::FromStr as _;
use std::sync::Arc;
use uuid::Uuid;

/// The ids of the shared credentials per credential types key, as remembered on the connection.
pub type LastShared = Vec<(String, Vec<String>)>;

/// Handles the non-interactive `CredentialsSelected` action, which is triggered by accepting the `ShareCredentials` prompt set by `read_oid4vp_authorization_request`.
/// Sends the authorization response including the verifiable credentials.
/// This fn completes the OID4VP authorization flow, therefore the active flow and pending connection data are cleared at the end and the user is redirected to the "me" page.
#[tracing::instrument(skip_all, err)]
pub async fn handle_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(selections) = listen::<CredentialsSelected>(action)
        .and_then(|payload| (!payload.is_interactive).then_some(payload.selections))
    {
        info!(
            "Responding to OID4VP authorization request with {} credential(s)",
            selections
                .iter()
                .map(|selection| selection.credential_uuids.len())
                .sum::<usize>()
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

        let (vp_token_payload, history_credentials, last_shared) =
            build_oid4vp_vp_token_and_history_credentials(&state, stronghold_manager, identity_manager, selections)
                .await?;

        let response = provider_manager
            .generate_response(&oid4vp_authorization_request, vp_token_payload)
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        debug!("Generated OID4VP authorization response: {response:?}");

        let pending_connection_data = state.core_utils.pending_connection_data.clone().ok_or_else(|| {
            AppError::Error("Expected pending connection data for OID4VP authorization request".to_string())
        })?;
        let interaction_result = |outcome| {
            Some(InteractionResult {
                kind: InteractionKind::Presentation,
                counterparty: pending_connection_data.client_metadata.client_name.clone(),
                outcome,
            })
        };

        #[cfg(not(feature = "test_utils"))]
        match send_authorization_response(&response).await {
            InteractionOutcome::Approved => {}
            // Nothing was shared: keep the flow so the user can try again.
            outcome @ InteractionOutcome::NotSent { .. } => {
                log::warn!("Failed to send OID4VP authorization response to verifier: {outcome:?}");
                drop(state_guard);
                return Ok(AppState {
                    interaction_result: interaction_result(outcome),
                    ..state
                });
            }
            // The verifier refused the presentation: end the flow without recording a share.
            outcome @ InteractionOutcome::Rejected { .. } => {
                log::warn!("Verifier rejected the OID4VP authorization response: {outcome:?}");
                drop(state_guard);
                return Ok(AppState {
                    core_utils: CoreUtils {
                        active_flow: None,
                        pending_connection_data: None,
                        ..state.core_utils
                    },
                    current_user_prompt: Some(CurrentUserPrompt::Redirect {
                        target: "me".to_string(),
                    }),
                    interaction_result: interaction_result(outcome),
                    ..state
                });
            }
        }
        info!("OID4VP presentation response accepted by verifier");

        let interaction_result = interaction_result(InteractionOutcome::Approved);
        let mut connections = state.connections;
        let mut history = state.history;

        update_history_and_connections(
            history_credentials,
            last_shared,
            &pending_connection_data,
            &mut connections,
            &mut history,
        )
        .await?;

        drop(state_guard);
        return Ok(AppState {
            connections,
            core_utils: CoreUtils {
                active_flow: None,
                pending_connection_data: None,
                ..state.core_utils
            },
            current_user_prompt: Some(CurrentUserPrompt::Redirect {
                target: "me".to_string(),
            }),
            history,
            interaction_result,
            ..state
        });
    }

    Ok(state)
}

#[tracing::instrument(skip_all, err)]
pub async fn build_oid4vp_vp_token_and_history_credentials(
    state: &AppState,
    stronghold_manager: &StrongholdManager,
    identity_manager: &IdentityManager,
    selections: Vec<CredentialSelection>,
) -> Result<(VpToken, Vec<HistoryCredential>, LastShared), AppError> {
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
    let mut last_shared = vec![];
    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let available_credentials_map: std::collections::HashMap<String, _> = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|record| (record.display_credential.id.clone(), record))
        .collect();

    let mut selected_verifiable_credentials: Vec<(CredentialQuery, Vec<Value>)> = Vec::new();
    for selection in selections {
        let requested_credential_query = dcql_query
            .credentials
            .iter()
            .find(|credential_query| credential_query.id.as_ref() == selection.query_id)
            .ok_or_else(|| AppError::Error(format!("Unknown credential query: {}", selection.query_id)))?;

        let mut verifiable_credentials = Vec::new();
        for user_selected_uuid in &selection.credential_uuids {
            let verifiable_credential_record = available_credentials_map
                .get(&user_selected_uuid.to_string())
                .ok_or_else(|| AppError::Error(format!("Selected credential not found: {user_selected_uuid}")))?;

            if !satisfies_credential_query(requested_credential_query, verifiable_credential_record) {
                return Err(AppError::Error(format!(
                    "Credential {user_selected_uuid} does not satisfy credential query {}",
                    selection.query_id
                )));
            }

            verifiable_credentials.push(verifiable_credential_record.verifiable_credential.clone());
            history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
        }

        if !verifiable_credentials.is_empty() {
            last_shared.push((
                credential_query_types_key(requested_credential_query),
                selection.credential_uuids.iter().map(Uuid::to_string).collect(),
            ));
            selected_verifiable_credentials.push((requested_credential_query.clone(), verifiable_credentials));
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

    Ok((vp_token_payload, history_credentials, last_shared))
}

#[tracing::instrument(skip_all)]
pub async fn update_history_and_connections(
    history_credentials: Vec<HistoryCredential>,
    last_shared: LastShared,
    pending_connection_data: &PendingConnectionData,
    connections: &mut Connections,
    history: &mut Vec<HistoryEvent>,
) -> Result<(), AppError> {
    let client_metadata = &pending_connection_data.client_metadata;
    let previously_connected = connections.contains(client_metadata.client_id.as_str());
    let connection = connections.update_last_interaction_or_insert_new(pending_connection_data);
    connection.last_shared_credentials.extend(last_shared);

    if let Some(logo_uri) = client_metadata.logo_uri.clone() {
        persist_asset(&hash(logo_uri.as_str()), &connection.id).ok();
    }

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
    selected_verifiable_credentials: Vec<(CredentialQuery, Vec<Value>)>,
    did_method: &str,
    subject_manager: &Arc<Subject>,
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
    signing_algorithm: Algorithm,
) -> Result<VpToken, AppError> {
    let verifier_audience = oid4vp_authorization_request.body.client_id.to_string();
    let required_nonce = oid4vp_authorization_request.body.extension.nonce.clone();

    let dcql_query = &oid4vp_authorization_request.body.extension.dcql_query;
    let mut builder = VpTokenBuilder::builder_dcql_query(dcql_query.clone());

    // TODO: Move most of this logic to `openid4vc` crates.
    for (credential_query_from_dcql, vc_values) in selected_verifiable_credentials {
        let mut presentation_format_items = Vec::new();
        for vc_value in vc_values {
            let presentation_format_item = match &credential_query_from_dcql.format {
                Format::JwtVcJson => {
                    let key_id = subject_manager
                        .key_id(did_method, signing_algorithm)
                        .await
                        .ok_or_else(|| {
                            AppError::Error(format!("Failed to get signing method ID for DID method {did_method}"))
                        })?;

                    let full_did = subject_manager
                        .identifier(did_method, signing_algorithm)
                        .await
                        .map_err(|e| AppError::Error(format!("Failed to get DID identifier: {e}")))?;

                    let holder_url: Url = Url::parse(&full_did)
                        .map_err(|e| AppError::Error(format!("Failed to parse DID as URL: {e}")))?;

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

                    // TODO: unify holder DID/key selection into a single helper.
                    // "Which DID/key do we sign with?" is answered differently at every call site: the `cnf`
                    // claim here, from `credentialSubject.id` when signing a DACT, and from the preferred
                    // DID method elsewhere. Each answer has to match the DID the credential was issued to, so they
                    // all need to behave identically. This should be unified into a single helper.
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

                    let holder_did = DIDUrl::parse(cnf_kid)
                        .map_err(|e| AppError::Error(format!("Failed to parse 'cnf' DID URL: {e}")))?
                        .did()
                        .to_string();
                    let holder_url = Url::parse(&holder_did)
                        .map_err(|e| AppError::Error(format!("Failed to parse holder URL: {e}")))?;

                    let id = VcDataUrl::parse(&format!("data:application/vc+sd-jwt,{vcdm2_sd_jwt}"))
                        .map_err(|e| AppError::Error(format!("Failed to parse VcDataUrl: {e}")))?;
                    let enveloped_credential = EnvelopedVc::new(id);

                    let mut properties = IotaObject::default();
                    properties.insert("iss".to_string(), holder_did.clone().into());
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

            presentation_format_items.push(presentation_format_item);
        }

        let presentations = Presentations::try_new(presentation_format_items)
            .map_err(|e| AppError::Error(format!("Failed to create presentations: {e}")))?;

        builder = builder.add_presentations(credential_query_from_dcql.id, presentations);
    }

    // Build and validate the VP token
    builder
        .build()
        .map_err(|e| AppError::Error(format!("Failed to build VpToken: {e:?}",)))
}
