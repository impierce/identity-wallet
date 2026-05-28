use crate::error::AppError;
use crate::persistence::{download_asset, hash};
use crate::state::core_utils::IdentityManager;
use did_manager::Resolver;
use identity_iota::{credential::Jwt, document::CoreDocument, verification::jws::Decoder};
use identity_jose::jwt::JwtClaims;
use jsonschema::ValidationError;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use log::{debug, info, warn};
use oid4vc::oid4vc_core::utils::did::resolve_key_id;
use oid4vc::oid4vc_core::Verify;
use serde_json::Value;
use std::fs::File;

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

// TODO: Implement one single verifier that can be used for all our JWT validation purposes. See `struct Verifier`. Also consider
// the JWT encoding rules defined here: https://www.w3.org/TR/vc-data-model-1.1/#jwt-encoding
/// Validate a jwt_vc_json, checks the JWT and the Issuer DID.
pub async fn validate_jwt_vc_json(credential_jwt: &str, identity_manager: &IdentityManager) -> Result<Value, AppError> {
    let jwt_header = decode_header(credential_jwt).map_err(|_| AppError::GetCredentialStatusError)?;
    let key_id = resolve_key_id(credential_jwt).map_err(|_| AppError::GetCredentialStatusError)?;

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
    validation.validate_aud = false;
    validation.required_spec_claims.clear();

    let token_data = decode::<Value>(credential_jwt, &decoding_key, &validation)
        .map_err(|_| AppError::InvalidCredentialFormatError)?;

    token_data
        .claims
        .get("vc")
        .ok_or(AppError::InvalidCredentialFormatError)
        .cloned()
}

/// Validate supported credential types against their corresponding JSON Schema.
/// This function is only capable of validating VC's and subsequent Credential Formats/Types.
/// All VC's must have a `type` field, which is either a string or an array of strings.
pub fn validate_credential_types(data: &Value) -> Result<(), AppError> {
    let type_field = data.get("type");

    match type_field {
        Some(_type) if !_type.is_null() => {
            match serde_json::from_value::<StringOrArray>(_type.clone())
                .map_err(|_| AppError::InvalidCredentialFormatError)?
            {
                StringOrArray::String(credential_type) => Ok(credential_type.validate(data)?),
                StringOrArray::Array(credential_type_array) => credential_type_array
                    .iter()
                    .try_for_each(|credential_type| credential_type.validate(data)),
            }
        }
        _ => {
            debug!("No credential type found, skipping validation");
            Ok(())
        }
    }
}

/// Validate any given data in serde_json::Value format against any given JSON Schema by path.
pub fn validate_credential_against_schema(json_schema_path: String, data: &Value) -> Result<(), AppError> {
    let json_schema_file = File::open(&json_schema_path)
        .map_err(|_| AppError::Error("Failed to find or read from JSON Schema file".to_string()))?;
    let json_schema: Value = serde_json::from_reader(json_schema_file)
        .map_err(|_| AppError::Error("Failed to convert JSON Schema &str to serde_json::Value".to_string()))?;

    // Select correct draft version for JSON Schema Validator
    let schema = match json_schema
        .get("$schema")
        .and_then(|value| value.as_str().map(ToString::to_string))
        .ok_or(AppError::Error("Invalid or missing \"$schema\" field".to_string()))?
        .as_str()
    {
        "https://json-schema.org/draft/2019-09/schema#" => {
            jsonschema::draft201909::new(&json_schema).map_err(|_| {
                AppError::Error(format!(
                    "Failed to compile JSON Schema from serde_json::Value: {json_schema}"
                ))
            })?
        }
        "https://json-schema.org/draft/2020-12/schema" => jsonschema::draft202012::new(&json_schema).map_err(|_| {
            AppError::Error(format!(
                "Failed to compile JSON Schema from serde_json::Value: {json_schema}"
            ))
        })?,
        _ => jsonschema::draft202012::new(&json_schema).map_err(|_| {
            AppError::Error(format!(
                "Failed to compile JSON Schema from serde_json::Value: {json_schema}"
            ))
        })?,
    };

    let errors: Vec<ValidationError> = schema.iter_errors(data).collect();
    if !errors.is_empty() {
        Err(AppError::Error(format!(
            "The data is invalid according to the given JSON Schema: {errors:?}"
        )))
    } else {
        Ok(())
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum StringOrArray {
    String(CredentialType),
    Array(Vec<CredentialType>),
}

#[derive(serde::Deserialize, PartialEq, Debug, strum::Display)]
enum CredentialType {
    VerifiableCredential,
    #[serde(alias = "AchievementCredential")]
    OpenBadgeCredential,
    #[serde(other)]
    Unknown,
}

#[derive(serde::Deserialize, PartialEq, Debug, strum::Display)]
enum CredentialTypeVersion {
    VerifiableCredentialV1_1,
    VerifiableCredentialV2,
    OpenBadgeCredentialV3,
    #[serde(other)]
    Unknown,
}

impl CredentialType {
    fn get_version(&self, data: &Value) -> Result<CredentialTypeVersion, AppError> {
        let context_array = serde_json::from_value::<Vec<String>>(data["@context"].clone())
            .map_err(|_| AppError::InvalidCredentialFormatError)?;

        match self {
            CredentialType::OpenBadgeCredential => {
                match context_array
                    .get(1)
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .as_str()
                {
                    context
                        if context.starts_with("https://purl.imsglobal.org/spec/ob/v3p0/context-")
                            && context.ends_with(".json") =>
                    {
                        Ok(CredentialTypeVersion::OpenBadgeCredentialV3)
                    }
                    _ => Err(AppError::InvalidCredentialFormatError),
                }
            }
            CredentialType::VerifiableCredential => {
                match context_array
                    .first()
                    .ok_or(AppError::InvalidCredentialFormatError)?
                    .as_str()
                {
                    "https://www.w3.org/2018/credentials/v1" => Ok(CredentialTypeVersion::VerifiableCredentialV1_1),
                    "https://www.w3.org/ns/credentials/v2" => Ok(CredentialTypeVersion::VerifiableCredentialV2),
                    _ => Err(AppError::InvalidCredentialFormatError),
                }
            }
            CredentialType::Unknown => {
                warn!("No version found for credential type: {self:?}");
                Ok(CredentialTypeVersion::Unknown)
            }
        }
    }

    fn validate(&self, data: &Value) -> Result<(), AppError> {
        let version = self.get_version(data)?;

        match version {
            CredentialTypeVersion::Unknown => {
                warn!("Credential Type unknown, skipping validation.");
                Ok(())
            }
            _ => {
                let json_schema_path = format!("resources/jsonschemas/{version}.json");

                validate_credential_against_schema(json_schema_path, data)?;
                debug!("Credential type: {self:?} successfully validated against corresponding JSON Schema");

                Ok(())
            }
        }
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
        let result = validate_credential_types(&EXAMPLE_BASIC_OB3);
        assert!(result.is_ok());
    }

    #[test]
    fn credential_schema_validation_err() {
        let mut invalid_ob3 = EXAMPLE_BASIC_OB3.clone();

        *invalid_ob3.get_mut("id").unwrap() = json!(["InvalidId"]);

        let result = validate_credential_types(&invalid_ob3);
        assert!(result.is_err());
    }

    #[test]
    fn credential_schema_validation_unknown_type() {
        let mut invalid_ob3 = EXAMPLE_BASIC_OB3.clone();

        *invalid_ob3.get_mut("type").unwrap() = json!(["UnknownType"]);

        let result = validate_credential_types(&invalid_ob3);
        assert!(result.is_ok());
    }
}
