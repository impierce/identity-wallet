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
};

use identity_credential::{
    credential::Jwt,
    presentation::Presentation,
    sd_jwt_v2::Sha256Hasher,
    sd_jwt_vc::{SdJwtVc, SdJwtVcPresentationBuilder},
};
use identity_iota::did::CoreDID;
use log::info;
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::oid4vp::oid4vp::PresentationInputType;
use oid4vc::oid4vp::oid4vp::OID4VP;
use oid4vc::{
    oid4vc_manager::managers::presentation::create_presentation_submission,
    oid4vci::credential_format_profiles::CredentialFormats,
};
use oid4vc::{oid4vc_manager::managers::presentation::create_sd_jwt_presentation_submission, oid4vp::oid4vp};
use uuid::Uuid;

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

        let oid4vp_authorization_request =
            match serde_json::from_value(serde_json::json!(state.core_utils.active_connection_request)).unwrap() {
                ConnectionRequest::OID4VP(oid4vp_authorization_request) => oid4vp_authorization_request,
                ConnectionRequest::SIOPv2(_) => unreachable!(),
            };

        let mut history_credentials = vec![];

        let verifiable_credentials: Vec<(CredentialFormats, serde_json::Value)> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .iter()
            .filter_map(|verifiable_credential_record| {
                info!("Verifiable Credential Record: {:#?}", verifiable_credential_record);

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

        // Divide all Verifiable Credentials into SD-JWT VC and JWT VC JSONs.
        let (sd_jwt_vc_credentials, jwt_vc_json_credentials) = verifiable_credentials.into_iter().try_fold(
            (Vec::new(), Vec::new()),
            |(mut sd_jwt_vc_credentials, mut jwt_vc_json_credentials), (format, verifiable_credential)| {
                match format {
                    CredentialFormats::VcSdJwt(()) => sd_jwt_vc_credentials.push(verifiable_credential),
                    CredentialFormats::JwtVcJson(()) => jwt_vc_json_credentials.push(verifiable_credential),
                    _ => return Err(AppError::InvalidCredentialFormatError),
                }
                Ok((sd_jwt_vc_credentials, jwt_vc_json_credentials))
            },
        )?;

        info!("`jwt_vc_json` Credentials: {:#?}", jwt_vc_json_credentials);
        info!("`sd_jwt_vc` Credentials: {:#?}", sd_jwt_vc_credentials);

        // Create the Authorization Response Input.
        let authorization_response_input = match (sd_jwt_vc_credentials.len(), jwt_vc_json_credentials.len()) {
            (0, 0) => {
                return Err(AppError::Error(
                    "No credentials available to fulfill the request".to_string(),
                ));
            }

            // If multiple presentations are provided, this means that the `vp_token` in the Authorization
            // Response will be a sequence which cannot be serialized into a x-www-form-urlencoded string by `reqwest`.
            // See: https://github.com/nox/serde_urlencoded/issues/75#issuecomment-648257888
            (sd_jwt_vc_count, 0) if sd_jwt_vc_count > 1 => {
                return Err(AppError::Error(
                    "Sending multiple SD-JWT VCs is not supported".to_string(),
                ))
            }
            (sd_jwt_vc_count, jwt_vc_json_count) if (sd_jwt_vc_count > 0 && jwt_vc_json_count > 0) => {
                return Err(AppError::Error(
                    "Sending multiple W3C Verifiable Presentations and SD-JWT VCs is not supported".to_string(),
                ))
            }
            (1, 0) => {
                let sd_jwt_vc = sd_jwt_vc_credentials
                    .first()
                    .and_then(serde_json::Value::as_str)
                    .and_then(|sd_jwt_vc| SdJwtVc::parse(sd_jwt_vc).ok())
                    .ok_or(AppError::Error("Failed to parse SD-JWT VC".to_string()))?;

                let (sd_jwt_vc, _) = SdJwtVcPresentationBuilder::new(sd_jwt_vc, &Sha256Hasher::new())
                    .map_err(|e| {
                        AppError::Error(format!(
                            "Failed to create SD-JWT VC Presentation Builder for SD-JWT VC: {e}",
                        ))
                    })?
                    // TODO: Allow concealing claims (selective disclosure)
                    // .conceal("address")
                    // TODO: Implement Key Binding
                    // .attach_key_binding_jwt(kb_jwt)
                    .finish()
                    .map_err(|e| AppError::Error(format!("Failed to attach KeyBindingJwt to SD-JWT VC: {e}")))?;

                let presentation_submission = create_sd_jwt_presentation_submission(
                    Uuid::new_v4().to_string(),
                    &oid4vp_authorization_request.body.extension.presentation_definition,
                    &[serde_json::json!(sd_jwt_vc
                        .clone()
                        .into_disclosed_object(&Sha256Hasher::new())
                        .map_err(|e| AppError::Error(format!(
                            "Failed to create Disclosed Object for SD-JWT VC: {e}"
                        )))?)],
                )
                .map_err(|e| {
                    AppError::Error(format!("Failed to create Presentation Submission for SD-JWT VC: {e}",))
                })?;

                oid4vp::AuthorizationResponseInput {
                    verifiable_presentation_input: PresentationInputType::SdJwtVc(sd_jwt_vc.to_string()),
                    presentation_submission,
                }
            }
            (0, jwt_vc_json_count) if jwt_vc_json_count > 0 => {
                let presentation_submission = create_presentation_submission(
                    Uuid::new_v4().to_string(),
                    &oid4vp_authorization_request.body.extension.presentation_definition,
                    &jwt_vc_json_credentials
                        .iter()
                        .map(get_unverified_jwt_claims)
                        .collect::<Result<Vec<_>, _>>()?,
                )
                .map_err(PresentationSubmissionError)?;

                let identity_manager = state_guard
                    .identity_manager
                    .as_ref()
                    .ok_or(MissingManagerError("identity"))?;

                let did_method = state
                    .profile_settings
                    .preferred_did_methods
                    .first()
                    .ok_or(AppError::Error("Default DID method is missing".to_string()))?;

                // Get the algorithm from the client_metadata if it exists
                let algorithm = identity_manager
                    .provider_manager
                    .get_matching_signing_algorithm(&oid4vp_authorization_request)
                    .await
                    // If there is no matching algorithm, use the Provider's first supported algorithm.
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

                let subject_did = identity_manager
                    .subject
                    .identifier(did_method, algorithm)
                    .await
                    .map_err(AppError::OID4VCSubjectIdentifierError)?;

                let mut presentation_builder =
                    Presentation::builder(subject_did.parse().map_err(|_| DidParseError)?, Default::default());
                for jwt_vc_json in jwt_vc_json_credentials {
                    presentation_builder = presentation_builder.credential(Jwt::from(
                        jwt_vc_json.as_str().ok_or(InvalidCredentialFormatError)?.to_string(),
                    ));
                }

                oid4vp::AuthorizationResponseInput {
                    verifiable_presentation_input: PresentationInputType::Presentation(Box::new(
                        presentation_builder.build().map_err(PresentationBuilderError)?,
                    )),
                    presentation_submission,
                }
            }
            _ => {
                return Err(AppError::Error(
                    "Invalid combination of SD-JWT VC and JWT VC JSONs".to_string(),
                ));
            }
        };

        info!("get the provider_manager");

        info!("generating response");
        let response = provider_manager
            .generate_response(&oid4vp_authorization_request, authorization_response_input)
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
}

// TODO: move this functionality to the oid4vc-manager crate.
/// Returns (client_name, logo_uri, connection_url, client_id)
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
