use crate::{
    error::AppError::{self, *},
    get_unverified_jwt_claims,
    state::{actions::Action, user_prompt::CurrentUserPrompt, AppState, Connection},
    utils::{download_asset, LogoType},
};
use identity_credential::{credential::Jwt, presentation::Presentation};
use log::{debug, info};
use oid4vc_core::authorization_request::AuthorizationRequestObject;
use oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vci::credential_format_profiles::{
    w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, Credential, CredentialFormats,
};
use oid4vp::{evaluate_input, OID4VPUserClaims, OID4VP};
use siopv2::SIOPv2;
use uuid::Uuid;

pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequestObject<SIOPv2>>),
    OID4VP(Box<AuthorizationRequestObject<OID4VP>>),
}

// Reads the request url from the payload and validates it.
pub async fn read_authorization_request(state: &mut AppState, action: Action) -> Result<(), AppError> {
    info!("read_authorization_request");

    let state_guard = state.managers.lock().await;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .provider_manager;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;

    let payload = action.payload.ok_or(MissingPayloadError)?;

    info!("trying to validate request: {:?}", payload);

    if let Result::Ok(siopv2_authorization_request) = provider_manager
        .validate_request::<SIOPv2>(serde_json::from_value(payload.clone()).map_err(|source| {
            OID4VCAuthorizationRequestError {
                extension: "SIOPv2",
                source,
            }
        })?)
        .await
    {
        let redirect_uri = siopv2_authorization_request.redirect_uri.to_string();

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
    } else if let Result::Ok(oid4vp_authorization_request) = provider_manager
        .validate_request::<OID4VP>(serde_json::from_value(payload.clone()).map_err(|source| {
            OID4VCAuthorizationRequestError {
                extension: "OID4VP",
                source,
            }
        })?)
        .await
    {
        let verifiable_credentials = stronghold_manager.values().map_err(StrongholdValuesError)?.unwrap();
        info!("verifiable credentials: {:?}", verifiable_credentials);

        let uuids: Vec<String> = oid4vp_authorization_request
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

        if logo_uri.is_some() {
            debug!(
                "{}",
                format!(
                    "Downloading client logo from url: {}",
                    logo_uri.clone().unwrap().as_str()
                )
            );
            if let Some(logo_uri) = logo_uri.as_ref().and_then(|s| s.parse::<reqwest::Url>().ok()) {
                let _ = download_asset(logo_uri, LogoType::IssuerLogo, 0).await;
            }
        }

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
        return Err(InvalidAuthorizationRequest(payload));
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
        .generate_response(&siopv2_authorization_request, Default::default())
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

    if logo_uri.is_some() {
        debug!(
            "{}",
            format!(
                "Downloading issuer logo from url: {}",
                logo_uri.clone().unwrap().as_str()
            )
        );
        if let Some(logo_uri) = logo_uri.as_ref().and_then(|s| s.parse::<reqwest::Url>().ok()) {
            let _ = download_asset(logo_uri, LogoType::IssuerLogo, 0).await;
        }
    }

    let result = state
        .connections
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_interacted = connection_time.clone();
        });

    if result.is_none() {
        state.connections.push(Connection {
            id: "TODO".to_string(),
            client_name,
            url: connection_url,
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

    let payload = action.payload.ok_or(MissingPayloadError)?;

    let credential_uuids: Vec<Uuid> = serde_json::from_value::<Vec<String>>(payload["credential_uuids"].clone())
        .map_err(|_| MissingPayloadValueError("credential_uuids"))?
        .into_iter()
        .map(|index| index.parse().map_err(InvalidUuidError))
        .collect::<Result<Vec<_>, AppError>>()?;

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
        &oid4vp_authorization_request.extension.presentation_definition,
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

    let verifiable_presentation = presentation_builder.build().map_err(PresentationBuilderError)?;

    info!("get the provider_manager");

    info!("generating response");
    let response = provider_manager
        .generate_response(
            &oid4vp_authorization_request,
            OID4VPUserClaims {
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
            id: "TODO".to_string(),
            client_name,
            url: connection_url,
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
    siopv2_authorization_request: &AuthorizationRequestObject<SIOPv2>,
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = siopv2_authorization_request
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(siopv2_authorization_request
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
    oid4vp_authorization_request: &AuthorizationRequestObject<OID4VP>,
) -> anyhow::Result<(String, Option<String>, String)> {
    let connection_url = oid4vp_authorization_request
        .redirect_uri
        .domain()
        .ok_or(anyhow::anyhow!("unable to get domain from redirect_uri"))?
        .to_string();

    // Get the client_name and logo_uri from the client_metadata if it exists.
    Ok(oid4vp_authorization_request
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
