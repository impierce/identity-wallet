use crate::{
    error::AppError::{self, *},
    get_unverified_jwt_claims, reducer,
    state::{
        actions::{listen, Action, ActionTrait},
        connections::{get_oid4vp_client_name_and_logo_uri, Connection, ConnectionRequest},
        history_event::{EventType, HistoryCredential, HistoryEvent},
        user_prompt::CurrentUserPrompt,
        AppState, Reducer,
    },
};
use identity_credential::{credential::Jwt, presentation::Presentation};
use log::info;
use oid4vc::oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vc::oid4vci::credential_format_profiles::w3c_verifiable_credentials::jwt_vc_json::JwtVcJson;
use oid4vc::oid4vci::credential_format_profiles::{Credential, CredentialFormats};
use oid4vc::oid4vp::oid4vp;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Action to authenticate the selected credentials.
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "bindings/actions/CredentialsSelected.ts")]
pub struct CredentialsSelected {
    #[ts(type = "Array<string>")]
    pub credential_uuids: Vec<uuid::Uuid>,
}

#[typetag::serde(name = "[Authenticate] Credentials selected")]
impl ActionTrait for CredentialsSelected {
    fn reducers<'a>(&self) -> Vec<Reducer<'a>> {
        vec![reducer!(handle_oid4vp_authorization_request)]
    }
}

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

        // TODO: This is a HORRIBLE solution to determine the connection_id by the non-unique "issuer name".
        // It is a TEMPORARY solution and should only be used in DEMO environments,
        // since we currently lack a unique identitfier to distinguish connections.
        let connection_id = base64::encode_config(&client_name, base64::URL_SAFE);

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
            connection_id: Some(connection_id),
            credentials: history_credentials,
        });

        state.current_user_prompt = Some(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });
    }

    Ok(state)
}
