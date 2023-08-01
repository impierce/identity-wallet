use crate::{
    crypto::stronghold::insert_into_stronghold,
    state::{
        actions::Action,
        user_flow::{CurrentUserFlow, CurrentUserFlowType, Offer},
        AppState,
    },
};
use did_key::{generate, Ed25519KeyPair};
use identity_credential::credential::Credential;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use log::info;
use oid4vc_manager::methods::key_method::KeySubject;
use oid4vci::{
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    token_request::{PreAuthorizedCode, TokenRequest},
    Wallet,
};
use std::sync::Arc;

pub async fn read_credential_offer(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("read_credential_offer");
    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let credential_offer: CredentialOffer = match serde_json::from_value(payload)? {
        CredentialOfferQuery::CredentialOffer(credential_offer) => credential_offer,
        _ => unreachable!(),
    };
    info!("credential offer: {:?}", credential_offer);
    *state.credential_offer.lock().unwrap() = Some(credential_offer.clone());
    *state.current_user_flow.lock().unwrap() = Some(CurrentUserFlow::Offer(Offer {
        r#type: CurrentUserFlowType::Offer,
        options: vec![serde_json::to_value(&credential_offer).unwrap()],
    }));
    Ok(())
}

pub async fn send_credential_request(state: &AppState, action: Action) -> anyhow::Result<()> {
    info!("send_credential_request");

    let payload = action.payload.ok_or(anyhow::anyhow!("unable to read payload"))?;
    let offer_index: usize = serde_json::from_value(payload["offer_index"].clone())?;
    let credential_offer = state.credential_offer.lock().unwrap().clone().unwrap();
    // The credential offer contains a credential format for a university degree.
    let university_degree_credential_format = match credential_offer.credentials.get(offer_index) {
        Some(CredentialsObject::ByValue(credential_format)) => credential_format,
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

    // Get the credential.
    let credential_response = wallet
        .get_credential(
            credential_issuer_metadata,
            &token_response,
            university_degree_credential_format.to_owned(),
        )
        .await
        .unwrap();

    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    let credential_0_as_json = decode::<serde_json::Value>(
        credential_response.credential.clone().unwrap().as_str().unwrap(),
        &key,
        &validation,
    )
    .unwrap()
    .claims;
    let credential_0 = serde_json::from_value::<Credential>(credential_0_as_json.get("vc").unwrap().clone()).unwrap();

    *state.credentials.lock().unwrap() = Some(vec![credential_0]);

    let buffer = serde_json::to_vec(&credential_response.credential.unwrap())?;

    insert_into_stronghold(b"key".to_vec(), buffer, "my-password").await?;

    *state.current_user_flow.lock().unwrap() = None;
    *state.credential_offer.lock().unwrap() = None;

    Ok(())
}
