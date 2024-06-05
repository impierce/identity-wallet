use did_manager::Resolver;
use identity_credential::domain_linkage::{DomainLinkageConfiguration, JwtDomainLinkageValidator};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{
    core::{FromJson, ToJson},
    credential::JwtCredentialValidationOptions,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

// https://wiki.iota.org/identity.rs/how-tos/domain-linkage/create-and-verify/#verifying-a-did-and-domain-linkage
pub async fn validate_domain_linkage(mut url: url::Url, did: &str) -> ValidationResult {
    // let scheme_host = format!("{}://{}", url.scheme(), url.host_str().unwrap());
    // let mut url = identity_iota::core::Url::from_str(&scheme_host).unwrap();

    // let mut url = url.origin().ascii_serialization();
    info!("url: {:?}", url);
    info!("did: {:?}", did);

    println!("url: {:?}", url.to_string());

    // Option 1: identity.rs only supports JWTs in the `linked_dids` array (not JSON-LD)
    // let domain_linkage_configuration = DomainLinkageConfiguration::fetch_configuration(url.clone()).await;
    // info!("domain_linkage_configuration: {:?}", domain_linkage_configuration);

    // Option 2: use reqwest directly to parse the `linked_dids` exhaustively
    url.set_path(".well-known/did-configuration.json");
    info!("url: {:?}", url.to_string());

    println!("url: {:?}", url.to_string());
    println!(
        "url (parsed): {:?}",
        &identity_iota::core::Url::from_str(url.as_str()).unwrap()
    );

    let did_configuration = reqwest::get(url.to_string()).await;

    warn!("did_configuration: {:?}", did_configuration);

    if did_configuration.is_err() {
        return ValidationResult {
            status: ValidationStatus::Unknown,
            message: did_configuration.err().map(|e| e.to_string()),
        };
    };

    let json = did_configuration.unwrap().json::<serde_json::Value>().await;

    warn!("json: {:?}", json);

    if json.is_err() {
        return ValidationResult {
            status: ValidationStatus::Unknown,
            message: json.err().map(|e| e.to_string()),
        };
    };

    let mut did_configuration = json.unwrap();

    info!("did_configuration: {:?}", did_configuration);

    if let serde_json::Value::Object(ref mut root) = did_configuration {
        if let Some(serde_json::Value::Array(ref mut linked_dids)) = root.get_mut("linked_dids") {
            linked_dids.retain(|did| matches!(did, serde_json::Value::String(_)));
            info!("Removed non-string values from `linked_dids`");
        }
    }

    info!("did_configuration: {}", did_configuration.to_json_pretty().unwrap());
    let domain_linkage_configuration = DomainLinkageConfiguration::from_json_value(did_configuration);
    // === End

    if domain_linkage_configuration.is_err() {
        info!(
            "Error fetching domain linkage configuration: {:?}",
            domain_linkage_configuration.err()
        );
        return ValidationResult {
            status: ValidationStatus::Unknown,
            message: None,
        };
    }

    let validator = JwtDomainLinkageValidator::with_signature_verifier(EdDSAJwsVerifier::default());

    let resolver = Resolver::new().await;
    let document = resolver.resolve(did).await.unwrap();

    info!("document: {:?}", document);

    url.set_scheme("https").unwrap();
    url.set_host(Some("identity.foundation")).unwrap();
    let _ = url.set_port(None);
    let x = &identity_iota::core::Url::from_str(url.as_str());
    println!("x: {:?}", x);

    let res = validator.validate_linkage(
        &document,
        &domain_linkage_configuration.expect("Error should have been handled already"),
        &identity_iota::core::Url::from_str(url.as_str()).unwrap(),
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

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

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
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!(
                        {
                            "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                            "linked_dids": [
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
                            ]
                        }
                    )),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let result = validate_domain_linkage(url::Url::parse(&mock_server.uri()).unwrap(), "did:foo:bar").await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Unknown,
                message: None,
            }
        );
    }

    #[tokio::test]
    async fn when_well_known_contains_json_ld_and_jwt_then_skip_json_ld_and_return_success() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!(
                        {
                            "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                            "linked_dids": [
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
                              },
                              "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNI3o2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSJ9.eyJleHAiOjE3NjQ4NzkxMzksImlzcyI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNIiwibmJmIjoxNjA3MTEyNzM5LCJzdWIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly9pZGVudGl0eS5mb3VuZGF0aW9uLy53ZWxsLWtub3duL2RpZC1jb25maWd1cmF0aW9uL3YxIl0sImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rb1RIc2dOTnJieThKekNOUTFpUkx5VzVRUTZSOFh1dTZBQThpZ0dyTVZQVU0iLCJvcmlnaW4iOiJpZGVudGl0eS5mb3VuZGF0aW9uIn0sImV4cGlyYXRpb25EYXRlIjoiMjAyNS0xMi0wNFQxNDoxMjoxOS0wNjowMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjAtMTItMDRUMTQ6MTI6MTktMDY6MDAiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJEb21haW5MaW5rYWdlQ3JlZGVudGlhbCJdfX0.aUFNReA4R5rcX_oYm3sPXqWtso_gjPHnWZsB6pWcGv6m3K8-4JIAvFov3ZTM8HxPOrOL17Qf4vBFdY9oK0HeCQ"
                            ]
                        }
                    )),
            )
            .expect(1)
            .mount(&mock_server)
            .await;

        let result = validate_domain_linkage(
            url::Url::parse(&mock_server.uri()).unwrap(),
            "did:key:z6MkoTHsgNNrby8JzCNQ1iRLyW5QQ6R8Xuu6AA8igGrMVPUM",
        )
        .await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Success,
                message: None
            }
        );
    }

    #[tokio::test]
    async fn when_well_known_contains_unexpected_did_then_return_failure() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(".well-known/did-configuration.json"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!(
                        {
                            "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                            "linked_dids": [
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
                              },
                              "eyJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNI3o2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSJ9.eyJleHAiOjE3NjQ4NzkxMzksImlzcyI6ImRpZDprZXk6ejZNa29USHNnTk5yYnk4SnpDTlExaVJMeVc1UVE2UjhYdXU2QUE4aWdHck1WUFVNIiwibmJmIjoxNjA3MTEyNzM5LCJzdWIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly9pZGVudGl0eS5mb3VuZGF0aW9uLy53ZWxsLWtub3duL2RpZC1jb25maWd1cmF0aW9uL3YxIl0sImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rb1RIc2dOTnJieThKekNOUTFpUkx5VzVRUTZSOFh1dTZBQThpZ0dyTVZQVU0iLCJvcmlnaW4iOiJpZGVudGl0eS5mb3VuZGF0aW9uIn0sImV4cGlyYXRpb25EYXRlIjoiMjAyNS0xMi0wNFQxNDoxMjoxOS0wNjowMCIsImlzc3VhbmNlRGF0ZSI6IjIwMjAtMTItMDRUMTQ6MTI6MTktMDY6MDAiLCJpc3N1ZXIiOiJkaWQ6a2V5Ono2TWtvVEhzZ05OcmJ5OEp6Q05RMWlSTHlXNVFRNlI4WHV1NkFBOGlnR3JNVlBVTSIsInR5cGUiOlsiVmVyaWZpYWJsZUNyZWRlbnRpYWwiLCJEb21haW5MaW5rYWdlQ3JlZGVudGlhbCJdfX0.aUFNReA4R5rcX_oYm3sPXqWtso_gjPHnWZsB6pWcGv6m3K8-4JIAvFov3ZTM8HxPOrOL17Qf4vBFdY9oK0HeCQ"
                            ]
                        }
                    )),
            )
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
}
