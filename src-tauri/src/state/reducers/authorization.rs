use crate::{
    error::AppError::{self, *},
    get_unverified_jwt_claims,
    state::{actions::Action, user_prompt::CurrentUserPrompt, AppState, Connection},
};
use identity_credential::{credential::Jwt, presentation::Presentation};
use log::info;
use oid4vc_core::authorization_request::{AuthorizationRequest, Object};
use oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vci::credential_format_profiles::{
    w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, Credential, CredentialFormats,
};
use oid4vp::{evaluate_input, oid4vp::OID4VP};
use siopv2::siopv2::SIOPv2;

pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    OID4VP(Box<AuthorizationRequest<Object<OID4VP>>>),
}

// Reads the request url from the payload and validates it.
pub async fn read_authorization_request(state: &mut AppState, action: Action) -> Result<(), AppError> {
    info!("read_authorization_request");

    let authorization_request = match action {
        Action::ReadRequest { authorization_request } => authorization_request,
        _ => return Err(InvalidActionError { action }),
    };

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let generic_authorization_request = provider_manager
        .validate_request(authorization_request.clone())
        .await
        .map_err(|_| InvalidQRCodeError(authorization_request))?;

    if let Result::Ok(siopv2_authorization_request) =
        AuthorizationRequest::<Object<SIOPv2>>::from_generic(&generic_authorization_request)
    {
        let redirect_uri = siopv2_authorization_request.body.redirect_uri.to_string();

        let (client_name, logo_uri, connection_url) =
            get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request)
                .map_err(|_| MissingAuthorizationRequestParameterError("client_name"))?;

        let previously_connected = state
            .connections
            .iter_mut()
            .any(|connection| connection.url == connection_url && connection.client_name == client_name);

        state
            .active_connection_request
            .replace(ConnectionRequest::SIOPv2(siopv2_authorization_request.into()));

        info!("client_name in credential_offer: {:?}", client_name);
        info!("logo_uri in read_authorization_request: {:?}", logo_uri);

        state.current_user_prompt.replace(CurrentUserPrompt::AcceptConnection {
            client_name,
            logo_uri,
            redirect_uri,
            previously_connected,
        });
    } else if let Result::Ok(oid4vp_authorization_request) =
        AuthorizationRequest::<Object<OID4VP>>::from_generic(&generic_authorization_request)
    {
        let verifiable_credentials = stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();
        info!("verifiable credentials: {:?}", verifiable_credentials);

        let uuids: Vec<String> = oid4vp_authorization_request
            .body
            .extension
            .presentation_definition
            .input_descriptors()
            .iter()
            .map(|input_descriptor| {
                verifiable_credentials
                    .iter()
                    .find_map(|verifiable_credential_record| {
                        let jwt = verifiable_credential_record.verifiable_credential.credential().unwrap();
                        evaluate_input(input_descriptor, &get_unverified_jwt_claims(jwt))
                            .then_some(verifiable_credential_record.display_credential.id.clone())
                    })
                    .ok_or(NoMatchingCredentialError)
            })
            .collect::<Result<Vec<String>, AppError>>()?;

        info!("uuids of VCs that can fulfill the request: {:?}", uuids);

        let (client_name, logo_uri, _) = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request)
            .map_err(|_| MissingAuthorizationRequestParameterError("client_name"))?;

        info!("client_name in credential_offer: {:?}", client_name);
        info!("logo_uri in read_authorization_request: {:?}", logo_uri);

        state
            .active_connection_request
            .replace(ConnectionRequest::OID4VP(oid4vp_authorization_request.into()));

        // TODO: communicate when no credentials are available.
        if !uuids.is_empty() {
            state.current_user_prompt = Some(CurrentUserPrompt::ShareCredentials {
                client_name,
                logo_uri,
                options: uuids,
            });
        }
    } else {
        return Err(InvalidAuthorizationRequest(Box::new(generic_authorization_request)));
    };

    Ok(())
}

// Sends the authorization response.
pub async fn handle_siopv2_authorization_request(state: &mut AppState, _action: Action) -> Result<(), AppError> {
    let state_guard = state.managers.lock().await;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let active_connection_request = state
        .active_connection_request
        .take()
        .ok_or(MissingStateParameterError("active connection request"))?;

    let siopv2_authorization_request = match active_connection_request {
        ConnectionRequest::SIOPv2(siopv2_authorization_request) => siopv2_authorization_request,
        ConnectionRequest::OID4VP(_) => unreachable!(),
    };

    info!("generating response");

    let response = provider_manager
        .generate_response(&*siopv2_authorization_request, Default::default())
        .map_err(GenerateAuthorizationResponseError)?;
    info!("response generated: {:?}", response);

    if provider_manager.send_response(&response).await.is_err() {
        info!("failed to send response");
        return Err(SendAuthorizationResponseError);
    }
    info!("response successfully sent");

    let connection_time = chrono::Utc::now().to_rfc3339();

    let (client_name, logo_uri, connection_url) = get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request)
        .map_err(|_| MissingAuthorizationRequestParameterError("connection_url"))?;

    let result = state
        .connections
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_interacted = connection_time.clone();
        });

    if result.is_none() {
        state.connections.push(Connection {
            client_name,
            url: connection_url,
            logo_uri,
            verified: false,
            first_interacted: connection_time.clone(),
            last_interacted: connection_time,
        })
    };

    state.current_user_prompt.replace(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn handle_oid4vp_authorization_request(state: &mut AppState, action: Action) -> Result<(), AppError> {
    info!("handle_presentation_request");

    let credential_uuids = match action {
        Action::CredentialsSelected { credential_uuids } => credential_uuids,
        _ => return Err(InvalidActionError { action }),
    };

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;

    let active_connection_request = state
        .active_connection_request
        .take()
        .ok_or(MissingStateParameterError("active connection request"))?;

    let oid4vp_authorization_request = match active_connection_request {
        ConnectionRequest::OID4VP(oid4vp_authorization_request) => oid4vp_authorization_request,
        ConnectionRequest::SIOPv2(_) => unreachable!(),
    };

    let verifiable_credentials: Vec<Credential<JwtVcJson>> = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .iter()
        .filter_map(
            |verifiable_credential_record| match &verifiable_credential_record.verifiable_credential {
                CredentialFormats::JwtVcJson(jwt_vc_json) => credential_uuids
                    .contains(&verifiable_credential_record.display_credential.id.parse().unwrap())
                    .then_some(jwt_vc_json.to_owned()),
                _ => unimplemented!(),
            },
        )
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
        .active_profile
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

    let verifiable_presentation: Presentation<Jwt, identity_core::common::Object> =
        presentation_builder.build().map_err(PresentationBuilderError)?;

    info!("get the provider_manager");

    info!("generating response");
    let response = provider_manager
        .generate_response(
            &oid4vp_authorization_request,
            oid4vp::oid4vp::AuthorizationResponseInput {
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

    let (client_name, logo_uri, connection_url) = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request)
        .map_err(|_| MissingAuthorizationRequestParameterError("connection_url"))?;

    let result = state
        .connections
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_interacted = connection_time.clone();
        });

    if result.is_none() {
        state.connections.push(Connection {
            client_name,
            url: connection_url,
            logo_uri,
            verified: false,
            first_interacted: connection_time.clone(),
            last_interacted: connection_time,
        })
    };

    state.current_user_prompt.replace(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    Ok(())
}

// TODO: move this functionality to the oid4vc-manager crate.
fn get_siopv2_client_name_and_logo_uri(
    siopv2_authorization_request: &AuthorizationRequest<Object<SIOPv2>>,
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = siopv2_authorization_request
        .body
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(siopv2_authorization_request
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

// TODO: move this functionality to the oid4vc-manager crate.
fn get_oid4vp_client_name_and_logo_uri(
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
