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

use identity_credential::{credential::Jwt, presentation::Presentation};
use log::info;
use oid4vc::oid4vc_core::{
    authorization_request::{AuthorizationRequest, Object},
    client_metadata::ClientMetadataResource,
};
use oid4vc::oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vc::oid4vp::oid4vp;
use oid4vc::oid4vp::oid4vp::OID4VP;

// Sends the authorization response including the verifiable credentials.
pub async fn handle_oid4vp_authorization_request(mut state: AppState, action: Action) -> Result<AppState, AppError> {
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

        let mut history_credentials = Vec::new();

        let verifiable_credentials: Vec<serde_json::Value> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .iter()
            .filter_map(|verifiable_credential_record| {
                let share_credential = credential_uuids
                    .contains(&verifiable_credential_record.display_credential.id.parse().unwrap())
                    .then_some(verifiable_credential_record.verifiable_credential.clone());

                if share_credential.is_some() {
                    history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
                }

                share_credential
            })
            .collect();

        println!("verifiable_credentials: {:?}", verifiable_credentials);

        let presentation_submission = create_presentation_submission(
            &oid4vp_authorization_request.body.extension.presentation_definition,
            &verifiable_credentials
                .iter()
                .map(get_unverified_jwt_claims)
                .collect::<Vec<_>>(),
        )
        .map_err(PresentationSubmissionError)?;

        info!("get the subject did");

        let subject_did = state
            .profile_settings
            .profile
            .as_ref()
            .ok_or(MissingStateParameterError("active profile"))?
            .primary_did
            .clone();

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

        let verifiable_presentation: Presentation<Jwt, _> =
            presentation_builder.build().map_err(PresentationBuilderError)?;

        info!("get the provider_manager");

        info!("generating response");
        let response = provider_manager
            .generate_response(
                &oid4vp_authorization_request,
                oid4vp::AuthorizationResponseInput {
                    verifiable_presentation,
                    presentation_submission,
                },
            )
            .map_err(GenerateAuthorizationResponseError)?;
        info!("response generated: {:?}", response);

        if provider_manager.send_response(&response).await.is_err() {
            info!("failed to send response");
            return Err(SendAuthorizationResponseError);
        }
        info!("response successfully sent");

        let (client_name, logo_uri, connection_url) =
            get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request);

        let previously_connected = state.connections.contains(connection_url.as_str(), &client_name);
        let mut connections = state.connections;
        let connection = connections.update_or_insert(&connection_url, &client_name);

        let file_name = match logo_uri {
            Some(logo_uri) => hash(logo_uri.as_str()),
            None => "_".to_string(),
        };
        persist_asset(&file_name, &connection.id).ok();

        // History
        if !previously_connected {
            // Only add a `ConnectionAdded` event if the connection was not previously connected.
            state.history.push(HistoryEvent {
                connection_name: connection.name.clone(),
                event_type: EventType::ConnectionAdded,
                connection_id: connection.id.clone(),
                date: connection.last_interacted.clone(),
                credentials: vec![],
            });
        }
        state.history.push(HistoryEvent {
            connection_name: connection.name.clone(),
            event_type: EventType::CredentialsShared,
            connection_id: connection.id.clone(),
            date: connection.last_interacted.clone(),
            credentials: history_credentials,
        });

        state.connections = connections;
        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });
    }

    Ok(state)
}

// Helper

// TODO: move this functionality to the oid4vc-manager crate.
pub fn get_oid4vp_client_name_and_logo_uri(
    oid4vp_authorization_request: &AuthorizationRequest<Object<OID4VP>>,
) -> (String, Option<String>, String) {
    // Get the connection url from the redirect url host (or use the redirect url if it does not
    // contain a host).
    let redirect_uri = oid4vp_authorization_request.body.redirect_uri.clone();
    let connection_url = redirect_uri.host_str().unwrap_or(redirect_uri.as_str());

    // Get the client_name and logo_uri from the client_metadata if it exists.
    oid4vp_authorization_request
        .body
        .extension
        .client_metadata
        .as_ref()
        .and_then(|client_metadata| match client_metadata {
            ClientMetadataResource::ClientMetadata {
                client_name, logo_uri, ..
            } => {
                let client_name = client_name.as_ref().cloned().unwrap_or(connection_url.to_string());
                let logo_uri = logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());
                Some((client_name, logo_uri, connection_url.to_string()))
            }
            _ => None,
        })
        // Otherwise use the connection_url as the client_name.
        .unwrap_or((connection_url.to_string(), None, connection_url.to_string()))
}
