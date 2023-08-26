use crate::{
    get_unverified_jwt_claims,
    state::{
        actions::Action,
        user_prompt::{CurrentUserPrompt, CurrentUserPromptType, Selection},
        AppState,
    },
};
use identity_credential::{credential::Jwt, presentation::Presentation};
use log::info;
use oid4vc_manager::managers::presentation::create_presentation_submission;
use oid4vci::credential_format_profiles::{
    w3c_verifiable_credentials::jwt_vc_json::JwtVcJson, Credential, CredentialFormats,
};
use oid4vp::evaluate_input;
use uuid::Uuid;

// Reads the request url from the payload and validates it.
pub async fn read_authorization_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_authorization_request");

    let state_guard = state.managers.lock().await;
    let provider_manager = &state_guard.identity_manager.as_ref().unwrap().provider_manager;
    let stronghold_manager = state_guard.stronghold_manager.as_ref().unwrap();

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;

    info!("trying to validate request: {:?}", payload);

    let authorization_request = provider_manager
        .validate_request(serde_json::from_value(payload).unwrap())
        .await
        .unwrap();
    info!("validated authorization request: {:?}", authorization_request);

    let verifiable_credentials = stronghold_manager.values()?.unwrap();
    info!("verifiable credentials: {:?}", verifiable_credentials);

    let uuids: Vec<String> = if authorization_request.presentation_definition().is_some() {
        authorization_request
            .presentation_definition()
            .as_ref()
            .unwrap()
            .input_descriptors()
            .iter()
            .map(|input_descriptor| {
                verifiable_credentials
                    .iter()
                    .find_map(|verifiable_credential_record| {
                        let verifiable_credential = match &verifiable_credential_record.verifiable_credential {
                            CredentialFormats::JwtVcJson(jwt_vc_json) => jwt_vc_json.credential.clone(),
                            _ => unimplemented!(),
                        };
                        evaluate_input(input_descriptor, &get_unverified_jwt_claims(&verifiable_credential))
                            .then_some(verifiable_credential_record.display_credential.id.clone())
                    })
                    .unwrap()
            })
            .collect()
    } else {
        info!("||DEBUG|| generating response");
        *state.debug_messages.lock().unwrap() = vec!["generating response".into()];
        let response =
            provider_manager.generate_response(authorization_request.clone(), Default::default(), None, None)?;
        info!("||DEBUG|| response generated: {:?}", response);

        provider_manager.send_response(response).await?;
        info!("||DEBUG|| response successfully sent");
        return Ok(());
    };

    info!("uuids of VCs that can fulfill the request: {:?}", uuids);

    *state.active_authorization_request.lock().unwrap() = Some(authorization_request);

    if !uuids.is_empty() {
        *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::Selection(Selection {
            r#type: CurrentUserPromptType::SelectCredentials,
            options: uuids,
        }));
    }

    Ok(())
}

// Sends the authorization response including the verifiable credentials.
pub async fn send_authorization_response(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("send_authorization_response");

    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard.stronghold_manager.as_ref().unwrap();
    let provider_manager = &state_guard.identity_manager.as_ref().unwrap().provider_manager;

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
        .map(|index| index.parse().unwrap())
        .collect();

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
        match authorization_request.presentation_definition().as_ref() {
            Some(presentation_definition) => presentation_definition,
            None => {
                info!("||DEBUG|| presentation definition not found");
                *state.debug_messages.lock().unwrap() = vec!["presentation definition not found".into()];
                return Err(anyhow::anyhow!("presentation definition not found"));
            }
        },
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
        authorization_request,
        Default::default(),
        Some(verifiable_presentation),
        Some(presentation_submission),
    )?;
    info!("||DEBUG|| response generated: {:?}", response);

    provider_manager.send_response(response).await?;
    info!("||DEBUG|| response successfully sent");

    *state.current_user_prompt.lock().unwrap() = None;

    Ok(())
}
