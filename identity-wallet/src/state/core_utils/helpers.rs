use crate::{
    error::AppError,
    state::{did::validate_domain_linkage::Verifier, SUPPORTED_CRED_TYPE_SCHEMAS},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use did_manager::Resolver;
use identity_iota::{
    credential::{
        DecodedJwtCredential, FailFast, Jwt, JwtCredentialValidationOptions, JwtCredentialValidator, StatusCheck,
    },
    document::CoreDocument,
    verification::jws::Decoder,
};
use identity_jose::jwt::JwtClaims;
use jsonschema::ValidationError;
use log::{debug, info, warn};
use serde_json::Value;
use std::fs::File;

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
        .inspect_err(|err| warn!("Failed to decode credential jwt: {:#?}", err))
        .ok()?;

    let claims: JwtClaims<Value> = serde_json::from_slice(decoded_credential_jwt.claims())
        .inspect_err(|err| warn!("Failed to parse credential claims: {:#?}", err))
        .ok()?;

    info!("jwt_vc_json Credential claims: {:#?}", claims);

    // Resolve the DID
    resolver
        .resolve(claims.iss()?)
        .await
        .inspect_err(|err| warn!("Failed to resolve issuer DID.: {:#?}", err))
        .ok()
}

/// Validate a jwt_vc_json, checks the JWT, the VC against VC Data Model 1.1 and checks the Issuer DID.
pub async fn jwt_vc_json_validator(credential_jwt: Jwt) -> Result<DecodedJwtCredential<Value>, AppError> {
    // `SkipUnsupported` allows for custom credential types, such as the StatusList2021Entry (https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry)
    let validator = JwtCredentialValidator::with_signature_verifier(Verifier);
    let options = JwtCredentialValidationOptions::new().status_check(StatusCheck::SkipUnsupported);

    let resolver = Resolver::new().await;
    let issuer_document = get_issuer_document(&resolver, &credential_jwt)
        .await
        .ok_or(AppError::Error("Failed to resolve issuer DID".to_string()))?;

    validator
        .validate::<_, Value>(&credential_jwt, &issuer_document, &options, FailFast::FirstError)
        .map_err(|_| AppError::Error("Invalid jwt_vc_json".to_string()))
}

/// Validate supported credential types against their corresponding Json schema.
/// This function is only capable of validating VC's and subsequent Credential Formats/Types.
/// All VC's must have a `type` field, which is either a string or an array of strings.
pub fn credential_schema_validation(data: &Value) -> Result<(), AppError> {
    if let Some(credential_type) = data.get("type").and_then(|f| f.as_str()) {
        if SUPPORTED_CRED_TYPE_SCHEMAS.contains(&credential_type) {
            let json_schema_path = format!("resources/jsonschemas/{}.json", credential_type);

            json_schema_validation(json_schema_path, data)?;
            debug!(
                "Credential type: {:?} succesfully validated against corresponding Json schema",
                credential_type
            );
        } else {
            warn!("No supported schema found for Credential type: {:?}", credential_type);
        }
    } else {
        let credential_type_array: Vec<String> = data
            .get("type")
            .and_then(|f| f.as_array())
            .ok_or_else(|| AppError::InvalidCredentialFormatError)?
            .iter()
            .map(|f| {
                f.as_str()
                    .map(String::from)
                    .ok_or(AppError::InvalidCredentialFormatError)
            })
            .collect::<Result<Vec<String>, AppError>>()?;

        if SUPPORTED_CRED_TYPE_SCHEMAS
            .iter()
            .any(|x| credential_type_array.contains(&x.to_string()))
        {
            for mut supported_cred_type in SUPPORTED_CRED_TYPE_SCHEMAS {
                if credential_type_array.contains(&supported_cred_type.to_string()) {
                    // OpenBadgeCredentials can be typed as "OpenBadgeCredential" or "AchievementCredential"
                    if *supported_cred_type == "AchievementCredential" {
                        supported_cred_type = &"OpenBadgeCredential";
                    }

                    let json_schema_path = format!("resources/jsonschemas/{}.json", supported_cred_type);
                    json_schema_validation(json_schema_path, data)?;

                    debug!(
                        "Credential type: {:?} succesfully validated against corresponding Json schema",
                        supported_cred_type
                    );
                }
            }
        } else {
            warn!(
                "No supported schema found for Credential types: {:?}",
                credential_type_array
            );
        }
    }

    Ok(())
}

/// Validate any given data in serde_json::Value format against any given JsonSchema by path.
pub fn json_schema_validation(json_schema_path: String, data: &Value) -> Result<(), AppError> {
    let json_schema_file = File::open(json_schema_path.clone())
        .map_err(|_| AppError::Error("Failed to find or read from JsonSchema file".to_string()))?;
    let reader = std::io::BufReader::new(json_schema_file);
    let json_schema: Value = serde_json::from_reader(reader)
        .map_err(|_| AppError::Error("Failed to convert JsonSchema &str to serde_json::Value".to_string()))?;

    let schema = jsonschema::draft201909::new(&json_schema)
        .map_err(|_| AppError::Error("Failed to compile JsonSchema from serde_json::Value".to_string()))?;

    let result = schema.iter_errors(data);

    let errors: Vec<ValidationError> = result.collect();
    if !errors.is_empty() {
        Err(AppError::Error(format!(
            "The data is invalid according to the given JsonSchema: {:?}",
            errors
        )))
    } else {
        Ok(())
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
    use lazy_static::lazy_static;
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

    lazy_static! {
        static ref EXAMPLE_BASIC_OB3: Value = json!({
            "@context": [
              "https://www.w3.org/ns/credentials/v2",
              "https://purl.imsglobal.org/spec/ob/v3p0/context-3.0.3.json"
            ],
            "id": "http://example.com/credentials/3527",
            "type": ["VerifiableCredential", "AchievementCredential"],
            "issuer": {
              "id": "https://example.com/issuers/876543",
              "type": ["Profile"],
              "name": "Example Corp"
            },
            "validFrom": "2010-01-01T00:00:00Z",
            "name": "Teamwork Badge",
            "credentialSubject": {
              "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
              "type": ["AchievementSubject"],
              "achievement": {
                        "id": "https://example.com/achievements/21st-century-skills/teamwork",
                        "type": ["Achievement"],
                        "criteria": {
                            "narrative": "Team members are nominated for this badge by their peers and recognized upon review by Example Corp management."
                        },
                        "description": "This badge recognizes the development of the capacity to collaborate within a group environment.",
                        "name": "Teamwork"
                    }
            }
        });
    }

    #[test]
    fn credential_schema_validation_ok() {
        let result = credential_schema_validation(&EXAMPLE_BASIC_OB3);
        assert!(result.is_ok());
    }

    #[test]
    fn credential_schema_validation_err() {
        let mut invalid_ob3 = EXAMPLE_BASIC_OB3.clone();

        *invalid_ob3.get_mut("id").unwrap() = json!(["InvalidType"]);

        let result = credential_schema_validation(&invalid_ob3);
        assert!(result.is_err());
    }
}
