// This folder is contains the lazy static ref Validators compiled from the JSON Schemas as to ensure easy compilation into the executable binary without the hassle of carrying over the JSON Schemafiles.
// Furthermore, it contains the items and functions needed for JSON Schema validation of credentials.
use jsonschema::{Retrieve, Uri, ValidationError, Validator};
use lazy_static::lazy_static;
use log::{debug, error, info, warn};
use serde_json::Value;
use std::fs::{self};
use std::path::PathBuf;

use crate::error::AppError;

const JSONSCHEMAS_DIR: &str = "src/jsonschemas";

lazy_static! {
    pub static ref VERIFIABLE_CREDENTIAL_V1_1_VALIDATOR: Validator =
        compile_validator(include_str!("VerifiableCredentialV1_1.json"))
            .expect("Failed to compile VerifiableCredentialV1_1 JSON Schema");
    pub static ref VERIFIABLE_CREDENTIAL_V2_VALIDATOR: Validator =
        compile_validator(include_str!("VerifiableCredentialV2.json"))
            .expect("Failed to compile VerifiableCredentialV2 JSON Schema");
    pub static ref EUROPEAN_DIGITAL_CREDENTIAL_V3_3_VALIDATOR: Validator =
        compile_validator(include_str!("EuropeanDigitalCredentialV3_3.json"))
            .expect("Failed to compile EuropeanDigitalCredentialV3_3 JSON Schema");
    pub static ref OPEN_BADGE_CREDENTIAL_V3_VALIDATOR: Validator =
        compile_validator(include_str!("OpenBadgeCredentialV3.json"))
            .expect("Failed to compile OpenBadgeCredentialV3 JSON Schema");
}

/// Helper function to create the static ref Validators from JSON Schema files.
fn compile_validator(json_schema_str: &str) -> Result<Validator, AppError> {
    let json_schema: Value = serde_json::from_str(json_schema_str)
        .map_err(|_| AppError::Error("Failed to convert JSON Schema &str to serde_json::Value".to_string()))?;

    // Define the relative path to our jsonschema folder needed for the LocalRetriever
    let jsonschema_dir = std::env::current_dir().unwrap().join(JSONSCHEMAS_DIR);

    // Select correct draft version for JSON Schema Validator and construct schema with LocalRetriever
    let schema = match json_schema
        .get("$schema")
        .and_then(|value| value.as_str().map(ToString::to_string))
        .ok_or(AppError::Error("Invalid or missing \"$schema\" field".to_string()))?
        .as_str()
    {
        "https://json-schema.org/draft/2019-09/schema#" => jsonschema::draft201909::options()
            .with_retriever(LocalRetriever {
                base_path: jsonschema_dir.clone(),
            })
            .build(&json_schema)
            .map_err(|_| {
                AppError::Error(format!(
                    "Failed to compile JSON Schema from serde_json::Value: {json_schema}"
                ))
            })?,
        // Default to draft 2020-12
        _ => jsonschema::draft202012::options()
            .with_retriever(LocalRetriever {
                base_path: jsonschema_dir.clone(),
            })
            .build(&json_schema)
            .map_err(|_| {
                AppError::Error(format!(
                    "Failed to compile JSON Schema from serde_json::Value: {json_schema}"
                ))
            })?,
    };

    Ok(schema)
}

/// Validate supported credential types against their corresponding JSON Schema.
/// This function is only capable of validating VC's and subsequent Credential Formats/Types.
/// All VC's must have a `type` field, which is either a string or an array of strings.
pub fn validate_credential_types(data: &Value) -> Result<(), AppError> {
    // Data should be passed as a serde_json::Value::Object as per the VerifiableCredentialRecord::try_new() method.
    // However this block double checks and emits the correct error message when this is not the case.
    // Serde_json::Value typing is error prone and sometimes Objects are wrapped as Strings resulting in Value::String.
    // Therefore, we try to "unwrap" the String type here once before also failing on that type.
    let data = match data {
        Value::String(str) => {
            let parsed_data = serde_json::from_str::<Value>(str).map_err(|_| AppError::InvalidCredentialFormatError)?;
            if !parsed_data.is_object() {
                return Err(AppError::InvalidCredentialFormatError);
            }
            parsed_data
        }
        Value::Object(_) => data.clone(),
        _ => {
            return Err(AppError::InvalidCredentialFormatError);
        }
    };

    let type_field = data.get("type");

    match type_field {
        Some(_type) if !_type.is_null() => {
            match serde_json::from_value::<StringOrArray>(_type.clone())
                .map_err(|_| AppError::InvalidCredentialFormatError)?
            {
                StringOrArray::String(credential_type) => Ok(credential_type.validate(&data)?),
                StringOrArray::Array(credential_type_array) => credential_type_array
                    .iter()
                    .try_for_each(|credential_type| credential_type.validate(&data)),
            }
        }
        _ => {
            debug!("No credential type found, skipping validation");
            Ok(())
        }
    }
}

// Structs

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum StringOrArray {
    String(CredentialType),
    Array(Vec<CredentialType>),
}

#[derive(serde::Deserialize, PartialEq, Debug, strum::Display)]
pub enum CredentialType {
    VerifiableCredential,
    #[serde(alias = "AchievementCredential")]
    OpenBadgeCredential,
    EuropeanDigitalCredential,
    #[serde(other)]
    Unknown,
}

#[derive(serde::Deserialize, PartialEq, Debug, strum::Display)]
pub enum CredentialTypeVersion {
    VerifiableCredentialV1_1,
    VerifiableCredentialV2,
    EuropeanDigitalCredentialV3_3,
    OpenBadgeCredentialV3,
    #[serde(other)]
    Unknown,
}

impl CredentialTypeVersion {
    pub fn get_validator(&self) -> Result<&'static Validator, AppError> {
        match self {
            CredentialTypeVersion::VerifiableCredentialV1_1 => Ok(&VERIFIABLE_CREDENTIAL_V1_1_VALIDATOR),
            CredentialTypeVersion::VerifiableCredentialV2 => Ok(&VERIFIABLE_CREDENTIAL_V2_VALIDATOR),
            CredentialTypeVersion::EuropeanDigitalCredentialV3_3 => Ok(&EUROPEAN_DIGITAL_CREDENTIAL_V3_3_VALIDATOR),
            CredentialTypeVersion::OpenBadgeCredentialV3 => Ok(&OPEN_BADGE_CREDENTIAL_V3_VALIDATOR),
            CredentialTypeVersion::Unknown => Err(AppError::InvalidCredentialFormatError),
        }
    }
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
            CredentialType::EuropeanDigitalCredential => {
                // The current provided ELM EDC schema contains no specific context value, only the context value of the VC DM 1.1 it builds upon.
                // Therefore, there is no way to determine the version except for the description.
                // For now we will shortcut this as ELM schemas are still in development and only time will tell the best way to determine versions once multiple schemas are published.
                Ok(CredentialTypeVersion::EuropeanDigitalCredentialV3_3)
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
                let errors: Vec<ValidationError> = version.get_validator()?.iter_errors(data).collect();
                if !errors.is_empty() {
                    Err(AppError::Error(format!(
                        "The data is invalid according to the given JSON Schema: {errors:?}"
                    )))
                } else {
                    info!("Credential type: {self:?} successfully validated against corresponding JSON Schema");
                    Ok(())
                }
            }
        }
    }
}

/// This struct is solely used to implement the `Retrieve` trait from the `jsonschema` crate,
/// allowing us to load local JSON Schema files referenced via $ref in our JSON Schemas
struct LocalRetriever {
    base_path: PathBuf,
}

/// Implementation of the `Retrieve` trait for loading local JSON Schema files
impl Retrieve for LocalRetriever {
    fn retrieve(&self, uri: &Uri<String>) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        // Convert the URI/filename to a path in the resources folder
        let file_path = self.base_path.join(uri.path().to_string().trim_start_matches('/'));
        let content = fs::read_to_string(file_path)?;
        let json = serde_json::from_str(&content)?;
        Ok(json)
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
        static ref EXAMPLE_BASIC_ELM_EDC: Value = json!({
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://elm.edc.nl/credentials/v3.3/context.json"
            ],
            "id": "http://example.com/credentials/elm-edc-001",
            "type": [
                "VerifiableCredential",
                "EuropeanDigitalCredential"
            ],
            "name": "ELM EDC Example Credential",
            "issuer": {
                "id": "https://example.com/issuers/123456",
                "type": "Organisation",
                "legalName": { "en": "ELM Example University" },
                "location": {
                "type": "Location",
                "address": {
                    "type": "Address",
                    "countryCode": { "id": "http://publications.europa.eu/resource/authority/country/NLD" }
                }
                }
            },
            "issuanceDate": "2023-01-01T00:00:00Z",
            "issued": "2023-01-01T00:00:00Z",
            "validFrom": "2023-01-01T00:00:00Z",
            "credentialProfiles": [
                { "id": "http://data.europa.eu/snb/model/elm/2.0" }
            ],
            "displayParameter": {
                "type": "DisplayParameter",
                "title": { "en": "Example Credential" },
                "description": { "en": "A demo credential" },
                "language": [ { "id": "http://publications.europa.eu/resource/authority/language/ENG" } ],
                "primaryLanguage": { "id": "http://publications.europa.eu/resource/authority/language/ENG" },
                "individualDisplay": [
                {
                    "type": "IndividualDisplay",
                    "language": {
                        "id": "http://publications.europa.eu/resource/authority/language/ENG",
                        "type": "Concept"
                    },
                    "displayDetail": [
                    {
                        "type": "DisplayDetail",
                        "image": {
                            "type": "MediaObject",
                            "contentType": { "id": "http://publications.europa.eu/resource/authority/file-type/PNG" },
                            "contentEncoding": { "id": "http://publications.europa.eu/resource/authority/encoding/BASE64" },
                            "content": "iVBOR..."
                        },
                        "page": 1
                    }
                    ]
                }
                ]
            },
            "credentialSchema": [{
                "id": "https://elm.edc.nl/credentials/v3.3/schema.json",
                "type": "JsonSchema"
            }],
            "credentialSubject": {
                "id": "did:example:abcdef1234567890",
                "type": "Person",
                "fullName": { "en": "John Doe" }
            }
        });
    }

    #[test]
    fn credential_schema_validation_elm_edc_ok() {
        let result = validate_credential_types(&EXAMPLE_BASIC_ELM_EDC);
        assert!(result.is_ok());
    }

    #[test]
    fn credential_schema_validation_obv3_ok() {
        let result = validate_credential_types(&EXAMPLE_BASIC_OB3);
        assert!(result.is_ok());
    }

    #[test]
    fn credential_schema_validation_obv3_err() {
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
