use crate::error::AppError::{self, *};
use crate::state::credentials::VerifiableCredentialRecord;
use crate::state::did::extract_url_from_did_web;
use crate::state::AppState;
use base64::Engine;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode_header, Header};
use log::info;
use oid4vc::oid4vc_core::utils::jwt::get_unverified_jwt_claims;
use oid4vc::oid4vc_core::{jwt::encode, Subject};
use openid_federation::FederationClient;
use serde::Serialize;
use serde_json::json;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

// TODO: this should actually be the reducer and the sharing to LinkedIn or whatever platform should be the helper, since that part doesnt update the AppState.
pub async fn create_public_link(state: &AppState, credential_id: &str) -> Result<Url, AppError> {
    // TODO: perhaps add JWT claims to the DisplayCredential as well to avoid stronghold operations here?
    let id = Uuid::from_str(credential_id).map_err(|e| AppError::Error(e.to_string()))?;
    let managers = state.core_utils.managers.lock().await;
    let subject = managers
        .identity_manager
        .as_ref()
        .ok_or(MissingManagerError("identity"))?
        .subject
        .clone();
    let stronghold_manager = managers
        .stronghold_manager
        .as_ref()
        .ok_or(MissingManagerError("stronghold"))?
        .clone();

    // Get the VerifiableCredentialRecord belonging to the ID from Stronghold since the entire JWT, and its JWT claims, live there instead of just the VC part.
    let vcr_bytes = stronghold_manager
        .get(id)
        .map_err(|e| AppError::Error(e.to_string()))?
        .ok_or(AppError::Error(
            "Failed to get VerifiableCredentialRecord bytes from Stronghold".to_string(),
        ))?;
    let vcr: VerifiableCredentialRecord =
        serde_json::from_slice(&vcr_bytes).map_err(|e| AppError::Error(e.to_string()))?;
    let jwt = &mut vcr.verifiable_credential.clone();
    let jwt_data = get_unverified_jwt_claims(jwt).map_err(|e| AppError::Error(e.to_string()))?;
    let jwt_str = jwt.as_str().map(ToString::to_string).ok_or(AppError::Error(
        "Failed to convert the Public Link credential JWT to a string".to_string(),
    ))?;

    let jwt_header = decode_header(&jwt_str).map_err(|e| {
        AppError::Error(format!(
            "Failed to decode JWT header of the Public Link credential: {e}"
        ))
    })?;

    let kid = jwt_header.kid.ok_or(AppError::Error(
        "Public Link credential JWT header is missing `kid` field".to_string(),
    ))?;
    let credential_issuer_did = kid.split('#').next().unwrap_or(&kid); // A did:web needs a # key fragment, a did:key doesn't

    // TODO: implement the iss claim as fall back? Getting it from the KID is always safer since that is also checked during signature validation
    // // Get the credential's issuer DID from the credential data
    // let credential_issuer_did = jwt_data
    //     .get("iss")
    //     .and_then(|v| v.as_str())
    //     .ok_or(AppError::Error("Issuer (iss) not found".to_string()))?;

    // Get the credential's subject ID. This means anonymous credentials (without a credentialSubject.id) cannot be shared publicly.
    let credential_subject_id = jwt_data
        .get("vc")
        .and_then(|v| v.get("credentialSubject"))
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error(
            "Credential Subject ID not found in Public Link Credential".to_string(),
        ))?;

    // The DID we issue the DACT with must equal the credentialSubject.id of the credential which is given access to. How to handle key rotation in the future here?
    let did_method = credential_subject_id.split(':').take(2).collect::<Vec<_>>().join(":");
    let algorithm = algorithm_from_did(credential_subject_id)?;
    let own_did = subject.identifier(&did_method, algorithm).await.map_err(|e| {
        AppError::Error(format!(
            "Failed to get own DID for key type `{algorithm:?}` and did method `{did_method}`: {}",
            e
        ))
    })?;
    if own_did != credential_subject_id {
        return Err(AppError::Error(
            "The credential subject ID of the credential to be shared does not match any of the DIDs of the wallet"
                .to_string(),
        ));
    }

    // Compose the JWT header
    let header = Header {
        alg: algorithm,
        typ: Some("JWT".to_string()),
        kid: Some(credential_subject_id.to_string()),
        ..Default::default()
    };

    // Get the JTI claim from the credential data
    let jti = jwt_data
        .get("jti")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error("JTI not found".to_string()))?;
    let token_jti = Uuid::new_v4().to_string();
    let now = Utc::now();
    // TODO: this is a hardcoded expiration date of 1 year after issuance.
    let exp = now + Duration::days(365);

    let claims = PublicLinkTokenClaims {
        iss: credential_subject_id.to_string(), // This is the same as setting it to own_did, since the check above validates whether our own did which we will use for signing this DACT is the same as the credentialSubject.id.
        sub: jti.to_string(),
        aud: credential_issuer_did.to_string(),
        iat: now.timestamp(),
        nbf: now.timestamp(),
        exp: exp.timestamp(),
        jti: token_jti.clone(),
        status: "active".to_string(), // TODO: implement TSL revocation
    };

    let data_access_consent_token_jwt = encode(subject, header, claims, &did_method)
        .await
        .map_err(|e| AppError::Error(e.to_string()))?;

    info!("Generated Data Access Consent Token JWT: {data_access_consent_token_jwt}");

    // Extract the Issuer DID from the `aud` claim of the token
    let public_verifier_endpoint_url =
        get_trusted_verifier_public_verification_endpoint(state, credential_issuer_did).await?;

    // TODO: this hardcoded endpoint "discovery" could suffice for now but this implicitly states that the store_dact endpoint must always be on the same host path as the public verification endpoint.
    // Our public verification endpoint is {HOST}/verify, so we remove the /verify suffix and append /store-data-access-consent-token.
    let public_verifier_dact_endpoint_url = {
        let base = public_verifier_endpoint_url
            .strip_suffix("/verify") // TODO: import this as a const from SSI-agent to avoid mismatch?
            .unwrap_or(public_verifier_endpoint_url.as_str()); // This unwrap_or should never happen but if somehow the /verify endpoint doesnt have /verify as suffix we default to the unmodified url.
        format!("{}/store-data-access-consent-token", base)
    };

    info!("Public verifier endpoint URL: {public_verifier_endpoint_url}");
    info!("Public verifier DACT storage endpoint URL: {public_verifier_dact_endpoint_url}");

    // Before the public link is returned, the DACT needs to be stored by the verifier
    let client = reqwest::Client::new();
    let response = client
        .post(&public_verifier_dact_endpoint_url)
        .header("Content-Type", "application/json")
        .body(json!({ "dactId": token_jti, "jwt": data_access_consent_token_jwt }).to_string()) // TODO import this response type from ssi-agent
        .send()
        .await
        .map_err(|e| {
            AppError::Error(format!(
                "Failed to store Data Access Consent Token at the Verifier: {e}"
            ))
        })?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(AppError::Error(format!(
            "Verifier rejected the Data Access Consent Token (HTTP {status}): {body}"
        )));
    }

    let public_link = format!("{}/{}", public_verifier_endpoint_url, token_jti);

    let public_link_url = Url::parse(&public_link)
        .map_err(|e| AppError::Error(format!("Failed to create valid Public Link URL: {}", e)))?;

    info!("Succesfully generated public link URL: {}", public_link_url);

    Ok(public_link_url)
}

/// TODO: integrate OpenID Fed here once that mvp is done
/// Resolve the Issuer's Trust Chain to find the Ecosystem Leader (Trust Anchor) and its public verification endpoint, who will be the trusted verifier.
/// Step 1: Resolve the Issuer's trust chain
/// Step 2: get the Trust Anchor's DID Document
/// Step 3: find the Public Verification Endpoint service
pub async fn get_trusted_verifier_public_verification_endpoint(
    state: &AppState,
    issuer_did: &str,
) -> Result<String, AppError> {
    let federation_client = FederationClient::new();

    let issuer_url = extract_url_from_did_web(issuer_did).ok_or(AppError::Error(format!(
        "Failed to extract URL from issuer DID: {issuer_did}"
    )))?;

    let trust_list_urls: Vec<Url> = state
        .trust_lists
        .0
        .iter()
        .flat_map(|trust_list| trust_list.entries.keys().cloned())
        .collect();

    info!("Trust list URLs: {trust_list_urls:?}");

    let trust_chain = federation_client
        .discover_trust_chain(&issuer_url, Some(&trust_list_urls))
        .await
        .map_err(|e| {
            AppError::Error(format!(
                "Failed to discover a trusted trust_anchor for Issuer of credential to be shared: {e}"
            ))
        })?;

    let (trust_anchor_url, _) = trust_chain
        .trust_anchor_entity_id_and_configuration()
        .map_err(|e| AppError::Error(format!("Failed to get trust_anchor entity_id: {e}")))?;

    let trust_anchor_public_verification_endpoint = trust_anchor_url
        .join("/public/verify") // TODO: fix this hardcode via entity config metadata
        .map_err(|e| AppError::Error(format!("Failed to construct public verification endpoint URL: {e}")))?;

    Ok(trust_anchor_public_verification_endpoint.to_string())
}

#[derive(Serialize, Debug)]
struct PublicLinkTokenClaims {
    iss: String,
    sub: String,
    aud: String,
    iat: i64,
    nbf: i64,
    exp: i64,
    jti: String,
    status: String, // TODO: impl TSL revocation
}

/// Derive the signing algorithm from either a did:key or did:jwk.
fn algorithm_from_did(did: &str) -> Result<jsonwebtoken::Algorithm, AppError> {
    if let Some(multibase) = did.strip_prefix("did:key:z") {
        let bytes = bs58::decode(multibase)
            .into_vec()
            .map_err(|e| AppError::Error(format!("base58 decode error: {e}")))?;
        match (bytes.first(), bytes.get(1)) {
            (Some(0xed), Some(0x01)) => Ok(jsonwebtoken::Algorithm::EdDSA), // Ed25519
            (Some(0x12), Some(0x00)) => Ok(jsonwebtoken::Algorithm::ES256), // P-256
            _ => Err(AppError::Error(format!(
                "Unsupported did:key multicodec: 0x{:02x}{:02x}",
                bytes.first().unwrap_or(&0),
                bytes.get(1).unwrap_or(&0)
            ))),
        }
    } else if did.starts_with("did:jwk:") {
        // did:jwk:<base64url(JWK)>
        let jwk_b64 = did.strip_prefix("did:jwk:").unwrap();
        let jwk_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(jwk_b64)
            .map_err(|e| AppError::Error(format!("did:jwk base64 decode error: {e}")))?;
        let jwk: serde_json::Value =
            serde_json::from_slice(&jwk_bytes).map_err(|e| AppError::Error(format!("did:jwk JSON error: {e}")))?;
        match jwk.get("crv").and_then(|v| v.as_str()) {
            Some("Ed25519") => Ok(jsonwebtoken::Algorithm::EdDSA),
            Some("P-256") => Ok(jsonwebtoken::Algorithm::ES256),
            Some(crv) => Err(AppError::Error(format!("Unsupported did:jwk curve: {crv}"))),
            None => Err(AppError::Error("did:jwk missing 'crv' field".to_string())),
        }
    } else {
        Err(AppError::Error(format!("Cannot derive algorithm from DID: {did}")))
    }
}
