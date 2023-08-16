use crate::{
    crypto::stronghold::insert_into_stronghold,
    get_jwt_claims,
    state::{
        actions::Action,
        user_prompt::{CredentialOffer as CredentialOfferPrompt, CurrentUserPrompt, CurrentUserPromptType},
        AppState,
    },
};
use did_key::{generate, Ed25519KeyPair};
use log::info;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::{
    credential_format_profiles::{CredentialFormats, WithCredential},
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    credential_response::CredentialResponseType,
    token_request::{PreAuthorizedCode, TokenRequest},
    Wallet,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

pub async fn read_credential_offer(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_credential_offer");
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    // Create a new subject.

    let subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(
        "this-is-a-very-UNSAFE-secret-key".as_bytes(),
    )));

    // Create a new wallet.
    let wallet: Wallet = Wallet::new(Arc::new(subject));

    let mut credential_offer: CredentialOffer = match serde_json::from_value(payload)? {
        CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
        CredentialOfferQuery::CredentialOfferUri(credential_offer_uri) => {
            wallet.get_credential_offer(credential_offer_uri).await?
        }
    };
    info!("credential offer: {:?}", credential_offer);

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.clone().credential_issuer;

    // Get the credential issuer metadata.
    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await;

    // For all credentials by reference, replace them with credentials by value using the CredentialIssuerMetadata.
    credential_offer
        .credentials
        .iter_mut()
        .for_each(|credential| match credential {
            CredentialsObject::ByReference(by_reference) => {
                *credential = CredentialsObject::ByValue(
                    credential_issuer_metadata
                        .as_ref()
                        .map(|credential_issuer_metadata| {
                            credential_issuer_metadata
                                .credentials_supported
                                .iter()
                                .find(|credential_supported| {
                                    credential_supported.scope == Some(by_reference.to_owned())
                                })
                                .unwrap()
                                .credential_format
                                .clone()
                        })
                        .unwrap(),
                );
            }
            _by_value => (),
        });

    *state.current_user_prompt.lock().unwrap() = Some(CurrentUserPrompt::CredentialOffer(CredentialOfferPrompt {
        r#type: CurrentUserPromptType::CredentialOffer,
        credential_offer: serde_json::to_value(&credential_offer).unwrap(),
    }));
    Ok(())
}

pub async fn send_credential_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("send_credential_request");

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let offer_indices: Vec<usize> = serde_json::from_value(payload["offer_indices"].clone())?;
    let credential_offer = match state.current_user_prompt.lock().unwrap().clone().unwrap() {
        CurrentUserPrompt::CredentialOffer(offer) => {
            let credential_offer: CredentialOffer = serde_json::from_value(offer.credential_offer)?;
            credential_offer
        }
        _ => unreachable!(),
    };

    // The credential offer contains a credential issuer url.
    let credential_issuer_url = credential_offer.credential_issuer;

    // Create a new subject.
    let subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(
        "this-is-a-very-UNSAFE-secret-key".as_bytes(),
    )));

    // Create a new wallet.
    let wallet: Wallet = Wallet::new(Arc::new(subject));

    // Get the authorization server metadata.
    let authorization_server_metadata = wallet
        .get_authorization_server_metadata(credential_issuer_url.clone())
        .await
        .unwrap();

    // Get the credential issuer metadata.
    let credential_issuer_metadata = wallet
        .get_credential_issuer_metadata(credential_issuer_url.clone())
        .await
        .unwrap();

    let credential_offer_formats = offer_indices
        .into_iter()
        .map(|offer_index| match credential_offer.credentials.get(offer_index) {
            Some(CredentialsObject::ByValue(credential_format)) => credential_format.to_owned(),
            _ => unreachable!(),
        })
        .collect::<Vec<CredentialFormats>>();

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
        .get_access_token(authorization_server_metadata.token_endpoint, token_request)
        .await
        .unwrap();

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
                .unwrap();

            let credential = match credential_response.credential {
                CredentialResponseType::Immediate(credential) => credential,
                _ => panic!("Credential was not a JWT VC JSON."),
            };

            vec![credential]
        }
        _batch => {
            let batch_credential_response = wallet
                .get_batch_credential(credential_issuer_metadata, &token_response, credential_offer_formats)
                .await
                .unwrap();

            batch_credential_response
                .credential_responses
                .into_iter()
                .map(|credential_response| match credential_response {
                    CredentialResponseType::Immediate(credential) => credential,
                    _ => panic!("Credential was not a JWT VC JSON."),
                })
                .collect()
        }
    };
    info!("credentials: {:?}", credentials);

    // Get the decoded JWT claims to be displayed in the frontend.
    let mut credential_displays = vec![];
    for credential in credentials.iter() {
        let key = Uuid::new_v4();

        match credential {
            CredentialFormats::JwtVcJson(credential) => {
                let credential_display = serde_json::from_value::<identity_credential::credential::Credential>(
                    get_jwt_claims(&credential.credential)["vc"].clone(),
                )
                .unwrap();
                credential_displays.push((key.to_string(), credential_display));
            }
            _ => unimplemented!(),
        }

        let buffer = json!(credential).to_string().as_bytes().to_vec();
        insert_into_stronghold(key, buffer, "my-password")?;
    }
    info!("credential_displays: {:?}", credential_displays);

    state.credentials.lock().unwrap().extend(credential_displays);
    *state.current_user_prompt.lock().unwrap() = None;

    Ok(())
}
