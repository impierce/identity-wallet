use crate::{
    crypto::stronghold::get_all_from_stronghold,
    get_jwt_claims,
    state::{
        actions::Action,
        user_flow::{CurrentUserFlow, CurrentUserFlowType, Selection},
        AppState,
    },
};
use identity_credential::{credential::Jwt, presentation::JwtPresentation};
use log::info;
use oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vci::credential_format_profiles::CredentialFormats;

// Reads the request url from the payload and validates it.
pub async fn read_authorization_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_request");
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    info!("trying to validate request: {:?}", payload);

    let authorization_request = provider
        .validate_request(serde_json::from_value(payload).unwrap())
        .await
        .unwrap();
    info!("validated authorization request: {:?}", authorization_request);

    let verifiable_credentials = get_all_from_stronghold("my-password").await?.unwrap();

    // Get the indices of the verifiable credentials that can be used to fulfill the request.
    let indices: Vec<usize> = verifiable_credentials
        .into_iter()
        .map(|vc| match vc {
            CredentialFormats::JwtVcJson(jwt_vc_json) => jwt_vc_json.credential,
            _ => unimplemented!(),
        })
        .enumerate()
        .filter_map(|(index, verifiable_credential)| {
            // Decode the verifiable credential from the JWT without validating.
            let verifiable_credential = get_jwt_claims(&verifiable_credential);

            // Create presentation submission using the presentation definition and the verifiable credential.
            match create_presentation_submission(
                authorization_request
                    .presentation_definition()
                    .as_ref()
                    .expect("presentation definition not found"),
                &verifiable_credential,
            ) {
                // The verifiable credential can be used to fulfill the request.
                Ok(_presentation_submission) => Some(index),
                // The verifiable credential cannot be used to fulfill the request.
                Err(_err) => None,
            }
        })
        .collect();

    info!("indices of VCs that can fulfill the request: {:?}", indices);

    // TODO: Make sure that the frontend receives the indices of the credentials that are conforming to the presentation
    // definition. Can The UserFlow be used for this? How?

    *state.active_authorization_request.lock().unwrap() = Some(authorization_request);

    if let Some(index) = indices.first() {
        *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Selection(Selection {
            r#type: CurrentUserFlowType::SelectCredentials,
            options: vec![index.to_string()],
        }));
    }

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn send_authorization_response(state: &AppState, action: Action) -> anyhow::Result<()> {
    // let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let payload = match action.payload {
        Some(payload) => payload,
        None => {
            info!("||DEBUG|| unable to read payload");
            *state.debug_messages.lock().unwrap() = vec!["unable to read payload".into()];
            return Err(anyhow::anyhow!("unable to read payload"));
        }
    };
    let credential_index: usize = serde_json::from_value(payload["credential_index"].clone())?;

    // let authorization_request = state
    //     .active_authorization_request
    //     .lock()
    //     .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
    //     .as_ref()
    //     .ok_or_else(|| anyhow::anyhow!("no active authentication request found"))?
    //     .clone();
    let authorization_request = match state
        .active_authorization_request
        .lock()
        .map_err(|_| anyhow::anyhow!("failed to obtain lock on active_authentication_request"))?
        .as_ref()
    {
        Some(authorization_request) => authorization_request.clone(),
        None => {
            info!("||DEBUG|| no active Authentication Request found");
            *state.debug_messages.lock().unwrap() = vec!["no active Authentication Request found".into()];
            return Err(anyhow::anyhow!("no active authentication request found"));
        }
    };

    info!("||DEBUG|| credential not found");
    *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];

    let verifiable_credentials: Vec<serde_json::Value> = get_all_from_stronghold("my-password")
        .await?
        .unwrap()
        .into_iter()
        .map(|vc| match vc {
            CredentialFormats::JwtVcJson(jwt_vc_json) => jwt_vc_json.credential,
            _ => unimplemented!(),
        })
        .collect();

    // let verifiable_credential = VERIFIABLE_CREDENTIALS
    //     .get(credential_index)
    //     .ok_or(anyhow::anyhow!("credential not found"))?;
    let verifiable_credential_jwt = match verifiable_credentials.get(credential_index) {
        Some(verifiable_credential) => verifiable_credential,
        None => {
            info!("||DEBUG|| credential not found");
            *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];
            return Err(anyhow::anyhow!("credential not found"));
        }
    };

    info!("||DEBUG|| decoding the verifiable credential");
    *state.debug_messages.lock().unwrap() = vec!["decoding the verifiable credential".into()];
    // Decode the verifiable credential from the JWT without validating.
    let verifiable_credential = get_jwt_claims(verifiable_credential_jwt);

    // Create presentation submission using the presentation definition and the verifiable credential.
    // let presentation_submission = create_presentation_submission(
    //     authorization_request
    //         .presentation_definition()
    //         .as_ref()
    //         .expect("presentation definition not found"),
    //     &verifiable_credential,
    // )?;
    let presentation_submission = create_presentation_submission(
        match authorization_request.presentation_definition().as_ref() {
            Some(presentation_definition) => presentation_definition,
            None => {
                info!("||DEBUG|| presentation definition not found");
                *state.debug_messages.lock().unwrap() = vec!["presentation definition not found".into()];
                return Err(anyhow::anyhow!("presentation definition not found"));
            }
        },
        &verifiable_credential,
    )?;

    info!("||DEBUG|| get the subject did");
    *state.debug_messages.lock().unwrap() = vec!["get the subject did".into()];
    let subject_did = state
        .active_profile
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .primary_did
        .clone();

    info!("||DEBUG|| credential not found");
    *state.debug_messages.lock().unwrap() = vec!["credential not found".into()];
    // Create a verifiable presentation using the JWT.
    let verifiable_presentation = JwtPresentation::builder(subject_did.parse().unwrap(), Default::default())
        .credential(Jwt::from(verifiable_credential_jwt.to_string()))
        .build()
        .unwrap();

    info!("||DEBUG|| get the provider");
    *state.debug_messages.lock().unwrap() = vec!["get the provider".into()];
    let guard = crate::PROVIDER_MANAGER.lock().await;
    let provider = guard.as_ref().unwrap();

    info!("||DEBUG|| generating response");
    *state.debug_messages.lock().unwrap() = vec!["generating response".into()];
    let response = provider.generate_response(
        authorization_request,
        Default::default(),
        Some(verifiable_presentation),
        Some(presentation_submission),
    )?;
    info!("||DEBUG|| response generated: {:?}", response);

    provider.send_response(response).await?;
    info!("||DEBUG|| response successfully sent");

    *state.current_user_flow.lock().unwrap() = None;

    Ok(())
}
