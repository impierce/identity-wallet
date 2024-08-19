use crate::error::AppError;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use identity_iota::verification::{
    jwk::{Jwk, JwkParams},
    jws::SignatureVerificationErrorKind,
};

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

pub struct DateUtils;

impl DateUtils {
    pub fn new_date_string() -> String {
        chrono::Utc::now().to_rfc3339()
    }
}

pub trait EncodedPublicKey {
    fn encoded_public_key(&self) -> Result<Vec<u8>, SignatureVerificationErrorKind>;
}

impl EncodedPublicKey for Jwk {
    fn encoded_public_key(&self) -> Result<Vec<u8>, SignatureVerificationErrorKind> {
        use SignatureVerificationErrorKind::*;

        match self.params() {
            JwkParams::Okp(okp_params) => Ok(URL_SAFE_NO_PAD.decode(&okp_params.x).map_err(|_| KeyDecodingFailure)?),
            JwkParams::Ec(ec_params) => {
                let x_bytes = URL_SAFE_NO_PAD.decode(&ec_params.x).map_err(|_| KeyDecodingFailure)?;
                let y_bytes = URL_SAFE_NO_PAD.decode(&ec_params.y).map_err(|_| KeyDecodingFailure)?;

                Ok(p256::EncodedPoint::from_affine_coordinates(
                    p256::FieldBytes::from_slice(&x_bytes),
                    p256::FieldBytes::from_slice(&y_bytes),
                    false, // false for uncompressed point
                )
                .as_bytes()
                .to_vec())
            }
            _ => Err(UnsupportedKeyType),
        }
    }
}

/// Helper trait for converting between different JWK types.
pub trait JwkConversion {
    fn try_into_identity_iota_jwk(&self) -> Result<identity_iota::verification::jwk::Jwk, AppError>;
    fn try_into_jsonwebtoken_jwk(&self) -> Result<jsonwebtoken::jwk::Jwk, AppError>;
}

impl JwkConversion for identity_iota::verification::jwk::Jwk {
    fn try_into_identity_iota_jwk(&self) -> Result<identity_iota::verification::jwk::Jwk, AppError> {
        Ok(self.clone())
    }

    fn try_into_jsonwebtoken_jwk(&self) -> Result<jsonwebtoken::jwk::Jwk, AppError> {
        serde_json::from_value(serde_json::json!(self))
            .map_err(|_| AppError::Error("Failed to convert JWK".to_string()))
    }
}

impl JwkConversion for jsonwebtoken::jwk::Jwk {
    fn try_into_identity_iota_jwk(&self) -> Result<identity_iota::verification::jwk::Jwk, AppError> {
        serde_json::from_value(serde_json::json!(self))
            .map_err(|_| AppError::Error("Failed to convert JWK".to_string()))
    }

    fn try_into_jsonwebtoken_jwk(&self) -> Result<jsonwebtoken::jwk::Jwk, AppError> {
        Ok(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_unverified_jwt_claims_successfully_gets_claims() {
        let jwt = serde_json::json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw");

        assert_eq!(
            get_unverified_jwt_claims(&jwt).unwrap(),
            serde_json::json!({
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
