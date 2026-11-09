use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use did_manager::Resolver;
use futures::future::join_all;
use hickory_resolver::{
    config::{ResolverConfig, CLOUDFLARE},
    net::runtime::TokioRuntimeProvider,
    proto::rr::{RData, RecordType},
    TokioResolver,
};
use identity_credential::domain_linkage::{DomainLinkageConfiguration, JwtDomainLinkageValidator};
use identity_iota::{
    core::{FromJson, ToJson},
    credential::JwtCredentialValidationOptions,
    document::CoreDocument,
    verification::{
        jwk::Jwk as IotaIdentityJwk,
        jws::{JwsVerifier, SignatureVerificationError, SignatureVerificationErrorKind, VerificationInput},
    },
};
use jsonwebtoken::{crypto::verify, jwk::Jwk as JsonWebTokenJwk, Algorithm, DecodingKey, Validation};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{str::FromStr, time::Duration};
use ts_rs::TS;
use url::{Host, Url};

use crate::{
    http_client::fetch_public_destination, state::did::validate_linked_verifiable_presentations::get_linked_domains,
};

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/ValidationResult.ts")]
pub struct ValidationResult {
    pub(crate) status: ValidationStatus,
    pub(crate) url: url::Url,
    pub(crate) name: Option<String>,
    #[ts(type = "string", optional)]
    pub(crate) logo_uri: Option<url::Url>,
    pub(crate) issuance_date: Option<String>,
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

/// This `Verifier` uses `jsonwebtoken` under the hood to verify verification input.
pub struct Verifier;
impl JwsVerifier for Verifier {
    fn verify(&self, input: VerificationInput, public_key: &IotaIdentityJwk) -> Result<(), SignatureVerificationError> {
        use SignatureVerificationErrorKind::*;

        debug!("Verifying input signature with alg: {}", input.alg);

        let algorithm =
            Algorithm::from_str(&input.alg.to_string()).map_err(|_| SignatureVerificationError::new(UnsupportedAlg))?;

        // Convert the `IotaIdentityJwk` first into a `JsonWebTokenJwk` and then into a `DecodingKey`.
        let decoding_key = public_key
            .to_json()
            .ok()
            .and_then(|public_key| JsonWebTokenJwk::from_json(&public_key).ok())
            .and_then(|jwk| DecodingKey::from_jwk(&jwk).ok())
            .ok_or(SignatureVerificationError::new(KeyDecodingFailure))?;

        let mut validation = Validation::new(algorithm);
        validation.validate_aud = false;
        validation.required_spec_claims.clear();

        match verify(
            &URL_SAFE_NO_PAD.encode(input.decoded_signature),
            &input.signing_input,
            &decoding_key,
            algorithm,
        ) {
            Ok(true) => {
                debug!("Signature successfully verified");
                Ok(())
            }
            Err(_) | Ok(false) => Err(SignatureVerificationError::new(
                // TODO: more fine-grained error handling?
                InvalidSignature,
            )),
        }
    }
}

impl ValidationResult {
    fn new(status: ValidationStatus, url: Url, message: Option<String>) -> Self {
        ValidationResult {
            status,
            url,
            name: None,
            logo_uri: None,
            issuance_date: None,
            message,
        }
    }
}

/// Validates the domain linkage between `did` and the origin of `url`: the DID Configuration resource at that origin
/// must hold a Domain Linkage Credential of `did` for exactly that origin.
///
/// https://wiki.iota.org/identity.rs/how-tos/domain-linkage/create-and-verify/#verifying-a-did-and-domain-linkage
pub async fn validate_domain_linkage(resolver: &Resolver, url: Url, did: &str) -> ValidationResult {
    let configuration = match fetch_configuration(url.clone()).await {
        Ok(configuration) => configuration,
        Err(err) => {
            return ValidationResult::new(
                ValidationStatus::Unknown,
                url,
                Some(format!("Error while fetching configuration: {err}")),
            )
        }
    };

    let document = match resolver.resolve(did).await {
        Ok(document) => document,
        Err(err) => return ValidationResult::new(ValidationStatus::Unknown, url, Some(err.to_string())),
    };

    debug!("Resolved document: {document:?}");

    check_linkage(&document, &configuration, url)
}

/// How many `LinkedDomains` origins of a DID document are checked, so that a long list cannot stall the prompt.
const MAX_LINKED_DOMAINS: usize = 5;

/// Validates that the service at `url`, which the wallet connects to, acts for a domain that `did` is linked to.
///
/// Passes when either:
/// - `did` is linked to the origin of `url` itself (see [`validate_domain_linkage`]), or
/// - `did` is linked to one of the origins in its DID document's `LinkedDomains` service, and that origin's host is a
///   `CNAME`, directly or through further `CNAME`s, of the host of `url`.
///
/// The second case lets a DID stay on the host of the service (such as `did:web:polaris.dev2.impierce.com`) while it
/// is linked to the domain of the organization behind it (such as `identity.polaris.impierce.com`). The Domain
/// Linkage Credential proves that the controller of that domain links it to the DID. The `CNAME` closes the loop: only
/// the controller of the domain can point it at the service the wallet is talking to.
pub async fn validate_connection_domain(resolver: &Resolver, url: Url, did: &str) -> ValidationResult {
    let document = match resolver.resolve(did).await {
        Ok(document) => document,
        Err(err) => return ValidationResult::new(ValidationStatus::Unknown, url, Some(err.to_string())),
    };

    debug!("Resolved document: {document:?}");

    validate_connection_domain_with(&document, url, &DnsCnameResolver).await
}

async fn validate_connection_domain_with(
    document: &CoreDocument,
    url: Url,
    cnames: &impl CnameResolver,
) -> ValidationResult {
    let direct = validate_linkage_at(document, url.clone()).await;
    if direct.status == ValidationStatus::Success {
        return direct;
    }

    let Some(connection_host) = url.host() else {
        return direct;
    };

    let linked_domains: Vec<Url> = get_linked_domains(document)
        .into_iter()
        .filter(|linked_domain| linked_domain.origin() != url.origin())
        .take(MAX_LINKED_DOMAINS)
        .collect();

    let linked_results = join_all(linked_domains.into_iter().map(|linked_domain| {
        let connection_host = connection_host.to_owned();
        async move {
            let result = validate_linkage_at(document, linked_domain.clone()).await;
            if result.status != ValidationStatus::Success {
                return result;
            }

            let Some(linked_host) = linked_domain.host().map(|host| host.to_owned()) else {
                return ValidationResult::new(
                    ValidationStatus::Failure,
                    linked_domain,
                    Some("linked domain has no host".to_string()),
                );
            };

            match aliases(cnames, &linked_host, &connection_host).await {
                Ok(true) => ValidationResult::new(
                    ValidationStatus::Success,
                    linked_domain,
                    Some(format!(
                        "{linked_host} is linked to the DID and points to {connection_host}"
                    )),
                ),
                Ok(false) => ValidationResult::new(
                    ValidationStatus::Failure,
                    linked_domain,
                    Some(format!(
                        "{linked_host} is linked to the DID but is not a CNAME of {connection_host}"
                    )),
                ),
                Err(err) => ValidationResult::new(
                    ValidationStatus::Unknown,
                    linked_domain,
                    Some(format!("CNAME lookup for {linked_host} failed: {err}")),
                ),
            }
        }
    }))
    .await;

    if let Some(success) = linked_results
        .iter()
        .find(|result| result.status == ValidationStatus::Success)
    {
        info!("Domain of {url} verified through linked domain {}", success.url);
        return success.clone();
    }

    let results: Vec<ValidationResult> = std::iter::once(direct).chain(linked_results).collect();
    let status = if results.iter().any(|result| result.status == ValidationStatus::Failure) {
        ValidationStatus::Failure
    } else {
        ValidationStatus::Unknown
    };
    let message = results
        .iter()
        .filter_map(|result| {
            let origin = result.url.origin().ascii_serialization();
            result.message.as_ref().map(|message| format!("{origin}: {message}"))
        })
        .collect::<Vec<_>>()
        .join("; ");

    ValidationResult::new(status, url, (!message.is_empty()).then_some(message))
}

/// Validates the domain linkage between the DID of `document` and the origin of `url`.
async fn validate_linkage_at(document: &CoreDocument, url: Url) -> ValidationResult {
    match fetch_configuration(url.clone()).await {
        Ok(configuration) => check_linkage(document, &configuration, url),
        Err(err) => ValidationResult::new(
            ValidationStatus::Unknown,
            url,
            Some(format!("Error while fetching configuration: {err}")),
        ),
    }
}

fn check_linkage(document: &CoreDocument, configuration: &DomainLinkageConfiguration, url: Url) -> ValidationResult {
    let validator = JwtDomainLinkageValidator::with_signature_verifier(Verifier);

    match validator.validate_linkage(
        document,
        configuration,
        &identity_iota::core::Url::from(url.clone()),
        &JwtCredentialValidationOptions::default(),
    ) {
        Ok(()) => ValidationResult::new(ValidationStatus::Success, url, None),
        Err(err) => ValidationResult::new(ValidationStatus::Failure, url, Some(err.to_string())),
    }
}

/// How many `CNAME` records are followed before giving up, well above any real chain of aliases.
const MAX_CNAME_HOPS: usize = 8;

/// Whether `alias` is `target` or leads to it through a chain of `CNAME` records.
///
/// Only names are followed: an IP address has no `CNAME`, so it only matches itself.
async fn aliases(cnames: &impl CnameResolver, alias: &Host, target: &Host) -> Result<bool, String> {
    let target = normalize_host(target);
    let mut name = normalize_host(alias);

    for _ in 0..=MAX_CNAME_HOPS {
        if name == target {
            return Ok(true);
        }
        if !matches!(alias, Host::Domain(_)) {
            return Ok(false);
        }
        match cnames.cname(&name).await? {
            Some(next) => name = normalize_name(&next),
            None => return Ok(false),
        }
    }

    warn!("Gave up following the CNAME chain of {alias} after {MAX_CNAME_HOPS} records");
    Ok(false)
}

fn normalize_host(host: &Host) -> String {
    normalize_name(&host.to_string())
}

/// Compares names the way DNS does: case-insensitively and without the trailing dot of a fully qualified name.
fn normalize_name(name: &str) -> String {
    name.trim_end_matches('.').to_ascii_lowercase()
}

#[async_trait]
trait CnameResolver: Sync {
    /// The target of the `CNAME` record of `name`, or `None` when `name` has none.
    async fn cname(&self, name: &str) -> Result<Option<String>, String>;
}

/// Looks up `CNAME` records with the resolvers of the operating system, or public resolvers where its configuration
/// cannot be read.
struct DnsCnameResolver;

/// Per lookup, so that a slow resolver cannot stall the prompt.
const DNS_TIMEOUT: Duration = Duration::from_secs(2);

impl DnsCnameResolver {
    /// Built per validation rather than shared, since a resolver's connections belong to the runtime they were made
    /// on.
    fn resolver() -> Result<TokioResolver, String> {
        let mut builder = TokioResolver::builder_tokio().unwrap_or_else(|err| {
            warn!("Failed to read the system DNS configuration, using public resolvers instead: {err}");
            TokioResolver::builder_with_config(
                ResolverConfig::udp_and_tcp(&CLOUDFLARE),
                TokioRuntimeProvider::default(),
            )
        });
        builder.options_mut().timeout = DNS_TIMEOUT;
        builder.options_mut().attempts = 1;
        builder.build().map_err(|err| err.to_string())
    }
}

#[async_trait]
impl CnameResolver for DnsCnameResolver {
    async fn cname(&self, name: &str) -> Result<Option<String>, String> {
        // The trailing dot makes the name fully qualified, so no search domain is appended to it.
        let fqdn = format!("{}.", normalize_name(name));

        match Self::resolver()?.lookup(fqdn.as_str(), RecordType::CNAME).await {
            Ok(lookup) => Ok(lookup.answers().iter().find_map(|record| match &record.data {
                RData::CNAME(cname) => Some(cname.0.to_ascii()),
                _ => None,
            })),
            Err(err) if err.is_no_records_found() || err.is_nx_domain() => Ok(None),
            Err(err) => Err(err.to_string()),
        }
    }
}

/// Acts as a replacement for `fetch_configuration()` from `identity_credential` which fails on JSON-LD inside `linked_dids`.
/// This implementation is also less strict (allows `http` scheme, does not fail on JSON-LD)
/// The resource at the `.well-known` endpoint is fetched and any non-string values from `linked_dids` before deserializing.
/// Returns a `DomainLinkageConfiguration` which can be verified using a verifier from `identity_credential`.
async fn fetch_configuration(mut url: Url) -> Result<DomainLinkageConfiguration, String> {
    // 1. Prepare the URL
    url.set_fragment(None);
    url.set_query(None);
    url.set_path(".well-known/did-configuration.json");

    debug!("Fetching DID configuration from: {url}");

    // 2. Fetch the resource. The URL is remote-supplied: from a QR code or from a DID document.
    let response = fetch_public_destination(url.clone())
        .await
        .map_err(|_| format!("failed to get response from resource url: {url}"))?;

    // 3. Parse to JSON value (mutable)
    let mut json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|_| "failed to parse response into JSON value".to_string())?;

    // 4. Remove all non-string values from `linked_dids` (JSON-LD)
    if let serde_json::Value::Object(ref mut root) = json {
        if let Some(serde_json::Value::Array(ref mut linked_dids)) = root.get_mut("linked_dids") {
            linked_dids.retain(|did| matches!(did, serde_json::Value::String(_)));
            debug!("Removed non-string values from `linked_dids`");
        }
    }

    // 5. Deserialize to `DomainLinkageConfiguration`
    let config = DomainLinkageConfiguration::from_json_value(json)
        .map_err(|_| "failed to deserialize DomainLinkageConfiguration from JSON".to_string())?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{persistence::STRONGHOLD, stronghold::StrongholdManager, subject::subject};

    use super::*;

    use identity_iota::verification::jws::JwsAlgorithm;
    use oid4vc::oid4vc_core::{Sign, Subject, Verify};

    use serde_json::json;
    use tempfile::NamedTempFile;
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

        let resolver = Resolver::new();

        let result =
            validate_domain_linkage(&resolver, url::Url::parse(&mock_server.uri()).unwrap(), "did:foo:bar").await;

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

        let resolver = Resolver::new();

        let url = url::Url::parse(&mock_server.uri()).unwrap();

        let result = validate_domain_linkage(&resolver, url.clone(), "did:foo:bar").await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Unknown,
                url,
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: Some(
                    "Error while fetching configuration: failed to deserialize DomainLinkageConfiguration from JSON"
                        .to_string()
                ),
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

        let resolver = Resolver::new();

        let url = url::Url::parse(&mock_server.uri()).unwrap();

        let result = validate_domain_linkage(
            &resolver,
            url.clone(),
            "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
        )
        .await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Failure,
                url,
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: Some("invalid semantic structure of the domain linkage configuration".to_string()),
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

        let resolver = Resolver::new();

        let result = validate_domain_linkage(
            &resolver,
            url.clone(),
            "did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp",
        )
        .await;

        assert_eq!(
            result,
            ValidationResult {
                status: ValidationStatus::Failure,
                url,
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: Some("invalid semantic structure of the domain linkage configuration".to_string()),
            }
        );
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn verifier_successfully_verifies_es256_signed_data() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

        let password = "sup3rSecr3t".to_string();

        let stronghold_manager = Arc::new(StrongholdManager::create(&password).unwrap());

        let subject = subject(stronghold_manager.clone(), password).await;

        let identifier = subject.identifier("did:key", Algorithm::ES256).await.unwrap();
        let fragment = identifier.split(':').next_back().unwrap();
        let public_key = subject.public_key(&format!("{identifier}#{fragment}")).await.unwrap();

        // x and y are each 32 bytes, they represent the public key
        let x = URL_SAFE_NO_PAD.encode(&public_key[1..33]);
        let y = URL_SAFE_NO_PAD.encode(&public_key[33..65]);

        let jwk = IotaIdentityJwk::from_json_value(serde_json::json!({
            "kty": "EC",
            "crv": "P-256",
            "x": x,
            "y": y,
        }))
        .unwrap();

        let message = URL_SAFE_NO_PAD.encode("foobar");

        let signature = subject.sign(&message, "did:key", Algorithm::ES256).await.unwrap();

        let input = VerificationInput {
            signing_input: message.to_string().as_bytes().into(),
            decoded_signature: signature.into(),
            alg: JwsAlgorithm::ES256,
        };

        assert!(Verifier.verify(input, &jwk).is_ok());
    }

    #[tokio::test]
    #[serial_test::serial]
    async fn verifier_successfully_verifies_eddsa_signed_data() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

        let password = "sup3rSecr3t".to_string();

        let stronghold_manager = Arc::new(StrongholdManager::create(&password).unwrap());

        let subject = subject(stronghold_manager.clone(), password).await;

        let identifier = subject.identifier("did:key", Algorithm::EdDSA).await.unwrap();
        let fragment = identifier.split(':').next_back().unwrap();
        let public_key = subject.public_key(&format!("{identifier}#{fragment}")).await.unwrap();

        // x represents the public key
        let x = URL_SAFE_NO_PAD.encode(public_key);

        let jwk = IotaIdentityJwk::from_json_value(serde_json::json!({
            "kty": "OKP",
            "crv": "Ed25519",
            "x": x,
        }))
        .unwrap();

        let message = URL_SAFE_NO_PAD.encode("foobar");

        let signature = subject.sign(&message, "did:key", Algorithm::EdDSA).await.unwrap();

        let input = VerificationInput {
            signing_input: message.to_string().as_bytes().into(),
            decoded_signature: signature.into(),
            alg: JwsAlgorithm::EdDSA,
        };

        assert!(Verifier.verify(input, &jwk).is_ok());
    }
}

#[cfg(test)]
mod connection_domain_tests {
    use super::*;

    use identity_credential::domain_linkage::DomainLinkageCredentialBuilder;
    use identity_iota::{
        core::{Duration, Object, Timestamp},
        did::CoreDID,
        document::{Service, ServiceEndpoint},
        verification::VerificationMethod,
    };
    use jsonwebtoken::{EncodingKey, Header};
    use p256::pkcs8::{EncodePrivateKey as _, LineEnding};
    use serde_json::json;
    use std::{collections::HashMap, sync::Mutex};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    /// A DID document with one ES256 key, which signs the Domain Linkage Credentials of the tests.
    struct TestIssuer {
        document: CoreDocument,
        key: EncodingKey,
        kid: String,
    }

    impl TestIssuer {
        fn new(linked_domain: Option<&Url>) -> Self {
            let secret_key = p256::SecretKey::from_slice(&[7; 32]).unwrap();
            let did: CoreDID = "did:web:service.example".parse().unwrap();
            let jwk = IotaIdentityJwk::from_json(&secret_key.public_key().to_jwk_string()).unwrap();
            let method = VerificationMethod::new_from_jwk(did.clone(), jwk, Some("key-1")).unwrap();

            let mut builder = CoreDocument::builder(Object::new())
                .id(did.clone())
                .verification_method(method);
            if let Some(linked_domain) = linked_domain {
                let origin = identity_iota::core::Url::parse(linked_domain.origin().ascii_serialization()).unwrap();
                builder = builder.service(
                    Service::builder(Object::new())
                        .id(format!("{did}#linked-domains").parse().unwrap())
                        .type_("LinkedDomains")
                        .service_endpoint(ServiceEndpoint::from(origin))
                        .build()
                        .unwrap(),
                );
            }

            let pem = secret_key.to_pkcs8_pem(LineEnding::LF).unwrap();
            TestIssuer {
                document: builder.build().unwrap(),
                key: EncodingKey::from_ec_pem(pem.as_bytes()).unwrap(),
                kid: format!("{did}#key-1"),
            }
        }

        /// Serves, at `server`, a DID Configuration that links the DID to `origin`.
        async fn serve_configuration(&self, server: &MockServer, origin: &Url) {
            let payload = DomainLinkageCredentialBuilder::new()
                .issuer(self.document.id().clone())
                .origin(identity_iota::core::Url::parse(origin.origin().ascii_serialization()).unwrap())
                .issuance_date(Timestamp::now_utc())
                .expiration_date(Timestamp::now_utc().checked_add(Duration::hours(1)).unwrap())
                .build()
                .and_then(|credential| credential.serialize_jwt(Default::default()))
                .unwrap();
            let mut header = Header::new(Algorithm::ES256);
            header.kid = Some(self.kid.clone());
            let jwt = jsonwebtoken::encode(
                &header,
                &serde_json::from_str::<serde_json::Value>(&payload).unwrap(),
                &self.key,
            )
            .unwrap();

            Mock::given(method("GET"))
                .and(path("/.well-known/did-configuration.json"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                    "@context": "https://identity.foundation/.well-known/did-configuration/v1",
                    "linked_dids": [jwt],
                })))
                .mount(server)
                .await;
        }
    }

    /// Answers `CNAME` lookups from a fixed table and records them.
    #[derive(Default)]
    struct FakeCnames {
        records: HashMap<&'static str, Result<&'static str, &'static str>>,
        lookups: Mutex<Vec<String>>,
    }

    impl FakeCnames {
        fn with(records: &[(&'static str, Result<&'static str, &'static str>)]) -> Self {
            FakeCnames {
                records: records.iter().copied().collect(),
                lookups: Mutex::default(),
            }
        }

        fn lookups(&self) -> Vec<String> {
            self.lookups.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl CnameResolver for FakeCnames {
        async fn cname(&self, name: &str) -> Result<Option<String>, String> {
            self.lookups.lock().unwrap().push(name.to_string());
            match self.records.get(name) {
                Some(Ok(target)) => Ok(Some(target.to_string())),
                Some(Err(err)) => Err(err.to_string()),
                None => Ok(None),
            }
        }
    }

    /// The service the wallet connects to, at `http://127.0.0.1:<port>`, and the domain its DID is linked to, at
    /// `http://localhost:<port>`. Their hosts differ, so only a `CNAME` can connect them.
    async fn service_and_linked_domain() -> (MockServer, Url, MockServer, Url) {
        let service = MockServer::start().await;
        let service_url: Url = service.uri().parse().unwrap();
        let linked = MockServer::start().await;
        let linked_url: Url = format!("http://localhost:{}", linked.address().port()).parse().unwrap();
        (service, service_url, linked, linked_url)
    }

    #[tokio::test]
    async fn the_connection_origin_itself_is_verified_without_dns() {
        let service = MockServer::start().await;
        let service_url: Url = format!("http://localhost:{}", service.address().port())
            .parse()
            .unwrap();
        let issuer = TestIssuer::new(None);
        issuer.serve_configuration(&service, &service_url).await;
        let cnames = FakeCnames::default();

        let result = validate_connection_domain_with(&issuer.document, service_url.clone(), &cnames).await;

        assert_eq!(
            result,
            ValidationResult::new(ValidationStatus::Success, service_url, None)
        );
        assert!(cnames.lookups().is_empty());
    }

    #[tokio::test]
    async fn a_linked_domain_that_is_a_cname_of_the_connection_host_is_verified() {
        let (service, service_url, linked, linked_url) = service_and_linked_domain().await;
        let issuer = TestIssuer::new(Some(&linked_url));
        // The service serves the same configuration as its linked domain, as UniCore does, which does not link the
        // service's own origin.
        issuer.serve_configuration(&service, &linked_url).await;
        issuer.serve_configuration(&linked, &linked_url).await;
        let cnames = FakeCnames::with(&[("localhost", Ok("127.0.0.1."))]);

        let result = validate_connection_domain_with(&issuer.document, service_url, &cnames).await;

        assert_eq!(result.status, ValidationStatus::Success, "{result:?}");
        assert_eq!(result.url, linked_url);
        assert_eq!(cnames.lookups(), ["localhost"]);
    }

    #[tokio::test]
    async fn a_linked_domain_that_is_not_a_cname_of_the_connection_host_fails() {
        let (service, service_url, linked, linked_url) = service_and_linked_domain().await;
        let issuer = TestIssuer::new(Some(&linked_url));
        issuer.serve_configuration(&service, &linked_url).await;
        issuer.serve_configuration(&linked, &linked_url).await;
        let cnames = FakeCnames::with(&[("localhost", Ok("load-balancer.example."))]);

        let result = validate_connection_domain_with(&issuer.document, service_url.clone(), &cnames).await;

        assert_eq!(result.status, ValidationStatus::Failure);
        assert_eq!(result.url, service_url);
        assert!(
            result.message.as_deref().is_some_and(
                |message| message.contains("localhost is linked to the DID but is not a CNAME of 127.0.0.1")
            ),
            "{result:?}"
        );
        assert_eq!(cnames.lookups(), ["localhost", "load-balancer.example"]);
    }

    #[tokio::test]
    async fn a_linked_domain_without_a_matching_credential_fails_without_dns() {
        let (_service, service_url, linked, linked_url) = service_and_linked_domain().await;
        let issuer = TestIssuer::new(Some(&linked_url));
        // The linked domain links the DID to another origin than its own.
        issuer
            .serve_configuration(&linked, &"https://other.example".parse().unwrap())
            .await;
        let cnames = FakeCnames::with(&[("localhost", Ok("127.0.0.1."))]);

        let result = validate_connection_domain_with(&issuer.document, service_url.clone(), &cnames).await;

        assert_eq!(result.status, ValidationStatus::Failure, "{result:?}");
        assert_eq!(result.url, service_url);
        assert!(cnames.lookups().is_empty());
    }

    #[tokio::test]
    async fn a_failed_cname_lookup_is_unknown() {
        let (_service, service_url, linked, linked_url) = service_and_linked_domain().await;
        let issuer = TestIssuer::new(Some(&linked_url));
        issuer.serve_configuration(&linked, &linked_url).await;
        let cnames = FakeCnames::with(&[("localhost", Err("timed out"))]);

        let result = validate_connection_domain_with(&issuer.document, service_url, &cnames).await;

        assert_eq!(result.status, ValidationStatus::Unknown, "{result:?}");
        assert!(result
            .message
            .as_deref()
            .is_some_and(|message| message.contains("CNAME lookup for localhost failed: timed out")));
    }

    #[tokio::test]
    async fn no_configuration_and_no_linked_domain_is_unknown() {
        let service = MockServer::start().await;
        let service_url: Url = service.uri().parse().unwrap();
        let issuer = TestIssuer::new(None);

        let result = validate_connection_domain_with(&issuer.document, service_url, &FakeCnames::default()).await;

        assert_eq!(result.status, ValidationStatus::Unknown, "{result:?}");
    }

    fn host(name: &str) -> Host {
        Host::parse(name).unwrap()
    }

    #[tokio::test]
    async fn aliases_follows_a_chain_of_cnames_case_insensitively() {
        let cnames = FakeCnames::with(&[
            ("identity.customer.example", Ok("customer-cdn.example.")),
            ("customer-cdn.example", Ok("Polaris.Dev2.Impierce.com.")),
        ]);

        let result = aliases(
            &cnames,
            &host("identity.customer.example"),
            &host("polaris.dev2.impierce.com"),
        )
        .await;

        assert_eq!(result, Ok(true));
    }

    #[tokio::test]
    async fn aliases_matches_the_same_host_without_dns() {
        let cnames = FakeCnames::default();

        let result = aliases(
            &cnames,
            &host("polaris.dev2.impierce.com"),
            &host("polaris.dev2.impierce.com"),
        )
        .await;

        assert_eq!(result, Ok(true));
        assert!(cnames.lookups().is_empty());
    }

    #[tokio::test]
    async fn aliases_gives_up_on_a_cname_loop() {
        let cnames = FakeCnames::with(&[("a.example", Ok("b.example.")), ("b.example", Ok("a.example."))]);

        let result = aliases(&cnames, &host("a.example"), &host("service.example")).await;

        assert_eq!(result, Ok(false));
        assert_eq!(cnames.lookups().len(), MAX_CNAME_HOPS + 1);
    }

    #[tokio::test]
    async fn aliases_does_not_look_up_an_ip_address() {
        let cnames = FakeCnames::default();

        let result = aliases(&cnames, &host("192.0.2.1"), &host("service.example")).await;

        assert_eq!(result, Ok(false));
        assert!(cnames.lookups().is_empty());
    }
}
