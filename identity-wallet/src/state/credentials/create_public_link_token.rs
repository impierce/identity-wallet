use crate::error::AppError::{self, *};
use crate::state::credentials::reducers::self_issue_credential::SubjectWrapper;
use crate::state::credentials::VerifiableCredentialRecord;
use crate::state::AppState;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use jsonwebtoken::Header;
use oid4vc::oid4vc_core::Sign;
use serde::Serialize;
use url::Url;

pub async fn create_public_link_token(state: &AppState, vcr: &VerifiableCredentialRecord) -> Result<String, AppError> {
    // Get the UniMe did
    let did_method = state
        .profile_settings
        .preferred_did_methods
        .first()
        .ok_or(AppError::Error("Failed to get a preferred did method".to_string()))?;

    let issuer_did: Url = state
        .dids
        .get(did_method)
        .ok_or(AppError::Error(
            "Failed to get the did for the preferred did method".to_string(),
        ))?
        .parse()
        .map_err(|_| AppError::Error("Failed to parse the did into a <Url>".to_string()))?;

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

    #[derive(Serialize)]
    struct PublicLinkTokenClaims<'a> {
        iss: &'a str,
        sub: &'a str,
        aud: &'a str,
        jti: &'a str,
        iat: i64,
        nbf: i64,
        exp: i64,
        status: &'a str,
    }

    let claims = PublicLinkTokenClaims {
        iss: issuer_did.as_str(),
        sub: jti,
        aud: credential_issuer_did,
        iat: now.timestamp(),
        nbf: now.timestamp(),
        exp: exp.timestamp(),
        jti,
        status: "active",
    };

    // Compose the JWT header
    let header = Header {
        alg: algorithm,
        typ: Some("JWT".to_string()),
        kid: Some(kid.clone()),
        ..Default::default()
    };

    // Encode header and claims to base64url
    let header_json =
        serde_json::to_vec(&header).map_err(|e| AppError::Error(format!("Failed to serialize header: {}", e)))?;
    let encoded_header = URL_SAFE_NO_PAD.encode(header_json);

    let claims_json =
        serde_json::to_vec(&claims).map_err(|e| AppError::Error(format!("Failed to serialize claims: {}", e)))?;
    let encoded_claims = URL_SAFE_NO_PAD.encode(claims_json);

    let message = format!("{}.{}", encoded_header, encoded_claims);

    let subject_wrapper = SubjectWrapper {
        subject: subject.clone(),
        preferred_did_method: did_method.to_string(),
    };
    drop(managers);

    // Sign the JWT
    let proof_value = Sign::sign(
        &*subject_wrapper.subject,
        &message,
        &subject_wrapper.preferred_did_method,
        algorithm,
    )
    .await
    .map_err(|e| AppError::Error(format!("Failed to sign JWT: {}", e)))?;

    let signature = URL_SAFE_NO_PAD.encode(proof_value.as_slice());
    let public_link_access_token = [message, signature].join(".");
    Ok(public_link_access_token)
}
