use crate::error::AppError::{self, *};
use crate::state::credentials::VerifiableCredentialRecord;
use crate::state::AppState;
use chrono::{Duration, Utc};
use jsonwebtoken::Header;
use oid4vc::oid4vc_core::jwt::encode;
use serde::Serialize;

pub async fn create_public_link_token(state: &AppState, vcr: &VerifiableCredentialRecord) -> Result<String, AppError> {
    // Get the UniMe did
    let did_method = state
        .profile_settings
        .preferred_did_methods
        .first()
        .ok_or(AppError::Error("Failed to get a preferred did method".to_string()))?;

    let issuer_did = state.dids.get(did_method).ok_or(AppError::Error(
        "Failed to get the did for the preferred did method".to_string(),
    ))?;

    // Get preferred key type and convert it to jsonwebtoken::Algorithm
    let key_type = state
        .profile_settings
        .preferred_key_types
        .first()
        .ok_or(AppError::Error("Failed to get a preferred key type".to_string()))?
        .as_str();
    let algorithm = match key_type {
        "EdDSA" => jsonwebtoken::Algorithm::EdDSA,
        "ES256" => jsonwebtoken::Algorithm::ES256,
        _ => return Err(AppError::Error("Unsupported key type".to_string())),
    };

    // Get kid
    let managers = state.core_utils.managers.lock().await;
    let subject = managers
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .subject
        .clone();

    let kid = subject
        .key_id(did_method, algorithm)
        .await
        .ok_or(AppError::Error("Failed to create a key id".to_string()))?;

    // Compose the JWT header
    let header = Header {
        alg: algorithm,
        typ: Some("JWT".to_string()),
        kid: Some(kid.clone()),
        ..Default::default()
    };

    // Get the credential's issuer DID from the credential data
    let data = &mut vcr.verifiable_credential.clone();
    let credential_issuer_did = data
        .get("iss")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error("Issuer (iss) not found".to_string()))?;

    // Get the JTI claim from the credential data
    let jti = data
        .get("jti")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error("JTI not found".to_string()))?;

    let now = Utc::now();
    let exp = now + Duration::days(365);

    let claims = PublicLinkTokenClaims {
        iss: issuer_did.to_string(),
        sub: jti.to_string(),
        aud: credential_issuer_did.to_string(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
        exp: exp.timestamp(),
        status: "active".to_string(), // TODO: implement TSL revocation
    };

    let public_link_jwt = encode(subject, header, claims, did_method)
        .await
        .map_err(|e| AppError::Error(e.to_string()))?;

    Ok(public_link_jwt)
}

#[derive(Serialize, Debug)]
struct PublicLinkTokenClaims {
    iss: String,
    sub: String,
    aud: String,
    iat: i64,
    nbf: i64,
    exp: i64,
    status: String, // TODO: impl TSL revocation
}
