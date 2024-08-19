use crate::state::core_utils::helpers::EcodedPublicKey;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use did_manager::Resolver;
use identity_credential::domain_linkage::{DomainLinkageConfiguration, JwtDomainLinkageValidator};
use identity_iota::{
    core::FromJson,
    credential::JwtCredentialValidationOptions,
    verification::{
        jwk::Jwk,
        jws::{
            JwsAlgorithm, JwsVerifier, SignatureVerificationError, SignatureVerificationErrorKind, VerificationInput,
        },
    },
};
use jsonwebtoken::{crypto::verify, Algorithm, DecodingKey, Validation};
use log::info;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/user_prompt/ValidationResult.ts")]
pub struct ValidationResult {
    pub(crate) status: ValidationStatus,
    pub(crate) message: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/user_prompt/ValidationStatus.ts")]
pub enum ValidationStatus {
    Success,
    Failure,
    #[default]
    Unknown,
}

struct Verifier;

impl JwsVerifier for Verifier {
    fn verify(&self, input: VerificationInput, public_key: &Jwk) -> Result<(), SignatureVerificationError> {
        use JwsAlgorithm::*;
        use SignatureVerificationErrorKind::*;

        let (algorithm, decoding_key) = match input.alg {
            EdDSA => (
                Algorithm::EdDSA,
                DecodingKey::from_ed_der(&public_key.encoded_public_key().unwrap()),
            ),
            ES256 => (
                Algorithm::ES256,
                DecodingKey::from_ec_der(&public_key.encoded_public_key().unwrap()),
            ),
            _ => return Err(SignatureVerificationError::new(UnsupportedAlg)),
        };

        let decoding_key =
            DecodingKey::from_jwk(&jsonwebtoken::jwk::Jwk::from_json_value(serde_json::json!(public_key)).unwrap())
                .map_err(|_| KeyDecodingFailure)?;

        let mut validation = Validation::new(algorithm);
        validation.validate_aud = false;
        validation.required_spec_claims.clear();

        match verify(
            &URL_SAFE_NO_PAD.encode(input.decoded_signature),
            &input.signing_input,
            &decoding_key,
            algorithm,
        ) {
            Ok(_) => Ok(()),
            Err(_) => Err(SignatureVerificationError::new(
                // TODO: more fine-grained error handling?
                SignatureVerificationErrorKind::InvalidSignature,
            )),
        }
    }
}

/// https://wiki.iota.org/identity.rs/how-tos/domain-linkage/create-and-verify/#verifying-a-did-and-domain-linkage
pub async fn validate_domain_linkage(url: url::Url, did: &str) -> ValidationResult {
    let did_configuration_result = fetch_configuration(url.clone()).await;

    let domain_linkage_configuration = match did_configuration_result {
        Ok(did_config) => did_config,
        Err(e) => {
            return ValidationResult {
                status: ValidationStatus::Unknown,
                message: Some(e.to_string()),
            };
        }
    };

    let validator = JwtDomainLinkageValidator::with_signature_verifier(Verifier);

    let resolver = Resolver::new().await;

    let document = match resolver.resolve(did).await {
        Ok(document) => document,
        Err(e) => {
            return ValidationResult {
                status: ValidationStatus::Unknown,
                message: Some(e.to_string()),
            };
        }
    };

    let url = identity_iota::core::Url::from(url);

    let res = validator.validate_linkage(
        &document,
        &domain_linkage_configuration,
        &url,
        &JwtCredentialValidationOptions::default(),
    );

    if res.is_ok() {
        ValidationResult {
            status: ValidationStatus::Success,
            message: None,
        }
    } else {
        ValidationResult {
            status: ValidationStatus::Failure,
            message: res.err().map(|e| e.to_string()),
        }
    }
}

/// Acts as a replacement for `fetch_configuration()` from `identity_credential` which fails on JSON-LD inside `linked_dids`.
/// This implementation is also less strict (allows `http` scheme, does not fail on JSON-LD)
/// The resource at the `.well-known` endpoint is fetched and any non-string values from `linked_dids` before deserializing.
/// Returns a `DomainLinkageConfiguration` which can be verified using a verifier from `identity_credential`.
async fn fetch_configuration(mut url: url::Url) -> Result<DomainLinkageConfiguration, String> {
    // 1. Prepare the URL
    url.set_fragment(None);
    url.set_query(None);
    url.set_path(".well-known/did-configuration.json");

    info!("Fetching DID configuration from: {}", url);

    // 2. Fetch the resource
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    // 3. Parse to JSON value (mutable)
    let mut json = response.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;

    // 4. Remove all non-string values from `linked_dids` (JSON-LD)
    if let serde_json::Value::Object(ref mut root) = json {
        if let Some(serde_json::Value::Array(ref mut linked_dids)) = root.get_mut("linked_dids") {
            linked_dids.retain(|did| matches!(did, serde_json::Value::String(_)));
            info!("Removed non-string values from `linked_dids`");
        }
    }

    // 5. Deserialize to `DomainLinkageConfiguration`
    let config = DomainLinkageConfiguration::from_json_value(json).map_err(|e| e.to_string())?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    // https://identity.foundation/.well-known/did-configuration.json
    const LINKED_DID_JSON_LD: &str = r#"
        {
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://identity.foundation/.well-known/did-configuration/v1"
            ],
            "issuer": "did:key:z6MkoTHsgNNrby8JzCNQ1iRLyW5QQ6R8Xuu6AA8igGrMVPUM",
            "issuanceDate": "2020-12-04T14:08:28-06:00",
            "expirationDate": "2025-12-04T14:08:28-06:00",
            "type": ["VerifiableCredential", "DomainLinkageCredential"],
            "credentialSubject": {
                "id": "did:key:z6MkoTHsgNNrby8JzCNQ1iRLyW5QQ6R8Xuu6AA8igGrMVPUM",
                "origin": "https://identity.foundation"
            },
            "proof": {
                "type": "Ed25519Signature2018",
                "created": "2020-12-04T20:08:28.540Z",
                "jws": "eyJhbGciOiJFZERTQSIsImI2NCI6ZmFsc2UsImNyaXQiOlsiYjY0Il19..D0eDhglCMEjxDV9f_SNxsuU-r3ZB9GR4vaM9TYbyV7yzs1WfdUyYO8rFZdedHbwQafYy8YOpJ1iJlkSmB4JaDQ",
                "proofPurpose": "assertionMethod",
                "verificationMethod": "did:key:z6MkoTHsgNNrby8JzCNQ1iRLyW5QQ6R8Xuu6AA8igGrMVPUM#z6MkoTHsgNNrby8JzCNQ1iRLyW5QQ6R8Xuu6AA8igGrMVPUM"
            }
        }
    "#;

    // https://identity.foundation/.well-known/did-configuration.json
    const LINKED_DID_JWT: &str = "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNI3o2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSJ9.eyJleHAiOjE3NjQ4NzkxMzksImlzcyI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNIiwibmJmIjoxNjA3MTEyNzM5LCJzdWIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly9pZGVudGl0eS5mb3VuZGF0aW9uLy53ZWxsLWtub3duL2RpZC1jb25maWd1cmF0aW9uL3YxIl0sImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rb1RIc2dOTnJieThKekNOUTFpUkx5VzVRUTZSOFh1dTZBQThpZ0dyTVZQVU0iLCJvcmlnaW4iOiJpZGVudGl0eS5mb3VuZGF0aW9uIn0sImV4cGlyYXRpb25EYXRlIjoiMjAyNS0xMi0wNFQxNDoxMjoxOS0wNjowMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjAtMTItMDRUMTQ6MTI6MTktMDY6MDAiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJEb21haW5MaW5rYWdlQ3JlZGVudGlhbCJdfX0.aUFNReA4R5rcX_oYm3sPXqWtso_gjPHnWZsB6pWcGv6m3K8-4JIAvFov3ZTM8HxPOrOL17Qf4vBFdY9oK0HeCQ";

    #[tokio::test]
    async fn when_no_well_known_then_return_validation_status_unknown() {
        let mock_server = MockServer::start().await;

        let result = validate_domain_linkage(url::Url::parse(&mock_server.uri()).unwrap(), "did:foo:bar").await;

        assert_eq!(result.status, ValidationStatus::Unknown);
        assert!(result.message.is_some());
    }

    #[tokio::test]
    async fn when_well_known_only_contains_json_ld_credential_then_return_unknown() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                    "linked_dids": [
                        serde_json::from_str::<serde_json::Value>(LINKED_DID_JSON_LD).unwrap(),
                    ]
                }
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        let result = validate_domain_linkage(url::Url::parse(&mock_server.uri()).unwrap(), "did:foo:bar").await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Unknown,
                message: Some("failed to decode JSON".to_string()),
            }
        );
    }

    #[tokio::test]
    async fn when_well_known_contains_json_ld_and_jwt_then_remove_json_ld_during_fetch() {
        // Mocking a `DomainLinkageCredential` in a local test is difficult since the `mock_server` does not have an actual domain.
        // We thereby only test if `fetch_configuration()` builds a `DomainLinkageConfiguration` that could be verified.

        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                    "linked_dids": [
                        serde_json::from_str::<serde_json::Value>(LINKED_DID_JSON_LD).unwrap(),
                        LINKED_DID_JWT
                    ]
                }
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        let domain_linkage_configuration = fetch_configuration(mock_server.uri().parse().unwrap()).await.unwrap();

        assert!(domain_linkage_configuration.linked_dids().len() == 1);
    }

    #[tokio::test]
    async fn when_well_known_contains_unexpected_did_then_return_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                    "linked_dids": [
                        LINKED_DID_JWT
                    ]
                }
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        let result = validate_domain_linkage(
            url::Url::parse(&mock_server.uri()).unwrap(),
            "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
        )
        .await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Failure,
                message: Some("invalid issuer DID".to_string())
            }
        );
    }

    #[tokio::test]
    async fn successfully_ignores_url_parts_other_than_origin() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!(
                {
                    "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                    "linked_dids": [
                        LINKED_DID_JWT
                    ]
                }
            )))
            .expect(1)
            .mount(&mock_server)
            .await;

        let mut url = url::Url::parse(&mock_server.uri()).unwrap();
        url.set_fragment(Some("foobar"));
        url.set_query(Some("page=1"));

        let result = validate_domain_linkage(url, "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp").await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Failure,
                message: Some("invalid issuer DID".to_string())
            }
        );
    }
}
