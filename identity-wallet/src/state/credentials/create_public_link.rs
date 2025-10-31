use std::str::FromStr;

use crate::error::AppError::{self, *};
use crate::state::core_utils::helpers::get_unverified_jwt_claims;
use crate::state::credentials::VerifiableCredentialRecord;
use crate::state::AppState;
use chrono::{Duration, Utc};
use did_manager::Resolver;
use identity_iota::core::ToJson;
use jsonwebtoken::Header;
use log::{info, warn};
use oid4vc::oid4vc_core::jwt::encode;
use serde::Serialize;
use serde_json::Value;
use url::Url;
use uuid::Uuid;

// TODO: this should actually be the reducer and the sharing to LinkedIn or whatever platform should be the helper, since that part doesnt update the AppState.
pub async fn create_public_link(state: &AppState, credential_id: &str) -> Result<Url, AppError> {
    // Get the VerifiableCredentialRecord belonging to the Id from Stronghold
    let id = Uuid::from_str(credential_id).map_err(|e| AppError::Error(e.to_string()))?;
    let stronghold_manager = state
        .core_utils
        .managers
        .lock()
        .await
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?
        .clone();
    let vcr_bytes = stronghold_manager
        .get(id)
        .map_err(|e| AppError::Error(e.to_string()))?
        .ok_or(AppError::Error(
            "Failed to get VerifiableCredentialRecord bytes from Stronghold".to_string(),
        ))?;
    let vcr: VerifiableCredentialRecord =
        serde_json::from_slice(&vcr_bytes).map_err(|e| AppError::Error(e.to_string()))?;

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
    let jwt = &mut vcr.verifiable_credential.clone();
    let jwt_data = get_unverified_jwt_claims(jwt)?;
    let credential_issuer_did = jwt_data
        .get("iss")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error("Issuer (iss) not found".to_string()))?;

    // Get the JTI claim from the credential data
    let jti = jwt_data
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

    // Extract the Issuer DID from the `aud` claim of the token
    let public_credential_endpoint_url = get_issuer_public_verification_endpoint(state, issuer_did).await?;

    let public_link = format!("{}/{}", public_credential_endpoint_url, public_link_jwt);

    let public_link_url =
        Url::parse(&public_link).map_err(|e| AppError::Error(format!("Invalid public link URL: {}", e)))?;

    Ok(public_link_url)
}

// TODO: integrate this with UniCore after updating the UniCore DID and endpoint
pub async fn get_issuer_public_verification_endpoint(_state: &AppState, issuer_did: &str) -> Result<String, AppError> {
    let resolver = Resolver::new().await;
    let issuer_document = resolver
        .resolve(issuer_did)
        .await
        .map_err(|_| AppError::Error("Failed to resolve issuer did".to_string()))?;

    let public_credential_endpoint = issuer_document.service().iter().find_map(|service| {
        service
            .type_()
            .contains("PublicEndpointStuff")
            .then(|| {
                info!("Found PublicEndpointPH: {service:#?}");
                service.service_endpoint()
            })
            .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
            .and_then(|endpoint_value| match endpoint_value {
                Value::String(url) => Some(url),
                Value::Object(obj) => obj
                    .get("url")
                    .or_else(|| obj.get("uri"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                _ => {
                    warn!("Unexpected service endpoint format: {endpoint_value:#?}");
                    None
                }
            })
    });
    public_credential_endpoint
        .ok_or_else(|| AppError::Error("No public credential endpoint found in issuer DID document".to_string()))
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
