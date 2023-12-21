use crate::{
    error::AppError::{self, *},
    state::{actions::Action, persistence::persist_asset, user_prompt::CurrentUserPrompt, AppState},
    utils::{download_asset, LogoType},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use identity_credential::credential;
use log::{debug, info};
use oid4vci::{
    credential_format_profiles::{CredentialFormats, WithCredential},
    credential_issuer::{credential_issuer_metadata, credentials_supported::CredentialsSupportedObject},
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    credential_response::CredentialResponseType,
    token_request::{PreAuthorizedCode, TokenRequest},
};
use serde_json::json;
use uuid::Uuid;

pub async fn read_credential_offer(state: &mut AppState, action: Action) -> Result<(), AppError> {
    info!("read_credential_offer");
    let state_guard = state.managers.lock().await;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    let payload = action.payload.ok_or(MissingPayloadError)?;

    let mut credential_offer: CredentialOffer = match serde_json::from_value(payload).map_err(InvalidCredentialOffer)? {
        CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
        CredentialOfferQuery::CredentialOfferUri(credential_offer_uri) => wallet
            .get_credential_offer(credential_offer_uri)
            .await
            .map_err(GetCredentialOfferError)?,
    };
    info!("credential offer: {:?}", credential_offer);

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.clone().credential_issuer;

    info!("credential issuer url: {:?}", credential_issuer_url);

    // Get the credential issuer metadata.
    let credential_issuer_metadata = if credential_offer
        .credentials
        .iter()
        .any(|credential| matches!(credential, CredentialsObject::ByReference(_)))
    {
        wallet
            .get_credential_issuer_metadata(credential_issuer_url.clone())
            .await
            .ok()
    } else {
        None
    };

    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .ok();

    info!("credential issuer metadata: {:?}", credential_issuer_metadata);

    let credentials_supported_objects: Vec<CredentialsSupportedObject> = credential_offer
        .credentials
        .iter()
        .map(|credential| {
            match credential {
                CredentialsObject::ByReference(by_reference) => credential_issuer_metadata
                    .as_ref()
                    .and_then(|credential_issuer_metadata| {
                        credential_issuer_metadata
                            .credentials_supported
                            .iter()
                            .find(|credential_supported| credential_supported.scope == Some(by_reference.to_owned()))
                    })
                    .ok_or(MissingCredentialOfferError(by_reference.clone())),
                _by_value => credential_issuer_metadata
                    .as_ref()
                    .and_then(|credential_issuer_metadata| credential_issuer_metadata.credentials_supported.first())
                    .ok_or(MissingCredentialOfferError("nothing found".to_string())),
            }
            .expect("TODO: handle missing")
            .to_owned()
        })
        .collect();

    // For all credentials by reference, replace them with credentials by value using the CredentialIssuerMetadata.
    for credential in credential_offer.credentials.iter_mut() {
        match credential {
            CredentialsObject::ByReference(by_reference) => {
                *credential = CredentialsObject::ByValue(
                    credential_issuer_metadata
                        .as_ref()
                        .and_then(|credential_issuer_metadata| {
                            credential_issuer_metadata
                                .credentials_supported
                                .iter()
                                .find(|credential_supported| {
                                    credential_supported.scope == Some(by_reference.to_owned())
                                })
                                .map(|credential_supported| credential_supported.credential_format.clone())
                        })
                        .ok_or(MissingCredentialOfferError(by_reference.clone()))?,
                );
            }
            _by_value => (),
        }
    }

    // Get the credential issuer display if present.
    let display = credential_issuer_metadata
        .and_then(|credential_issuer_metadata| {
            credential_issuer_metadata
                .display
                .map(|display| display.first().cloned())
        })
        .flatten();

    // Get the credential issuer name and logo uri or use the credential issuer url.
    let (issuer_name, logo_uri) = display
        .map(|display| {
            let issuer_name = display["client_name"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or(credential_issuer_url.to_string());
            let logo_uri = display["logo_uri"].as_str().map(|s| s.to_string());
            // ===== OpenID for Verifiable Credential Issuance - draft 12 (26 November 2023) =====
            // let issuer_name = display["name"]
            //     .as_str()
            //     .map(|s| s.to_string())
            //     .unwrap_or(credential_issuer_url.to_string());
            // let logo_uri = display["logo"]
            //     .as_object()
            //     .unwrap()
            //     .get("url")
            //     .unwrap()
            //     .as_str()
            //     .map(|s| s.to_string());
            (issuer_name, logo_uri)
        })
        .unwrap_or((credential_issuer_url.to_string(), None));

    info!("issuer_name in credential_offer: {:?}", issuer_name);
    info!("logo_uri in credential_offer: {:?}", logo_uri);

    let first_credentials_supported_object = credentials_supported_objects
        .first()
        .expect("TODO: handle empty credential offer");

    let credential_logo_url = first_credentials_supported_object.display.as_ref().and_then(|display| {
        display
            .first()
            .and_then(|value| value.get("logo").and_then(|value| value.get("url")))
    });

    info!("credential_logo_url: {:?}", credential_logo_url);

    if credential_logo_url.is_some() {
        debug!(
            "{}",
            format!(
                "Downloading credential logo from url: {}",
                credential_logo_url.unwrap().as_str().unwrap()
            )
        );
        let _ = download_asset(credential_logo_url.unwrap().as_str().unwrap(), LogoType::CredentialLogo).await;
    };

    if logo_uri.is_some() {
        debug!(
            "{}",
            format!(
                "Downloading issuer logo from url: {}",
                logo_uri.clone().unwrap().as_str()
            )
        );
        let _ = download_asset(logo_uri.as_ref().unwrap().as_str(), LogoType::IssuerLogo).await;
    }

    if credential_logo_url.is_none() && logo_uri.is_none() {
        debug!("No logo found in metadata.");
    }

    let display = first_credentials_supported_object
        .display
        .as_ref()
        .unwrap()
        .first()
        .unwrap()
        .clone();

    state.current_user_prompt = Some(CurrentUserPrompt::CredentialOffer {
        issuer_name,
        logo_uri,
        credential_offer,
        display,
    });

    Ok(())
}

pub async fn send_credential_request(state: &mut AppState, action: Action) -> Result<(), AppError> {
    info!("send_credential_request");
    let state_guard = state.managers.lock().await;
    let stronghold_manager = state_guard
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?;
    let wallet = &state_guard
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .wallet;

    let payload = action.payload.ok_or(MissingPayloadError)?;
    let offer_indices: Vec<usize> =
        serde_json::from_value(payload["offer_indices"].clone()).map_err(InvalidOfferIndicesError)?;

    let current_user_prompt = state
        .current_user_prompt
        .clone()
        .ok_or(MissingStateParameterError("current user prompt"))?;

    info!("current_user_prompt: {:?}", current_user_prompt);

    let credential_offer = match current_user_prompt {
        CurrentUserPrompt::CredentialOffer { credential_offer, .. } => credential_offer,
        _ => unreachable!(),
    };

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer;

    info!("credential issuer url: {:?}", credential_issuer_url);

    // Get the authorization server metadata.
    let authorization_server_metadata = wallet
        .get_authorization_server_metadata(credential_issuer_url.clone())
        .await
        .map_err(GetAuthorizationServerMetadataError)?;

    info!("authorization server metadata: {:?}", authorization_server_metadata);

    // Get the credential issuer metadata.
    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .map_err(GetCredentialIssuerMetadataError)?;

    info!("credential issuer metadata: {:?}", credential_issuer_metadata);

    // Get the credential issuer display.
    let display = credential_issuer_metadata
        .display
        .as_ref()
        .and_then(|display| display.first().cloned());

    // Get the credential issuer name or use the credential issuer url.
    let issuer_name = display
        .map(|display| {
            let issuer_name = display["client_name"]
                .as_str()
                .map(|s| s.to_string())
                .unwrap_or(credential_issuer_url.to_string());
            issuer_name
        })
        .unwrap_or(credential_issuer_url.to_string());

    let credential_offer_formats = offer_indices
        .into_iter()
        .map(|offer_index| match credential_offer.credentials.get(offer_index) {
            Some(CredentialsObject::ByValue(credential_format)) => credential_format.to_owned(),
            // Unreachable because we replace all `CredentialsObject::ByReference` with `CredentialsObject::ByValue` in `read_credential_offer`.
            _ => unreachable!(),
        })
        .collect::<Vec<CredentialFormats>>();

    info!("credential_offer_formats: {:?}", credential_offer_formats);

    // Create a token request with grant_type `pre_authorized_code`.
    let token_request = match credential_offer.grants {
        Some(Grants {
            pre_authorized_code, ..
        }) => TokenRequest::PreAuthorizedCode {
            grant_type: PreAuthorizedCode,
            pre_authorized_code: pre_authorized_code.unwrap().pre_authorized_code,
            user_pin: None,
        },
        None => unreachable!(),
    };
    info!("token_request: {:?}", token_request);

    // Get an access token.
    let token_response = wallet
        .get_access_token(authorization_server_metadata.token_endpoint.unwrap(), token_request)
        .await
        .map_err(GetAccessTokenError)?;

    info!("token_response: {:?}", token_response);

    let credentials: Vec<CredentialFormats<WithCredential>> = match credential_offer_formats.len() {
        0 => vec![],
        1 => {
            // Get the credential.
            let credential_response = wallet
                .get_credential(
                    credential_issuer_metadata,
                    &token_response,
                    credential_offer_formats[0].to_owned(),
                )
                .await
                .map_err(GetCredentialError)?;

            let credential = match credential_response.credential {
                CredentialResponseType::Immediate(credential) => credential,
                _ => panic!("Credential was not a jwt_vc_json."),
            };

            vec![credential]
        }
        _batch => {
            let batch_credential_response = wallet
                .get_batch_credential(credential_issuer_metadata, &token_response, credential_offer_formats)
                .await
                .map_err(GetBatchCredentialError)?;

            batch_credential_response
                .credential_responses
                .into_iter()
                .map(|credential_response| match credential_response {
                    CredentialResponseType::Immediate(credential) => credential,
                    _ => panic!("Credential was not a jwt_vc_json."),
                })
                .collect()
        }
    };
    info!("credentials: {:?}", credentials);

    for credential in credentials.into_iter() {
        let mut verifiable_credential_record = VerifiableCredentialRecord::from(credential);
        verifiable_credential_record.display_credential.issuer_name = Some(issuer_name.clone());
        let key: Uuid = verifiable_credential_record
            .display_credential
            .id
            .parse()
            .expect("invalid uuid");

        info!("generated hash-key: {:?}", key);

        persist_asset(
            "6440ceac338a920197100e60_NGDIL%20Logo%20Dark.svg",
            key.to_string().as_str(),
        )
        .ok();

        // Remove the old credential from the stronghold if it exists.
        stronghold_manager.remove(key).map_err(StrongholdDeletionError)?;

        stronghold_manager
            .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
            .map_err(StrongholdInsertionError)?;
    }

    state.credentials = stronghold_manager
        .values()
        .map_err(StrongholdValuesError)?
        .unwrap()
        .into_iter()
        .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
        .collect();

    state.current_user_prompt.replace(CurrentUserPrompt::Redirect {
        target: "me".to_string(),
    });

    Ok(())
}
