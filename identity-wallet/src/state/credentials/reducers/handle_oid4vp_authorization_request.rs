use crate::{
    error::AppError::{self, *},
    persistence::{hash, persist_asset},
    state::{
        actions::{listen, Action},
        core_utils::{
            helpers::get_unverified_jwt_claims,
            history_event::{EventType, HistoryCredential, HistoryEvent},
            ConnectionRequest,
        },
        credentials::actions::credentials_selected::CredentialsSelected,
        user_prompt::CurrentUserPrompt,
        AppState,
    },
    subject::SubjectWrapper,
};

use identity_credential::{
    credential::Jwt,
    presentation::Presentation,
    sd_jwt_v2::{KeyBindingJwt, Sha256Hasher},
    sd_jwt_vc::{SdJwtVc, SdJwtVcPresentationBuilder},
};
use identity_iota::{core::Timestamp, did::CoreDID};
use jsonwebtoken::Algorithm;
use log::info;
use oid4vc::oid4vp::{authorization_request::ClientMetadataParameters, oid4vp::PresentationInputType};
use oid4vc::{
    oid4vc_core::{
        authorization_request::{AuthorizationRequest, Object},
        client_metadata::ClientMetadataResource,
    },
    oid4vp::{ClaimFormatDesignation, ClaimFormatProperty},
};
use oid4vc::{
    oid4vc_manager::managers::presentation::create_presentation_submission,
    oid4vci::credential_format_profiles::CredentialFormats,
};
use oid4vc::{oid4vc_manager::managers::presentation::create_sd_jwt_presentation_submission, oid4vp::oid4vp};
use oid4vc::{oid4vc_manager::managers::presentation::merge_submissions, oid4vp::oid4vp::OID4VP};

// Sends the authorization response including the verifiable credentials.
pub async fn handle_oid4vp_authorization_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("handle_presentation_request");

    if let Some(credential_uuids) = listen::<CredentialsSelected>(action).map(|payload| payload.credential_uuids) {
        let state_guard = state.core_utils.managers.lock().await;

        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;
        let provider_manager = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .provider_manager;
        let subject = state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .subject
            .clone();

        let oid4vp_authorization_request =
            match serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap() {
                ConnectionRequest::OID4VP(oid4vp_authorization_request) => oid4vp_authorization_request,
                ConnectionRequest::SIOPv2(_) => unreachable!(),
            };

        let mut history_credentials = Vec::new();

        let verifiable_credentials: Vec<(CredentialFormats, serde_json::Value)> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .iter()
            .filter_map(|verifiable_credential_record| {
                info!("verifiable_credential_record: {:#?}", verifiable_credential_record);

                let share_credential = credential_uuids
                    .contains(&verifiable_credential_record.display_credential.id.parse().unwrap())
                    .then_some(verifiable_credential_record.verifiable_credential.clone());

                if share_credential.is_some() {
                    history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
                }

                share_credential.map(|share_credential| {
                    (
                        verifiable_credential_record.display_credential.format.clone(),
                        share_credential,
                    )
                })
            })
            .collect();

        let subject_syntax_type = state
            .profile_settings
            .preferred_did_methods
            .first()
            .unwrap()
            .to_string();

        let subject_wrapper = SubjectWrapper {
            subject,
            subject_syntax_type,
        };

        let sd_jwt_vcs: Vec<serde_json::Value> = verifiable_credentials
            .iter()
            .filter_map(|(format, vc)| {
                if format == &CredentialFormats::VcSdJwt(()) {
                    Some(vc.clone())
                } else {
                    None
                }
            })
            .collect();

        let now = Timestamp::now_utc();

        let mut verifiable_presentation_input = vec![];
        let mut presentation_submissions = vec![];

        info!(
            "Authorization Request: {}",
            serde_json::to_string_pretty(&oid4vp_authorization_request).unwrap()
        );

        let nonce = oid4vp_authorization_request.body.extension.nonce.clone();
        let aud = oid4vp_authorization_request.body.client_id.clone();

        let OID4VPClientMetadata { algorithm, .. } = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

        for sd_jwt_vc in sd_jwt_vcs {
            let sd_jwt_vc = sd_jwt_vc
                .as_str()
                .and_then(|sd_jwt_vc| SdJwtVc::parse(sd_jwt_vc).ok())
                .ok_or(AppError::Error("Failed to parse SD-JWT VC".to_string()))?;

            let alg = sd_jwt_vc
                .header()
                .get("alg")
                .and_then(|alg| alg.as_str())
                .map(ToString::to_string)
                .unwrap_or(serde_json::json!(algorithm).as_str().unwrap().to_string());
            let kb_jwt = KeyBindingJwt::builder()
                .iat(now.to_unix())
                .aud(aud.clone())
                .nonce(nonce.clone())
                .finish(&sd_jwt_vc, &Sha256Hasher::new(), &alg, &subject_wrapper)
                .await
                .map_err(|e| AppError::Error(format!("Failed to create KeyBindingJwt for SD-JWT VC: {:?}", e)))?;

            let (sd_jwt_vc, _) = SdJwtVcPresentationBuilder::new(sd_jwt_vc, &Sha256Hasher::new())
                .map_err(|e| {
                    AppError::Error(format!(
                        "Failed to create SD-JWT VC Presentation Builder for SD-JWT VC: {:?}",
                        e
                    ))
                })?
                .attach_key_binding_jwt(kb_jwt)
                .finish()
                .map_err(|e| AppError::Error(format!("Failed to attach KeyBindingJwt to SD-JWT VC: {:?}", e)))?;

            let presentation_submission = create_sd_jwt_presentation_submission(
                &oid4vp_authorization_request.body.extension.presentation_definition,
                &[serde_json::json!(sd_jwt_vc
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    // FIX THIS
                    .unwrap())],
            )
            .map_err(|e| {
                AppError::Error(format!(
                    "Failed to create Presentation Submission for SD-JWT VC: {:?}",
                    e
                ))
            })?;

            verifiable_presentation_input.push(PresentationInputType::Signed(sd_jwt_vc.to_string()));
            presentation_submissions.push(presentation_submission);
        }

        let verifiable_credentials: Vec<serde_json::Value> = verifiable_credentials
            .iter()
            .filter_map(|(format, vc)| {
                if format == &CredentialFormats::JwtVcJson(()) {
                    Some(vc.clone())
                } else {
                    None
                }
            })
            .collect();

        info!("verifiable_credentials: {:#?}", verifiable_credentials);

        if !verifiable_credentials.is_empty() {
            let presentation_submission = create_presentation_submission(
                &oid4vp_authorization_request.body.extension.presentation_definition,
                &verifiable_credentials
                    .iter()
                    .map(get_unverified_jwt_claims)
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .map_err(PresentationSubmissionError)?;

            presentation_submissions.push(presentation_submission);

            info!("get the subject did");

            let identity_manager = state_guard
                .identity_manager
                .as_ref()
                .ok_or(MissingManagerError("identity"))?;

            let OID4VPClientMetadata { algorithm, .. } =
                get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

            let subject_did = identity_manager
                .subject
                .identifier(state.profile_settings.preferred_did_methods.first().unwrap(), algorithm)
                .await
                .expect("No default DID method");

            let mut presentation_builder =
                Presentation::builder(subject_did.parse().map_err(|_| DidParseError)?, Default::default());
            for verifiable_credential in verifiable_credentials {
                presentation_builder = presentation_builder.credential(Jwt::from(
                    verifiable_credential
                        .as_str()
                        .ok_or(InvalidCredentialFormatError)?
                        .to_string(),
                ));
            }

            verifiable_presentation_input.push(PresentationInputType::Unsigned(
                presentation_builder.build().map_err(PresentationBuilderError)?,
            ));
        }

        let presentation_submission = if presentation_submissions.len() > 1 {
            merge_submissions(presentation_submissions)
        } else {
            presentation_submissions.pop().ok_or(AppError::Error(
                "Failed to create a Presentation Submission".to_string(),
            ))?
        };

        info!("Verifiable Presentation Input: {:#?}", verifiable_presentation_input);

        info!("get the provider_manager");

        info!("generating response");
        let response = provider_manager
            .generate_response(
                &oid4vp_authorization_request,
                oid4vp::AuthorizationResponseInput {
                    verifiable_presentation_input,
                    presentation_submission,
                },
            )
            .await
            .map_err(GenerateAuthorizationResponseError)?;
        info!("response generated: {:?}", response);

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
            algorithm: _algorithm,
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

// Helper

pub struct OID4VPClientMetadata {
    pub client_name: String,
    pub logo_uri: Option<String>,
    pub connection_url: String,
    pub client_id: String,
    pub algorithm: Algorithm,
}

// TODO: move this functionality to the oid4vc-manager crate.
/// Returns (client_name, logo_uri, connection_url, client_id, algorithm)
pub fn get_oid4vp_client_name_and_logo_uri(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> OID4VPClientMetadata {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = oid4vp_authorization_request.body.redirect_uri.clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    let client_id = oid4vp_authorization_request.body.client_id.clone();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    match &oid4vp_authorization_request.body.extension.client_metadata {
        ClientMetadataResource::ClientMetadata {
            client_name,
            logo_uri,
            extension: ClientMetadataParameters { vp_formats },
            other: _,
        } => {
            let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
            let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            // TODO: These helper functions become more and more complicated. This functionality needs to be implemented
            // in oid4vc-manager soon.
            // Get the algorithm from the client_metadata if it exists or default to EdDSA.
            let algorithm = vp_formats
                .get(&ClaimFormatDesignation::JwtVcJson)
                .and_then(|claim_format_property| match claim_format_property {
                    ClaimFormatProperty::Alg(alg) => alg.first().cloned(),
                    // TODO: implement `ProofType`.
                    ClaimFormatProperty::ProofType(_) | _ => None,
                })
                .unwrap_or(Algorithm::EdDSA);

            Some(OID4VPClientMetadata {
                client_name,
                logo_uri,
                connection_url: connection_url.to_string(),
                client_id: client_id.clone(),
                algorithm,
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
        algorithm: Algorithm::EdDSA,
    })
}
