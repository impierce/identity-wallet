use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action},
        connections::Connection,
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
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc::oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vc::oid4vci::credential_format_profiles::{
    w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, Credential, CredentialFormats,
};
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

        let verifiable_credentials: Vec<Credential<JwtVcJson>> = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .iter()
            .filter_map(|verifiable_credential_record| {
                let share_credential = match &verifiable_credential_record.verifiable_credential {
                    CredentialFormats::JwtVcJson(jwt_vc_json) => credential_uuids
                        .contains(&verifiable_credential_record.display_credential.id.parse().unwrap())
                        .then_some(jwt_vc_json.to_owned()),
                    _ => unimplemented!(),
                };

                if share_credential.is_some() {
                    history_credentials.push(HistoryCredential::from_credential(verifiable_credential_record));
                }

                share_credential
            })
            .collect();

        let presentation_submission = create_presentation_submission(
            &oid4vp_authorization_request.body.extension.presentation_definition,
            verifiable_credentials
                .iter()
                .map(|vc| get_unverified_jwt_claims(&vc.credential))
                .collect(),
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
                    .credential
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

        let connection_time = chrono::Utc::now().to_rfc3339();

        let (client_name, _logo_uri, connection_url) =
            get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request)
                .map_err(|_| MissingAuthorizationRequestParameterError("connection_url"))?;

        let mut connections = state.connections.clone();
        let result = connections
            .iter_mut()
            .find(|connection| connection.url == connection_url && connection.client_name == client_name)
            .map(|connection| {
                connection.last_interacted = connection_time.clone();
            });

        let connection_id = Connection::create_connection_id(&client_name);

        if result.is_none() {
            connections.push(Connection {
                id: connection_id.to_string(),
                client_name: client_name.to_string(),
                url: connection_url,
                verified: false,
                first_interacted: connection_time.clone(),
                last_interacted: connection_time.clone(),
            })
        };

        state.connections = connections;

        // History
        state.history.push(HistoryEvent {
            connection_name: client_name,
            date: connection_time,
            event_type: EventType::CredentialsShared,
            connection_id,
            credentials: history_credentials,
        });

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
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = oid4vp_authorization_request
        .body
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(oid4vp_authorization_request
        .body
        .extension
        .client_metadata
        .as_ref()
        .map(|client_metadata| {
            let client_name = client_metadata
                .client_name
                .as_ref()
                .cloned()
                .unwrap_or(connection_url.to_string());
            let logo_uri = client_metadata.logo_uri.as_ref().map(|logo_uri| logo_uri.to_string());

            (client_name, logo_uri, connection_url.clone())
        })
        // Otherwise use the connection_url as the client_name.
        .unwrap_or((connection_url.to_string(), None, connection_url)))
}
