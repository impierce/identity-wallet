use crate::error::AppError;
use crate::persistence::{download_asset, hash};
use crate::state::core_utils::IdentityManager;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use did_manager::Resolver;
use identity_iota::{credential::Jwt, document::CoreDocument, verification::jws::Decoder};
use identity_jose::jwt::JwtClaims;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use log::{info, warn};
use serde_json::Value;

/// Downloads the logo from the given logo URI and stores it in the assets folder, returns None if it errors.
pub async fn download_logo(logo_uri_str: &str) -> Option<String> {
    match logo_uri_str.parse() {
        Ok(parsed_url) => {
            if download_asset(parsed_url, &hash(logo_uri_str)).await.is_err() {
                warn!("Failed to download logo from URI: {logo_uri_str:?}");
                return None;
            }
            info!("Successfully downloaded logo from URI: {logo_uri_str:?}");
            Some(logo_uri_str.to_string())
        }
        Err(parse_err) => {
            warn!("Failed to parse logo URI: {logo_uri_str:#?}, {parse_err}");
            None
        }
    }
}

/// Get the claims from a JWT without performing validation.
pub fn get_unverified_jwt_claims(jwt: &serde_json::Value) -> Result<serde_json::Value, AppError> {
    jwt.as_str()
        .and_then(|string| string.splitn(3, '.').collect::<Vec<&str>>().get(1).cloned())
        .and_then(|payload| {
            URL_SAFE_NO_PAD
                .decode(payload)
                .ok()
                .and_then(|payload_bytes| serde_json::from_slice::<serde_json::Value>(&payload_bytes).ok())
        })
        .ok_or(AppError::Error("Failed to decode JWT claims".to_string()))
}

/// This function uses the credential in jwt format from the jwt_vc_json to resolve the issuer document.
pub async fn get_issuer_document(resolver: &Resolver, credential_jwt: &Jwt) -> Option<CoreDocument> {
    let decoder = Decoder::new();

    // Decode the linked verifiable credential.
    let decoded_credential_jwt = decoder
        .decode_compact_serialization(credential_jwt.as_str().as_bytes(), None)
        .inspect_err(|err| warn!("Failed to decode credential jwt: {err:#?}"))
        .ok()?;

    let claims: JwtClaims<Value> = serde_json::from_slice(decoded_credential_jwt.claims())
        .inspect_err(|err| warn!("Failed to parse credential claims: {err:#?}"))
        .ok()?;

    info!("jwt_vc_json Credential claims: {claims:#?}");

    // Resolve the DID
    resolver
        .resolve(claims.iss()?)
        .await
        .inspect_err(|err| warn!("Failed to resolve issuer DID.: {err:#?}"))
        .ok()
}

/// Validate a jwt_vc_json, checks the JWT and the Issuer DID. TODO
pub async fn validate_jwt_vc_json(credential_jwt: &str, identity_manager: &IdentityManager) -> Result<Value, AppError> {
    let jwt_header = decode_header(credential_jwt).map_err(|_| AppError::GetCredentialStatusError)?;
    let key_id = jwt_header.kid.ok_or(AppError::GetCredentialStatusError)?;

    let public_key = identity_manager
        .subject
        .public_key(&key_id)
        .await
        .map_err(|_| AppError::GetCredentialStatusError)?;
    let decoding_key = match jwt_header.alg {
        Algorithm::EdDSA => DecodingKey::from_ed_der(&public_key),
        Algorithm::ES256 => DecodingKey::from_ec_der(&public_key),
        _ => {
            warn!("Unsupported algorithm: {:?}", jwt_header.alg);
            return Err(AppError::GetCredentialStatusError);
        }
    };

    // Set up validation rules for the JWT.
    let mut validation = Validation::new(jwt_header.alg);
    validation.set_required_spec_claims(&["sub", "iat"]); // todo
    validation.validate_aud = false;

    let token_data = decode::<Value>(credential_jwt, &decoding_key, &validation)
        .map_err(|e| AppError::InvalidCredentialFormatError(format!("Failed to decode credential JWT: {e}")))?;

    token_data
        .claims
        .get("vc")
        .ok_or(AppError::InvalidCredentialFormatError(
            "JwtVcJson is missing the 'vc' claim".to_string(),
        ))
        .cloned()
}

/// This trait is solely to add a method to serde_json::Value for converting Values to Strings cleanly
pub trait ValueToString {
    fn to_clean_string(&self) -> Option<String>;
}

impl ValueToString for serde_json::Value {
    /// A simple helper function to convert a `serde_json::Value` to an `Option<String>`.
    /// The original as_str or to_string methods work terribly due to including quotes characters.
    /// The original as_str/to_string methods output the following: "/".../"" or Some("/".../"").
    /// This function cleanly outputs Some("...").
    /// Renaming this clarifies our code instead of having as_str and to_string calls everywhere.
    fn to_clean_string(&self) -> Option<String> {
        self.as_str().map(ToString::to_string)
    }
}

pub struct DateUtils;

impl DateUtils {
    pub fn new_date_string() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_unverified_jwt_claims_successfully_gets_claims() {
        let jwt = json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw");

        assert_eq!(
            get_unverified_jwt_claims(&jwt).unwrap(),
            json!({
              "iss": "http://192.168.1.127:9090/",
              "sub": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
              "exp": 9999999999i64,
              "iat": 0,
              "vc": {
                "@context": [
                  "https://www.w3.org/2018/credentials/v1",
                  "https://www.w3.org/2018/credentials/examples/v1"
                ],
                "type": [
                  "VerifiableCredential",
                  "PersonalInformation"
                ],
                "issuanceDate": "2022-01-01T00:00:00Z",
                "issuer": "http://192.168.1.127:9090/",
                "credentialSubject": {
                  "id": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
                  "givenName": "Ferris",
                  "familyName": "Crabman",
                  "email": "ferris.crabman@crabmail.com",
                  "birthdate": "1985-05-21"
                }
              }
            })
        );
    }
}
