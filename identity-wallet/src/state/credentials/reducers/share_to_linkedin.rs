use crate::error::AppError::{self, *};
use crate::http_client::get_http_client;
use crate::state::{
    actions::{listen, Action},
    credentials::actions::share_to_linkedin::ShareToLinkedIn,
    credentials::VerifiableCredentialRecord,
    did::extract_url_from_did_web,
    AppState,
};
use base64::Engine;
use chrono::{DateTime, Datelike, Duration, Utc};
use jsonwebtoken::Header;
use log::info;
use oid4vc::oid4vc_core::{jwt::encode, utils::did::extract_normalized_did_kid_from_jwt, Subject};
use openid_federation::FederationClient;
use serde::Serialize;
use serde_json::json;
use std::str::FromStr;
use url::Url;
use uuid::Uuid;

pub async fn share_to_linkedin(state: AppState, action: Action) -> Result<AppState, AppError> {
    if let Some(share_to_linkedin) = listen::<ShareToLinkedIn>(action) {
        let mut credentials = state.credentials.clone();
        let credential = credentials
            .iter_mut()
            .find(|cred| cred.id == share_to_linkedin.id)
            .ok_or(AppError::NoCredentialWithIdError(share_to_linkedin.id))?;

        // Build LinkedIn URL, all parameters must be URL percent-encoded, as specified by LinkedIn documentation: https://addtoprofile.linkedin.com/
        let mut linkedin_url = "https://www.linkedin.com/profile/add?startTask=CERTIFICATION_NAME".to_string();
        linkedin_url.push_str(format!("&name={}", urlencoding::encode(&credential.display_name)).as_str());
        linkedin_url.push_str(format!("&organizationName={}", urlencoding::encode(&credential.issuer_name)).as_str());

        let issue_date = DateTime::parse_from_rfc3339(&credential.metadata.date_issued)
            .map_err(|e| AppError::Error(format!("Failed to parse issue date: {}", e)))?;
        linkedin_url.push_str(format!("&issueYear={}", issue_date.year()).as_str());
        linkedin_url.push_str(format!("&issueMonth={}", issue_date.month()).as_str());

        if let Some(expiration_date_str) = &credential.metadata.expiration_date {
            let expiration_date = DateTime::parse_from_rfc3339(expiration_date_str)
                .map_err(|e| AppError::Error(format!("Failed to parse expiration date: {}", e)))?;
            linkedin_url.push_str(format!("&expirationYear={}", expiration_date.year()).as_str());
            linkedin_url.push_str(format!("&expirationMonth={}", expiration_date.month()).as_str());
        }

        // Get or create public link to the credential
        let public_link = if let Some(existing_link) = credential.public_link.clone() {
            info!(
                "Using existing public link for credential `{}`: `{}`",
                credential.id, existing_link
            );
            existing_link.clone()
        } else {
            let new_link = create_public_link(&state, &credential.id).await?.to_string();
            info!(
                "Created new public link for credential `{}`: `{}`",
                credential.id, new_link
            );
            new_link
        };

        linkedin_url.push_str(format!("&certUrl={}", urlencoding::encode(&public_link)).as_str());
        linkedin_url.push_str(format!("&certId={}", urlencoding::encode(&credential.id)).as_str());

        info!("Opening LinkedIn AddToProfile URL in browser: `{linkedin_url}`");

        // When testing, Tauri is often not initialized and the link doesn't actually need to be opened anyway.
        #[cfg(not(feature = "test_utils"))]
        {
            use tauri_plugin_opener::OpenerExt;

            let app_handle = state
                .core_utils
                .app_handle
                .clone()
                .ok_or(AppError::Error("Tauri app handle is not available".to_string()))?;
            app_handle
                .opener()
                .open_url(linkedin_url, None::<&str>)
                .map_err(|err| AppError::Error(format!("Failed to open URL in browser: {err}")))?;
        }

        credential.public_link = Some(public_link);
        return Ok(AppState { credentials, ..state });
    }

    Ok(state)
}

// TODO: this should actually be the reducer and the sharing to LinkedIn or whatever platform should be the helper, since that part doesnt update the AppState.
async fn create_public_link(state: &AppState, credential_id: &str) -> Result<Url, AppError> {
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

    let jwt_str = jwt.as_str().map(ToString::to_string).ok_or(AppError::Error(
        "Failed to convert the Public Link credential JWT to a string".to_string(),
    ))?;

    let kid = extract_normalized_did_kid_from_jwt(&jwt_str)
        .map_err(|e| AppError::Error(format!("Failed to extract kid from jwt: {e}")))?;

    let credential_issuer_did = kid.split('#').next().unwrap_or(&kid); // A did:web needs a # key fragment, a did:key doesn't

    // Get the credential's subject ID. This means anonymous credentials (without a credentialSubject.id) cannot be shared publicly.
    let credential_subject_id = vcr
        .display_credential
        .data
        .get("credentialSubject")
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
    let credential_id = vcr
        .display_credential
        .data
        .get("id")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Error("JTI not found".to_string()))?;

    let token_jti = Uuid::new_v4().to_string();
    let now = Utc::now();
    // TODO: this is a hardcoded expiration date of 1 year after issuance.
    let exp = now + Duration::days(365);

    let claims = PublicLinkTokenClaims {
        iss: credential_subject_id.to_string(), // This is the same as setting it to own_did, since the check above validates whether our own did which we will use for signing this DACT is the same as the credentialSubject.id.
        sub: credential_id.to_string(),
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
    let public_verifier_endpoint_url = get_trusted_verifier_public_verification_endpoint(credential_issuer_did).await?;

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
    let client = get_http_client().await;
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

/// Resolve the Issuer's Trust Chain to find the Ecosystem Leader (Trust Anchor) and its public verification endpoint, who will be the trusted verifier.
/// Step 1: Resolve the Issuer's trust chain
/// Step 2: get the Trust Anchor's URL
/// Step 3: Add the hardcoded /public/verify endpoint to the Trust Anchor's URL. TODO: in the future this should be either in DID services or Openid Fed metadata.
pub async fn get_trusted_verifier_public_verification_endpoint(issuer_did: &str) -> Result<String, AppError> {
    // This test feature is added to avoid the need to set up an entire trust ecosystem to create a unit test for this file.
    #[cfg(test)]
    if let Ok(endpoint) = std::env::var("UNIME_TEST_PUBLIC_VERIFIER_ENDPOINT") {
        return Ok(endpoint);
    }

    let federation_client = FederationClient::new();

    let issuer_url = extract_url_from_did_web(issuer_did).ok_or(AppError::Error(format!(
        "Failed to extract URL from issuer DID: {issuer_did}"
    )))?;

    let trust_chains = federation_client
        .discover_all_trust_chains(&issuer_url)
        .await
        .map_err(|e| {
            AppError::Error(format!(
                "Failed to discover a trusted trust_anchor for Issuer {issuer_url} of credential to be shared: {e}"
            ))
        })?;

    // TODO: improve trust chain selection logic, ultimately with policy and metadata checking, and iterating until a valid trust_anchor is found.
    let first_trust_chain = trust_chains.first().ok_or(AppError::Error(format!(
        "No trust_anchor/trust_chain found for Issuer of credential to be shared: {issuer_did}"
    )))?;
    let (trust_anchor_url, _) = first_trust_chain
        .trust_anchor_entity_id_and_configuration()
        .map_err(|e| AppError::Error(format!("Failed to get trust_anchor entity_id: {e}")))?;

    let trust_anchor_public_verification_endpoint = trust_anchor_url
        .join("/public/verify") // TODO: fix this hardcode via entity config metadata
        .map_err(|e| AppError::Error(format!("Failed to construct public verification endpoint URL: {e}")))?;

    Ok(trust_anchor_public_verification_endpoint.to_string())
}

/// Our own struct for standard JWT claims, mostly to make the claims we need non-optional form the start.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::STRONGHOLD;
    use crate::state::core_utils::{IdentityManager, Managers};
    use crate::state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS};
    use crate::stronghold::StrongholdManager;
    use crate::subject::subject;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use jsonwebtoken::Algorithm;
    use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
    use oid4vc::{oid4vc_manager::ProviderManager, oid4vci::Wallet};
    use serial_test::serial;
    use std::sync::Arc;
    use tempfile::NamedTempFile;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    fn build_jwt_with_kid(kid: &str) -> String {
        let header =
            URL_SAFE_NO_PAD.encode(serde_json::to_vec(&json!({ "alg": "EdDSA", "typ": "JWT", "kid": kid })).unwrap());
        let payload = URL_SAFE_NO_PAD.encode(serde_json::to_vec(&json!({ "sub": "test" })).unwrap());
        format!("{header}.{payload}.signature")
    }

    async fn setup_state_with_identity() -> Result<(AppState, String), AppError> {
        let path = NamedTempFile::new()
            .map_err(|e| AppError::Error(e.to_string()))?
            .into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

        let password = "sup3rSecr3t".to_string();
        let stronghold_manager = Arc::new(StrongholdManager::create(&password).map_err(StrongholdCreationError)?);
        let subject = subject(stronghold_manager.clone(), password).await;

        let provider_manager = ProviderManager::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCProviderManagerError)?;
        let wallet: Wallet = Wallet::new(
            subject.clone(),
            Vec::from(SUPPORTED_DID_METHODS),
            Vec::from(SUPPORTED_SIGNING_ALGORITHMS),
        )
        .map_err(OID4VCWalletError)?;

        let own_did = subject
            .identifier("did:key", Algorithm::EdDSA)
            .await
            .map_err(|e| AppError::Error(e.to_string()))?;

        let state = AppState {
            core_utils: crate::state::core_utils::CoreUtils {
                app_handle: None,
                managers: Arc::new(tauri::async_runtime::Mutex::new(Managers {
                    stronghold_manager: Some(stronghold_manager),
                    identity_manager: Some(IdentityManager {
                        subject,
                        provider_manager,
                        wallet,
                    }),
                })),
                active_flow: None,
            },
            ..Default::default()
        };

        Ok((state, own_did))
    }

    async fn assert_public_link_creation_for_format(format: CredentialFormats) {
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/store-data-access-consent-token"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        std::env::set_var(
            "UNIME_TEST_PUBLIC_VERIFIER_ENDPOINT",
            format!("{}/verify", mock_server.uri()),
        );

        let result = async {
            let (state, own_did) = setup_state_with_identity().await.unwrap();

            let credential_key = Uuid::new_v4();
            let kid = "did:web:issuer.example#key-1";
            let jwt = build_jwt_with_kid(kid);

            let vcr = VerifiableCredentialRecord {
                verifiable_credential: json!(jwt),
                display_credential: crate::state::credentials::DisplayCredential {
                    id: credential_key.to_string(),
                    format,
                    data: json!({
                        "id": format!("urn:uuid:{credential_key}"),
                        "credentialSubject": {
                            "id": own_did
                        }
                    }),
                    issuer_name: "Issuer".to_string(),
                    display_name: "Credential".to_string(),
                    ..Default::default()
                },
            };

            let managers = state.core_utils.managers.lock().await;
            let stronghold_manager = managers.stronghold_manager.as_ref().unwrap();
            stronghold_manager
                .insert(credential_key, serde_json::to_vec(&vcr).unwrap())
                .unwrap();
            drop(managers);

            let public_link = create_public_link(&state, &credential_key.to_string()).await.unwrap();
            assert!(public_link
                .as_str()
                .starts_with(&format!("{}/verify/", mock_server.uri())));
        }
        .await;

        std::env::remove_var("UNIME_TEST_PUBLIC_VERIFIER_ENDPOINT");
        result
    }

    #[tokio::test]
    #[serial]
    async fn create_public_link_from_jwt_vc_json() {
        assert_public_link_creation_for_format(CredentialFormats::JwtVcJson(())).await;
    }

    #[tokio::test]
    #[serial]
    async fn create_public_link_from_dc_sd_jwt() {
        assert_public_link_creation_for_format(CredentialFormats::DcSdJwt(())).await;
    }

    #[tokio::test]
    #[serial]
    async fn create_public_link_from_vc_sd_jwt() {
        assert_public_link_creation_for_format(CredentialFormats::VcSdJwt(())).await;
    }

    #[cfg(feature = "test_utils")]
    #[tokio::test]
    async fn share_to_linkedin_reducer_finishes_successfully() {
        let credential_id = Uuid::new_v4().to_string();
        let existing_public_link = "https://example.com/verify/existing".to_string();

        let state = AppState {
            credentials: vec![crate::state::credentials::DisplayCredential {
                id: credential_id.clone(),
                issuer_name: "Issuer".to_string(),
                display_name: "Credential".to_string(),
                metadata: crate::state::credentials::CredentialMetadata {
                    date_issued: "2026-01-01T00:00:00Z".to_string(),
                    ..Default::default()
                },
                public_link: Some(existing_public_link.clone()),
                ..Default::default()
            }],
            ..Default::default()
        };

        let action: Action = Arc::new(ShareToLinkedIn { id: credential_id });
        let updated_state = share_to_linkedin(state, action).await.unwrap();

        assert_eq!(updated_state.credentials.len(), 1);
        assert_eq!(
            updated_state.credentials[0].public_link.as_deref(),
            Some(existing_public_link.as_str())
        );
    }
}
