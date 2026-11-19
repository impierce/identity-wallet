use crate::did_resolver::Resolver;
use futures::future::join_all;
use hickory_resolver::{
    config::{ResolverConfig, CLOUDFLARE},
    net::runtime::TokioRuntimeProvider,
    proto::rr::{RData, RecordType},
    TokioResolver,
};
use identity_kit::{
    did::{Did, DidDocument},
    domain_linkage::{fetch_configuration, DomainLinkageValidator},
    jose::DefaultVerifier,
    resolver::{DocumentKeyResolver, KeyResolver},
    vc::validation::CredentialValidationOptions,
};
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{future::Future, time::Duration};
use ts_rs::TS;
use url::{Host, Url};

use crate::{
    http_client::IdentityKitHttpClient, state::did::validate_linked_verifiable_presentations::get_linked_domains,
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
/// must hold a Domain Linkage Credential of `did` for exactly that origin (DIF Well Known DID Configuration), validated
/// by identity-kit with the key `resolver` resolves for `did`.
pub async fn validate_domain_linkage(resolver: &Resolver, url: Url, did: &str) -> ValidationResult {
    match Did::parse(did) {
        Ok(did) => validate_linkage_at(resolver.inner(), &did, url).await,
        Err(err) => ValidationResult::new(ValidationStatus::Unknown, url, Some(format!("Invalid DID: {err}"))),
    }
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
    document: &DidDocument,
    url: Url,
    cnames: &impl CnameResolver,
) -> ValidationResult {
    // The document is resolved already: its keys are taken from it rather than resolved again for every origin.
    let keys = DocumentKeyResolver::new(document.clone());
    let did = &document.id;
    let direct = validate_linkage_at(&keys, did, url.clone()).await;
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

    let keys = &keys;
    let linked_results = join_all(linked_domains.into_iter().map(|linked_domain| {
        let connection_host = connection_host.to_owned();
        async move {
            let result = validate_linkage_at(keys, did, linked_domain.clone()).await;
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

/// Validates the domain linkage between `did` and the origin of `url`, with the keys of `did` from `keys`.
///
/// A configuration that cannot be fetched, or is not published, leaves the linkage unknown; one that does not link
/// `did` to the origin fails it.
async fn validate_linkage_at(keys: &impl KeyResolver, did: &Did, url: Url) -> ValidationResult {
    debug!(
        "Fetching the DID configuration of {}",
        url.origin().ascii_serialization()
    );
    let configuration = match fetch_configuration(&IdentityKitHttpClient, &url).await {
        Ok(configuration) => configuration,
        Err(err) => {
            return ValidationResult::new(
                ValidationStatus::Unknown,
                url,
                Some(format!("Error while fetching configuration: {err}")),
            )
        }
    };

    let validator = DomainLinkageValidator::new(keys, DefaultVerifier);
    match validator
        .validate_linkage(&configuration, did, &url, &CredentialValidationOptions::new())
        .await
    {
        Ok(_) => ValidationResult::new(ValidationStatus::Success, url, None),
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

trait CnameResolver: Sync {
    /// The target of the `CNAME` record of `name`, or `None` when `name` has none.
    fn cname(&self, name: &str) -> impl Future<Output = Result<Option<String>, String>> + Send;
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
        let public_resolvers = || {
            TokioResolver::builder_with_config(
                ResolverConfig::udp_and_tcp(&CLOUDFLARE),
                TokioRuntimeProvider::default(),
            )
        };
        // iOS offers no API to read the system DNS configuration: hickory reads it through `SCDynamicStore`, which is
        // macOS-only and does not link on iOS. The `system-config` feature is therefore not enabled for iOS.
        #[cfg(not(target_os = "ios"))]
        let mut builder = TokioResolver::builder_tokio().unwrap_or_else(|err| {
            warn!("Failed to read the system DNS configuration, using public resolvers instead: {err}");
            public_resolvers()
        });
        #[cfg(target_os = "ios")]
        let mut builder = public_resolvers();
        builder.options_mut().timeout = DNS_TIMEOUT;
        builder.options_mut().attempts = 1;
        builder.build().map_err(|err| err.to_string())
    }
}

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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{persistence::STRONGHOLD, stronghold::StrongholdManager, subject::subject};

    use super::*;

    use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
    use jsonwebtoken::{crypto::verify, Algorithm, DecodingKey};
    use oid4vc_core::{Sign, Subject, Verify};

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

    /// identity-kit reads a Linked Data Proof entry as data and validates only JWT entries, so a configuration without a
    /// JWT for the DID does not link it. The IOTA crates failed to read such a configuration (`Unknown`).
    #[tokio::test]
    async fn when_well_known_only_contains_json_ld_credential_then_return_failure() {
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
                status: ValidationStatus::Failure,
                url,
                name: None,
                logo_uri: None,
                issuance_date: None,
                message: Some("the configuration holds no JWT credential for `did:foo:bar`".to_string()),
            }
        );
    }

    #[tokio::test]
    async fn when_well_known_contains_json_ld_and_jwt_then_the_jwt_is_read() {
        // Mocking a `DomainLinkageCredential` in a local test is difficult since the `mock_server` does not have an actual domain.
        // We thereby only test that `fetch_configuration()` reads the JWT next to the Linked Data Proof entry.

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

        let domain_linkage_configuration =
            fetch_configuration(&IdentityKitHttpClient, &mock_server.uri().parse().unwrap())
                .await
                .unwrap();

        assert_eq!(domain_linkage_configuration.jwts().count(), 1);
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
                message: Some(
                    "the configuration holds no JWT credential for `did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp`"
                        .to_string()
                ),
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
                message: Some(
                    "the configuration holds no JWT credential for `did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp`"
                        .to_string()
                ),
            }
        );
    }

    /// `Subject::public_key` returns the key in the form `jsonwebtoken` verifies with, as openid4vc uses it.
    #[tokio::test]
    #[serial_test::serial]
    async fn subject_public_keys_verify_its_signatures() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
        let stronghold_manager = Arc::new(StrongholdManager::create("sup3rSecr3t").unwrap());
        let subject = subject(stronghold_manager.clone()).await.unwrap();

        for algorithm in [Algorithm::EdDSA, Algorithm::ES256] {
            let identifier = subject.identifier("did:key", algorithm).await.unwrap();
            let fragment = identifier.split(':').next_back().unwrap();
            let public_key = subject.public_key(&format!("{identifier}#{fragment}")).await.unwrap();
            let decoding_key = match algorithm {
                Algorithm::EdDSA => DecodingKey::from_ed_der(&public_key),
                _ => DecodingKey::from_ec_der(&public_key),
            };

            let message = URL_SAFE_NO_PAD.encode("foobar");
            let signature = subject.sign(&message, "did:key", algorithm).await.unwrap();

            assert!(verify(
                &URL_SAFE_NO_PAD.encode(signature),
                message.as_bytes(),
                &decoding_key,
                algorithm
            )
            .unwrap());
        }
    }
}

#[cfg(test)]
mod connection_domain_tests {
    use super::*;

    use chrono::Utc;
    use identity_kit::domain_linkage::DomainLinkageCredentialBuilder;
    use jsonwebtoken::{Algorithm, EncodingKey, Header};
    use p256::pkcs8::{EncodePrivateKey as _, LineEnding};
    use serde_json::json;
    use std::{collections::HashMap, sync::Mutex};
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    /// A DID document with one ES256 key under `assertionMethod`, which signs the Domain Linkage Credentials of the
    /// tests.
    struct TestIssuer {
        document: DidDocument,
        key: EncodingKey,
        kid: String,
    }

    impl TestIssuer {
        fn new(linked_domain: Option<&Url>) -> Self {
            let secret_key = p256::SecretKey::from_slice(&[7; 32]).unwrap();
            let did = "did:web:service.example";
            let kid = format!("{did}#key-1");
            let public_jwk: serde_json::Value = serde_json::from_str(&secret_key.public_key().to_jwk_string()).unwrap();

            let mut document = json!({
                "@context": ["https://www.w3.org/ns/did/v1"],
                "id": did,
                "verificationMethod": [{
                    "id": kid,
                    "type": "JsonWebKey2020",
                    "controller": did,
                    "publicKeyJwk": public_jwk,
                }],
                "assertionMethod": [kid],
            });
            if let Some(linked_domain) = linked_domain {
                document["service"] = json!([{
                    "id": format!("{did}#linked-domains"),
                    "type": "LinkedDomains",
                    "serviceEndpoint": linked_domain.origin().ascii_serialization(),
                }]);
            }

            let pem = secret_key.to_pkcs8_pem(LineEnding::LF).unwrap();
            TestIssuer {
                document: serde_json::from_value(document).unwrap(),
                key: EncodingKey::from_ec_pem(pem.as_bytes()).unwrap(),
                kid,
            }
        }

        /// Serves, at `server`, a DID Configuration that links the DID to `origin`.
        async fn serve_configuration(&self, server: &MockServer, origin: &Url) {
            self.serve_configuration_with(server, origin, |_| {}).await;
        }

        /// Like `serve_configuration`, with `edit_header` applied to the credential's JWS header.
        async fn serve_configuration_with(
            &self,
            server: &MockServer,
            origin: &Url,
            edit_header: impl FnOnce(&mut Header),
        ) {
            let claims = DomainLinkageCredentialBuilder::new(
                self.document.id.clone(),
                origin,
                Utc::now(),
                Utc::now() + chrono::Duration::hours(1),
            )
            .unwrap()
            .jwt_claims();
            // A domain linkage credential has no `typ` (DIF Well Known DID Configuration).
            let mut header = Header::new(Algorithm::ES256);
            header.typ = None;
            header.kid = Some(self.kid.clone());
            edit_header(&mut header);
            let jwt = jsonwebtoken::encode(&header, &claims, &self.key).unwrap();

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

    /// identity-kit follows the DIF Well Known DID Configuration: a domain linkage credential has no `typ` header. The
    /// IOTA validator accepted one.
    #[tokio::test]
    async fn a_domain_linkage_credential_with_a_typ_fails() {
        let service = MockServer::start().await;
        let service_url: Url = format!("http://localhost:{}", service.address().port())
            .parse()
            .unwrap();
        let issuer = TestIssuer::new(None);
        issuer
            .serve_configuration_with(&service, &service_url, |header| header.typ = Some("JWT".to_string()))
            .await;

        let result = validate_connection_domain_with(&issuer.document, service_url, &FakeCnames::default()).await;

        assert_eq!(result.status, ValidationStatus::Failure, "{result:?}");
    }

    /// identity-kit resolves the key of a domain linkage credential under `assertionMethod`, as for any credential
    /// issuer. The IOTA validator accepted any verification method of the document.
    #[tokio::test]
    async fn a_domain_linkage_credential_signed_with_a_key_outside_assertion_method_fails() {
        let service = MockServer::start().await;
        let service_url: Url = format!("http://localhost:{}", service.address().port())
            .parse()
            .unwrap();
        let mut issuer = TestIssuer::new(None);
        let mut document = serde_json::to_value(&issuer.document).unwrap();
        document.as_object_mut().unwrap().remove("assertionMethod");
        issuer.document = serde_json::from_value(document).unwrap();
        issuer.serve_configuration(&service, &service_url).await;

        let result = validate_connection_domain_with(&issuer.document, service_url, &FakeCnames::default()).await;

        assert_eq!(result.status, ValidationStatus::Failure, "{result:?}");
    }

    /// identity-kit follows no redirect to a DID configuration: it must be served at the origin's well-known URL. The
    /// IOTA-based fetch followed redirects.
    #[tokio::test]
    async fn a_redirected_did_configuration_is_unknown() {
        let service = MockServer::start().await;
        let service_url: Url = format!("http://localhost:{}", service.address().port())
            .parse()
            .unwrap();
        let elsewhere = MockServer::start().await;
        let issuer = TestIssuer::new(None);
        issuer.serve_configuration(&elsewhere, &service_url).await;
        Mock::given(method("GET"))
            .and(path("/.well-known/did-configuration.json"))
            .respond_with(ResponseTemplate::new(302).insert_header(
                "location",
                format!("{}/.well-known/did-configuration.json", elsewhere.uri()).as_str(),
            ))
            .mount(&service)
            .await;

        let result = validate_connection_domain_with(&issuer.document, service_url, &FakeCnames::default()).await;

        assert_eq!(result.status, ValidationStatus::Unknown, "{result:?}");
        assert!(elsewhere.received_requests().await.unwrap().is_empty());
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
