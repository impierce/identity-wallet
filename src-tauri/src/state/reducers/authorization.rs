use crate::{
    get_unverified_jwt_claims,
    state::{actions::Action, user_prompt::CurrentUserPrompt, AppState, Connection},
};
use identity_credential::{credential::Jwt, presentation::Presentation};
use log::info;
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
pub async fn read_authorization_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_authorization_request");

    let state_guard = state.managers.lock().await;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no identity manager found"))?
        .provider_manager;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no stronghold manager found"))?;

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    info!("trying to validate request: {:?}", payload);

    if let Result::Ok(siopv2_authorization_request) = provider_manager
        .validate_request::<SIOPv2>(serde_json::from_value(payload.clone())?)
        .await
    {
        let redirect_uri = siopv2_authorization_request.redirect_uri.to_string();

        let (client_name, logo_uri, connection_url) =
            get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request)?;

        let previously_connected = state
            .connections
            .lock()
            .unwrap()
            .iter_mut()
            .any(|connection| connection.url == connection_url && connection.client_name == client_name);

        state
            .active_connection_request
            .lock()
            .unwrap()
            .replace(ConnectionRequest::SIOPv2(siopv2_authorization_request.into()));

        info!("client_name in credential_offer: {:?}", client_name);
        info!("logo_uri in read_authorization_request: {:?}", logo_uri);

        state
            .current_user_prompt
            .lock()
            .unwrap()
            .replace(CurrentUserPrompt::AcceptConnection {
                client_name,
                logo_uri,
                redirect_uri,
                previously_connected,
            });
    } else if let Result::Ok(oid4vp_authorization_request) = provider_manager
        .validate_request::<OID4VP>(serde_json::from_value(payload.clone())?)
        .await
    {
        let verifiable_credentials = stronghold_manager.values()?.unwrap();
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
                    .ok_or(anyhow::anyhow!("unable to get credential"))
            })
            .collect::<anyhow::Result<Vec<String>>>()?;

        info!("uuids of VCs that can fulfill the request: {:?}", uuids);

        let (client_name, logo_uri, _) = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request)?;

        info!("client_name in credential_offer: {:?}", client_name);
        info!("logo_uri in read_authorization_request: {:?}", logo_uri);

        state
            .active_connection_request
            .lock()
            .unwrap()
            .replace(ConnectionRequest::OID4VP(oid4vp_authorization_request.into()));

        // TODO: communicate when no credentials are available.
        if !uuids.is_empty() {
            *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::ShareCredentials {
                client_name,
                logo_uri,
                options: uuids,
            });
        }
    } else {
        return Err(anyhow::anyhow!("unable to validate request"));
    };

    Ok(())
}

// Sends the authorization response.
pub async fn handle_siopv2_authorization_request(state: &AppState, _action: Action) -> anyhow::Result<()> {
    let state_guard = state.managers.lock().await;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no identity manager found"))?
        .provider_manager;

    let active_connection_request = state
        .active_connection_request
        .lock()
        .unwrap()
        .take()
        .ok_or(anyhow::anyhow!("no active connection request found"))?;

    let siopv2_authorization_request = match active_connection_request {
        ConnectionRequest::SIOPv2(siopv2_authorization_request) => siopv2_authorization_request,
        ConnectionRequest::OID4VP(_) => unreachable!(),
    };

    info!("||DEBUG|| generating response");
    *state.debug_messages.lock().unwrap() = vec!["generating response".into()];
    let response = provider_manager.generate_response(&siopv2_authorization_request, Default::default())?;
    info!("||DEBUG|| response generated: {:?}", response);

    if provider_manager.send_response(&response).await.is_err() {
        info!("||DEBUG|| failed to send response");

        state
            .active_connection_request
            .lock()
            .unwrap()
            .replace(ConnectionRequest::SIOPv2(siopv2_authorization_request));
        return Err(anyhow::anyhow!("failed to send response"));
    }
    info!("||DEBUG|| response successfully sent");

    let connection_time = chrono::Utc::now().to_rfc3339();

    let (client_name, logo_uri, connection_url) = get_siopv2_client_name_and_logo_uri(&siopv2_authorization_request)?;

    let result = state
        .connections
        .lock()
        .unwrap()
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_connected = connection_time.clone();
        });

    if result.is_none() {
        state.connections.lock().unwrap().push(Connection {
            client_name,
            url: connection_url,
            logo_uri,
            verified: false,
            first_connected: connection_time.clone(),
            last_connected: connection_time,
        })
    };

    state
        .current_user_prompt
        .lock()
        .unwrap()
        .replace(CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        });

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn handle_oid4vp_authorization_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("handle_presentation_request");

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no stronghold manager found"))?;
    let provider_manager = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(anyhow::anyhow!("no identity manager found"))?
        .provider_manager;

    let payload = match action.payload {
        Some(payload) => payload,
        None => {
            info!("||DEBUG|| unable to read payload");
            *state.debug_messages.lock().unwrap() = vec!["unable to read payload".into()];
            return Err(anyhow::anyhow!("unable to read payload"));
        }
    };

    let credential_uuids: Vec<Uuid> = serde_json::from_value::<Vec<String>>(payload["credential_uuids"].clone())?
        .into_iter()
        .map(|index| index.parse().map_err(|_| anyhow::anyhow!("unable to parse uuid")))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let active_connection_request = state
        .active_connection_request
        .lock()
        .unwrap()
        .take()
        .ok_or(anyhow::anyhow!("no active connection request found"))?;

    let oid4vp_authorization_request = match active_connection_request {
        ConnectionRequest::OID4VP(oid4vp_authorization_request) => oid4vp_authorization_request,
        ConnectionRequest::SIOPv2(_) => unreachable!(),
    };

    info!("||DEBUG|| credential not found");
    *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];

    let verifiable_credentials: Vec<Credential<JwtVcJson>> = stronghold_manager
        .values()?
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
    )?;

    info!("||DEBUG|| get the subject did");
    *state.debug_messages.lock().unwrap() = vec!["get the subject did".into()];
    let subject_did = state
        .active_profile
        .lock()
        .unwrap()
        .as_ref()
        .ok_or(anyhow::anyhow!("no active profile found"))?
        .primary_did
        .clone();

    let mut presentation_builder = Presentation::builder(subject_did.parse()?, Default::default());
    for verifiable_credential in verifiable_credentials {
        presentation_builder = presentation_builder.credential(Jwt::from(
            verifiable_credential
                .credential
                .as_str()
                .ok_or(anyhow::anyhow!("unable to get credential"))?
                .to_string(),
        ));
    }

    let verifiable_presentation = presentation_builder.build()?;

    info!("||DEBUG|| get the provider_manager");
    *state.debug_messages.lock().unwrap() = vec!["get the provider_manager".into()];

    info!("||DEBUG|| generating response");
    *state.debug_messages.lock().unwrap() = vec!["generating response".into()];
    let response = provider_manager.generate_response(
        &oid4vp_authorization_request,
        OID4VPUserClaims {
            verifiable_presentation,
            presentation_submission,
        },
    )?;
    info!("||DEBUG|| response generated: {:?}", response);

    if provider_manager.send_response(&response).await.is_err() {
        info!("||DEBUG|| failed to send response");
        state
            .active_connection_request
            .lock()
            .unwrap()
            .replace(ConnectionRequest::OID4VP(oid4vp_authorization_request));
        return Err(anyhow::anyhow!("failed to send response"));
    }
    info!("||DEBUG|| response successfully sent");

    let connection_time = chrono::Utc::now().to_rfc3339();

    let (client_name, logo_uri, connection_url) = get_oid4vp_client_name_and_logo_uri(&oid4vp_authorization_request)?;

    let result = state
        .connections
        .lock()
        .unwrap()
        .iter_mut()
        .find(|connection| connection.url == connection_url && connection.client_name == client_name)
        .map(|connection| {
            connection.last_connected = connection_time.clone();
        });

    if result.is_none() {
        state.connections.lock().unwrap().push(Connection {
            client_name,
            url: connection_url,
            logo_uri,
            verified: false,
            first_connected: connection_time.clone(),
            last_connected: connection_time,
        })
    };

    state
        .current_user_prompt
        .lock()
        .unwrap()
        .replace(CurrentUserPrompt::Redirect {
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
