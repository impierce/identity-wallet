//! The issuer signature of a received credential, and the DID that made it (ADR 0005).

use crate::{
    error::AppError,
    state::core_utils::{helpers::validate_jwt_vc_json, IdentityManager},
    subject::Subject,
};
use identity_kit::{did::DidUrl, jose::DefaultVerifier, sd_jwt_vc::SdJwt};
use oid4vc_core::utils::{did::extract_normalized_did_kid_from_jwt, jwt::get_unverified_jwt_claims};
use oid4vci::credential_format_profiles::CredentialFormats;
use serde_json::Value;

/// Verifies the issuer signature of `credential`, received in `format`, and returns the DID that made it.
///
/// The signing key is the one the JWT's `kid` names, a DID URL (relative to `iss` when it starts with `#`), resolved
/// with the wallet's resolver. When the JWT has an `iss`, it must be that DID: the credential names the issuer whose
/// key signed it. For an SD-JWT, the disclosures must also match the digests of the signed payload.
pub async fn verify_issuer_signature(
    format: &CredentialFormats,
    credential: &Value,
    identity_manager: &IdentityManager,
) -> Result<String, AppError> {
    let token = credential.as_str().ok_or(AppError::InvalidCredentialFormatError)?;
    let signing_did = signing_did(token)?;

    match format {
        CredentialFormats::JwtVcJson(()) => {
            validate_jwt_vc_json(token, identity_manager).await?;
        }
        CredentialFormats::DcSdJwt(()) | CredentialFormats::VcSdJwt(()) => {
            verify_sd_jwt(token, &identity_manager.subject).await?;
        }
        _ => {
            return Err(AppError::UnsupportedError(
                "The issuer signature of a credential in this format cannot be verified".to_string(),
            ))
        }
    }

    Ok(signing_did)
}

/// The DID of the key that `token` (a JWT, or an SD-JWT's issuer-signed JWT) names in its `kid`, checked against its
/// `iss`.
fn signing_did(token: &str) -> Result<String, AppError> {
    let untrusted =
        |reason: String| AppError::UntrustedError(format!("The credential's issuer is not verifiable: {reason}"));

    let kid = extract_normalized_did_kid_from_jwt(token).map_err(|err| untrusted(format!("{err:#}")))?;
    let did = DidUrl::parse(&kid)
        .map_err(|err| untrusted(format!("its `kid` is not a DID URL: {err}")))?
        .did()
        .to_string();

    let jwt = token.split('~').next().unwrap_or(token);
    let claims = get_unverified_jwt_claims(&Value::String(jwt.to_string()))
        .map_err(|err| untrusted(format!("its claims cannot be read: {err:#}")))?;
    match claims.get("iss") {
        None => {}
        Some(Value::String(iss)) if *iss == did => {}
        Some(_) => return Err(untrusted(format!("its `iss` is not {did}, whose key signed it"))),
    }

    Ok(did)
}

/// Verifies the issuer signature of the SD-JWT `token` with the key its `kid` names, and that every disclosure matches
/// a digest of the signed payload.
async fn verify_sd_jwt(token: &str, subject: &Subject) -> Result<(), AppError> {
    let untrusted =
        |reason: String| AppError::UntrustedError(format!("The credential's signature is invalid: {reason}"));

    let sd_jwt = SdJwt::parse(token).map_err(|err| untrusted(format!("not an SD-JWT: {err}")))?;
    sd_jwt
        .process()
        .map_err(|err| untrusted(format!("its disclosures do not match: {err}")))?;

    let kid =
        extract_normalized_did_kid_from_jwt(sd_jwt.jwt().as_str()).map_err(|err| untrusted(format!("{err:#}")))?;
    let public_key = subject
        .resolver()
        .await
        // Only a method listed under `verificationMethod`, as for every other signature the wallet verifies.
        .listed_public_key(&kid)
        .await
        .map_err(|err| untrusted(format!("{err:#}")))?;

    sd_jwt
        .jwt()
        .decode()
        .map_err(|err| untrusted(format!("{err}")))?
        .verify(&DefaultVerifier, &public_key)
        .map_err(|err| untrusted(format!("{err}")))
}
