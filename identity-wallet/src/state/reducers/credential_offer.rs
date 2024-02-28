use crate::{
    error::AppError::{self, *},
    state::{
        actions::{listen, Action, CredentialOffersSelected, QrCodeScanned},
        user_prompt::CurrentUserPrompt,
        AppState,
    },
    persistence::{persist_asset, download_asset, LogoType},
    verifiable_credential_record::VerifiableCredentialRecord,
};
use log::{debug, info};
use oid4vc::oid4vci::{
    credential_format_profiles::{CredentialFormats, WithCredential, WithParameters},
    credential_issuer::credentials_supported::CredentialsSupportedObject,
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    credential_response::CredentialResponseType,
    token_request::TokenRequest,
};
use serde_json::json;
use uuid::Uuid;

pub async fn read_credential_offer(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("read_credential_offer");

    // Sometimes reducers are connected to actions that they shouldn't execute
    // Therefore its also checked if it can parse to credential offer query
    // TODO find a better way to connect to the right reducer
    if let Some(credential_offer_uri) =
        listen::<QrCodeScanned>(action).and_then(|payload| payload.form_urlencoded.parse::<CredentialOfferQuery>().ok())
    {
        let state_guard = state.core_state.managers.lock().await;
        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

        let mut credential_offer: CredentialOffer = match credential_offer_uri {
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
                                .find(|credential_supported| {
                                    credential_supported.scope == Some(by_reference.to_owned())
                                })
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

        let mut display = vec![];
        for (index, credential_supported_object) in credentials_supported_objects.iter().enumerate() {
            let credential_logo_url = credential_supported_object.display.as_ref().and_then(|display| {
                display
                    .first()
                    .and_then(|value| value.get("logo").and_then(|value| value.get("url")))
            });

            info!("credential_logo_url: {:?}", credential_logo_url);

            if let Some(credential_logo_url) = credential_logo_url {
                debug!(
                    "{}",
                    format!("Downloading credential logo from url: {}", credential_logo_url)
                );
                if let Some(credential_logo_url) = credential_logo_url
                    .as_str()
                    .and_then(|s| s.parse::<reqwest::Url>().ok())
                {
                    let _ = download_asset(credential_logo_url, LogoType::CredentialLogo, index).await;
                }
            }

            if credential_logo_url.is_none() && logo_uri.is_none() {
                debug!("No logo found in metadata.");
            }

            display.push(
                credential_supported_object
                    .display
                    .as_ref()
                    .and_then(|display| display.first().cloned()),
            );
        }

        if logo_uri.is_some() {
            debug!(
                "{}",
                format!(
                    "Downloading issuer logo from url: {}",
                    logo_uri.as_ref().unwrap().as_str()
                )
            );
            if let Some(logo_uri) = logo_uri.as_ref().and_then(|s| s.parse::<reqwest::Url>().ok()) {
                let _ = download_asset(logo_uri, LogoType::IssuerLogo, 0).await;
            }
        }

        drop(state_guard);
        return Ok(AppState {
            current_user_prompt: Some(CurrentUserPrompt::CredentialOffer {
                issuer_name,
                logo_uri,
                credential_offer,
                display,
            }),
            ..state
        });
    }

    Ok(state)
}

pub async fn send_credential_request(state: AppState, action: Action) -> Result<AppState, AppError> {
    info!("send_credential_request");

    if let Some(offer_indices) = listen::<CredentialOffersSelected>(action).map(|payload| payload.offer_indices) {
        let state_guard = state.core_state.managers.lock().await;
        let stronghold_manager = state_guard
            .stronghold_manager
            .as_ref()
            .ok_or(MissingManagerError("stronghold"))?;
        let wallet = &state_guard
            .identity_manager
            .as_ref()
            .ok_or(MissingManagerError("identity"))?
            .wallet;

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
            .collect::<Vec<CredentialFormats<WithParameters>>>();

        info!("credential_offer_formats: {:?}", credential_offer_formats);

        // Create a token request with grant_type `pre_authorized_code`.
        let token_request = match credential_offer.grants {
            Some(Grants {
                pre_authorized_code, ..
            }) => TokenRequest::PreAuthorizedCode {
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

        for (i, credential) in credentials.into_iter().enumerate() {
            let mut verifiable_credential_record = VerifiableCredentialRecord::from(credential);
            verifiable_credential_record.display_credential.issuer_name = Some(issuer_name.clone());
            let key: Uuid = verifiable_credential_record
                .display_credential
                .id
                .parse()
                .expect("invalid uuid");

            info!("generated hash-key: {:?}", key);

            persist_asset(format!("credential_{}", i).as_str(), key.to_string().as_str()).ok();

            // Remove the old credential from the stronghold if it exists.
            stronghold_manager.remove(key).map_err(StrongholdDeletionError)?;

            stronghold_manager
                .insert(key, json!(verifiable_credential_record).to_string().as_bytes().to_vec())
                .map_err(StrongholdInsertionError)?;
        }

        let credentials = stronghold_manager
            .values()
            .map_err(StrongholdValuesError)?
            .unwrap()
            .into_iter()
            .map(|verifiable_credential_record| verifiable_credential_record.display_credential)
            .collect();

        let current_user_prompt = CurrentUserPrompt::Redirect {
            target: "me".to_string(),
        };

        drop(state_guard);
        return Ok(AppState {
            credentials,
            current_user_prompt: Some(current_user_prompt),
            ..state
        });
    }

    Ok(state)
}
