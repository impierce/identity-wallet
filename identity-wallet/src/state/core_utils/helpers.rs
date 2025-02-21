use crate::{error::AppError, state::SUPPORTED_CRED_TYPE_SCHEMAS};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use jsonschema::ValidationError;
use log::warn;
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

/// Validate supported credential types against their corresponding Json schema.
pub fn credential_schema_validation(data: &Value) -> Result<(), AppError> {
    let credential_type_array: Vec<String> = data
        .get("type")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f.as_str().map(String::from).unwrap())
        .collect(); // TODO: remove unwrap. This doesn't work for mDoc, only VC's.

    if SUPPORTED_CRED_TYPE_SCHEMAS.iter().any(|x| credential_type_array.contains(&x.to_string())) {
        for mut supported_cred_type in SUPPORTED_CRED_TYPE_SCHEMAS {
            if credential_type_array.contains(&supported_cred_type.to_string()) {
                println!("Supported schema found for Credential type: {:?}", supported_cred_type);

                // OpenBadgeCredentials can be typed as "OpenBadgeCredential" or "AchievementCredential"
                if *supported_cred_type == "AchievementCredential" {
                    supported_cred_type = &"OpenBadgeCredential";
                }
    
                let json_schema_path = format!("resources/jsonschemas/{}.json", supported_cred_type);
                json_schema_validation(json_schema_path, data)?;                
            }
        }
    }
    else {
        println!("No supported schema found for Credential type");
        warn!(
            "No supported schema found for Credential type: {:?}",
            credential_type_array
        );
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

    let result = schema.iter_errors(&data);

    let errors: Vec<ValidationError> = result.collect();
    if !errors.is_empty() {
        println!("The data is invalid according to the given JsonSchema: {:?}", errors);
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

    #[test]
    fn credential_schema_validation_ok() {
        let file = File::open("resources/example_basic_obv3.json").unwrap();
        let rdr = std::io::BufReader::new(file);
        let data = serde_json::from_reader(rdr).unwrap();

        let result = credential_schema_validation(&data);
        assert!(result.is_ok());
    }

    #[test]
    fn credential_schema_validation_err() {
        let file = File::open("resources/example_basic_obv3.json").unwrap();
        let rdr = std::io::BufReader::new(file);
        let mut data: Value = serde_json::from_reader(rdr).unwrap();

        *data.get_mut("id").unwrap() = serde_json::json!(["InvalidType"]);

        let result = credential_schema_validation(&data);
        assert!(result.is_err());
    }
}
