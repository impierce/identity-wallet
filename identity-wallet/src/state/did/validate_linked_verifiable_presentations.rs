use crate::did_resolver::Resolver;
use crate::{
    http_client::{get_http_client, IdentityKitHttpClient},
    state::{
        core_utils::helpers::download_logo,
        credentials::{
            reducers::send_token_request::{get_credential_status, has_token_status_list_claim},
            CredentialStatus, CredentialStatusType, DisplayCredential, VerifiableCredentialRecord,
        },
        did::validate_domain_linkage::ValidationResult,
    },
    subject::Subject,
    well_known::{well_known_candidates, OPENID_CREDENTIAL_ISSUER},
};
use futures::{
    stream::{iter, FuturesUnordered},
    StreamExt,
};
use identity_kit::{
    did::DidDocument,
    jose::DefaultVerifier,
    linked_vp::{decode, LinkedVpClient, LinkedVpError, LinkedVpValidationOptions, LinkedVpValidator},
    resolver::{CompositeKeyResolver, DocumentKeyResolver},
    vc::{
        validation::{CredentialValidationOptions, DecodedCredential, FailFast, StatusPolicy},
        Jwt,
    },
};
use log::{debug, info, warn};
use oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;
use url::Url;

#[derive(Clone, Serialize, Deserialize, Debug, TS, Default, PartialEq)]
#[ts(export, export_to = "bindings/user_prompt/LinkedVerifiableCredentialData.ts")]
pub struct LinkedVerifiableCredentialData {
    pub credential: DisplayCredential,
    pub issuer_domain_validations: Vec<ValidationResult>,
}

/// Validate the linked verifiable presentations for the given holder DID. Returns a list of linked verifiable
/// credential data. It starts by resolving the holder DID and then iterates over the linked verifiable presentation
/// URLs. For each linked verifiable presentation, it validates the presentation and then validates the linked
/// verifiable credentials. It only considers linked verifiable credentials with successful domain linkage validation.
pub async fn validate_linked_verifiable_presentations(
    subject: &Subject,
    holder_did: &str,
) -> Vec<Vec<LinkedVerifiableCredentialData>> {
    info!("Validating linked verifiable presentations for holder DID: {holder_did}");

    let resolver = subject.resolver().await;

    let holder_document = match resolver.resolve(holder_did).await {
        Ok(holder_document) => holder_document,
        _ => {
            warn!("Failed to resolve holder DID: {holder_did}");
            return vec![];
        }
    };

    debug!("Holder document: {holder_document:#?}");

    iter(get_linked_verifiable_presentation_urls(&holder_document))
        .filter_map(|linked_verifiable_presentation_url| {
            debug!("Processing linked verifiable presentation URL: {linked_verifiable_presentation_url}");
            // Validate the linked verifiable presentation and get the linked verifiable credential data
            get_validated_linked_presentation_data(subject, &holder_document, linked_verifiable_presentation_url)
        })
        .collect::<Vec<_>>()
        .await
}

/// The URLs of the `LinkedVerifiablePresentation` services of `document`. An entry that is not a URL is skipped, and
/// does not hide the other URLs of its service.
/// https://identity.foundation/linked-vp/#linked-verifiable-presentation
fn get_linked_verifiable_presentation_urls(document: &DidDocument) -> Vec<Url> {
    identity_kit::linked_vp::services(document)
        .filter_map(|service| {
            // The rejected endpoint URIs stay out of the log (ADR 0003): only the service id and why.
            service
                .inspect_err(|err| match err {
                    LinkedVpError::InvalidService { id, source, .. } => match source {
                        Some(source) => {
                            warn!("Skipping the invalid LinkedVerifiablePresentation service `{id}`: {source}")
                        }
                        None => warn!("Skipping the invalid LinkedVerifiablePresentation service `{id}`"),
                    },
                    _ => warn!("Skipping an invalid LinkedVerifiablePresentation service"),
                })
                .ok()
        })
        .flat_map(|service| {
            for (_, err) in &service.invalid_urls {
                warn!(
                    "Skipping an endpoint of LinkedVerifiablePresentation service `{}` that is not a URL: {err}",
                    service.id()
                );
            }
            service.urls
        })
        .collect()
}

/// Validate the linked verifiable presentations for the given holder document and linked verifiable presentation URL.
/// It returns a list of linked verifiable credential data.
async fn get_validated_linked_presentation_data(
    subject: &Subject,
    holder_document: &DidDocument,
    linked_verifiable_presentation_url: Url,
) -> Option<Vec<LinkedVerifiableCredentialData>> {
    let resolver = subject.resolver().await;
    let credentials =
        validate_linked_verifiable_presentation(&resolver, holder_document, linked_verifiable_presentation_url).await?;
    Some(get_validated_linked_credential_data(subject, credentials).await)
}

/// Fetches the linked verifiable presentation at `url` and validates it with identity-kit (DIF Linked Verifiable
/// Presentation): the holder and the signer are the DID of `holder_document`, signing with a key under
/// `authentication`, and every credential is signed by its issuer and has the holder as its subject.
///
/// Returns the credentials that validated, each with its JWT. A presentation that holds is kept even when some of its
/// credentials do not: those are left out, the others shown.
async fn validate_linked_verifiable_presentation(
    resolver: &Resolver,
    holder_document: &DidDocument,
    url: Url,
) -> Option<Vec<(Jwt, DecodedCredential)>> {
    // The holder's keys come from its document, which is resolved already; the issuers' keys are resolved.
    let keys = CompositeKeyResolver::new()
        .with(DocumentKeyResolver::new(holder_document.clone()))
        .with_shared(resolver.shared());
    let client = LinkedVpClient::new(IdentityKitHttpClient, LinkedVpValidator::new(keys, DefaultVerifier));

    let jwt = client
        .fetch(&url)
        .await
        .inspect_err(|err| warn!("Failed to retrieve linked verifiable presentation: {err}"))
        .ok()?;

    // UniMe checks the status of each credential itself below (`get_credential_status`), so identity-kit ignores the
    // status entries it has no check for, as the IOTA validator did with `StatusCheck::SkipUnsupported`.
    let options = LinkedVpValidationOptions::new()
        .with_credentials(CredentialValidationOptions::new().with_status(StatusPolicy::SkipUnsupported))
        .with_fail_fast(FailFast::AllErrors);
    let failed = match client.validator().validate(&jwt, &holder_document.id, &options).await {
        Ok(presentation) => {
            return Some(
                presentation
                    .presentation
                    .verifiable_credential
                    .into_iter()
                    .zip(presentation.credentials)
                    .collect(),
            )
        }
        // The presentation holds; some of its credentials do not.
        Err(LinkedVpError::Presentation(error)) if error.presentation.is_none() => error.credentials,
        Err(err) => {
            warn!("Failed to validate linked verifiable presentation: {err}");
            return None;
        }
    };

    for (index, err) in &failed {
        warn!("Skipping linked verifiable credential {index}: {err}");
    }
    let decoded = decode(&jwt).ok()?;
    // `credentials` and the indices in `failed` count every entry of `verifiableCredential`, while the presentation's
    // `verifiable_credential` keeps only the entries that are JWTs. The JWTs are therefore taken from the entries
    // themselves, so that each credential keeps its own.
    let entries = match decoded.claims.additional.get("vp")?.get("verifiableCredential")? {
        Value::Array(entries) => entries.clone(),
        entry => vec![entry.clone()],
    };
    Some(
        entries
            .into_iter()
            .zip(decoded.credentials)
            .enumerate()
            .filter(|(index, _)| !failed.contains_key(index))
            .filter_map(|(_, (entry, credential))| Some((Jwt::new(entry.as_str()?).ok()?, credential.ok()?)))
            .collect(),
    )
}

/// Builds the display data of the linked verifiable credentials that identity-kit validated. Skips credentials whose
/// status is not valid (including ones whose OAuth Token Status List status cannot be fetched) or whose issuer has no
/// valid domain linkage.
/// Since anyone can host a linked verifiable presentation, the issuer of each credential is checked: its DID document's
/// linked domains must link back to it.
async fn get_validated_linked_credential_data(
    subject: &Subject,
    linked_verifiable_credentials: Vec<(Jwt, DecodedCredential)>,
) -> Vec<LinkedVerifiableCredentialData> {
    let resolver = &subject.resolver().await;
    iter(linked_verifiable_credentials)
        .filter_map(|(linked_verifiable_credential_jwt, linked_verifiable_credential)| async move {
            // Resolve the issuer document and issuer DID
            let issuer_did = linked_verifiable_credential.credential.issuer.id().to_string();
            let issuer_document = resolver
                .resolve(&issuer_did)
                .await
                .inspect_err(|err| warn!("Failed to resolve issuer DID: {err:#}"))
                .ok()?;

            debug!("Issuer document: {issuer_document:#?}");

            // Resolve the issuer linked domains from the issuer document
            let issuer_linked_domains = get_linked_domains(&issuer_document);

            debug!("Issuer linked domains: {issuer_linked_domains:#?}");

            // Only linked verifiable credentials with at least one successful domain linkage validation are considered
            let validated_linked_domains = get_validated_linked_domains(resolver, &issuer_linked_domains, &issuer_did).await;

            if validated_linked_domains.is_empty() {
                warn!("No validated linked domains for issuer DID: {issuer_did}");
                return None;
            }

            // TODO: Uncomment this once json schema validation works on mobile.
            // Validate the linked verifiable credential against its corresponding JSON Schema
            // validate_credential_types(&linked_verifiable_credential.credential.to_json_value().ok()?).ok()?;

            let properties = &linked_verifiable_credential.credential.properties;
            let credential_name = properties.get("name").and_then(Value::as_str).map(ToString::to_string);
            let credential_logo_uri = match properties.get("logo_uri").and_then(Value::as_str) {
                Some(logo_uri) => download_logo(logo_uri).await,
                None => None,
            };

            let linked_domains = validated_linked_domains.iter().map(|result| result.url.clone()).collect::<Vec<url::Url>>();
            let (issuer_name, issuer_logo_uri) = get_issuer_info(&linked_domains).await;
            let issuance_date = linked_verifiable_credential.credential.issuance_date.to_rfc3339();

            debug!("LinkedVerifiableCredentialData: name: {credential_name:?}, credential_logo_uri: {credential_logo_uri:?}, issuer_name: {issuer_name:?}, issuer_logo_uri: {issuer_logo_uri:?}, issuance_date: {issuance_date}, validated_linked_domains: {linked_domains:#?}");

            let Ok(mut verifiable_credential_record) = VerifiableCredentialRecord::try_new(
                CredentialFormats::JwtVcJson(()),
                serde_json::json!(linked_verifiable_credential_jwt.as_str()),
                vec![],
            ) else {
                warn!("Failed to create `verifiable_credential_record` for linked verifiable credential");
                return None;
            };

            let credential_status = get_credential_status(&verifiable_credential_record, subject).await;

            // Expired credentials are already rejected by identity-kit. A credential with an OAuth Token Status List
            // status is only shown when that status is fetched and valid. UniMe cannot check other status mechanisms,
            // so such a credential is shown like one without a status.
            match &credential_status {
                Some(CredentialStatus { status: CredentialStatusType::Valid, .. }) => {}
                None if !has_token_status_list_claim(&verifiable_credential_record) => {}
                _ => {
                    warn!("Skipping linked verifiable credential that is not valid: {credential_status:?}");
                    return None;
                }
            }

            verifiable_credential_record.display_credential.credential_status = credential_status;
            verifiable_credential_record.display_credential.display_name = credential_name.unwrap_or_default();
            verifiable_credential_record.display_credential.metadata.icon = credential_logo_uri;
            verifiable_credential_record.display_credential.issuer_name = issuer_name.unwrap_or_default();
            verifiable_credential_record.display_credential.issuer_logo_uri = issuer_logo_uri;

            Some(LinkedVerifiableCredentialData {
                credential: verifiable_credential_record.display_credential,
                issuer_domain_validations: validated_linked_domains,
            })
        })
        .collect::<Vec<_>>()
        .await
}

/// Returns a Vec of successfully validated issuer linked domains.
async fn get_validated_linked_domains(
    // TODO: make this conditional configuration more 'ergonomic'.
    #[cfg(not(feature = "test_utils"))] resolver: &Resolver,
    #[cfg(feature = "test_utils")] _resolver: &Resolver,
    issuer_linked_domains: &[Url],
    issuer_did: &str,
) -> Vec<ValidationResult> {
    FuturesUnordered::from_iter(issuer_linked_domains.iter().map(|issuer_linked_domain| async move {
        let validation_result: ValidationResult = {
            #[cfg(not(feature = "test_utils"))]
            {
                use crate::state::did::validate_domain_linkage::validate_domain_linkage;

                validate_domain_linkage(resolver, issuer_linked_domain.clone(), issuer_did).await
            }
            #[cfg(feature = "test_utils")]
            {
                use crate::state::did::validate_domain_linkage::ValidationStatus;

                // Silence unused variable warning
                let _issuer_did = issuer_did;
                // Skip validation during tests
                ValidationResult {
                    status: ValidationStatus::Success,
                    url: issuer_linked_domain.clone(),
                    name: None,
                    logo_uri: None,
                    issuance_date: None,
                    message: None,
                }
            }
        };

        info!("Validation of domain linkage for issuer linked domain '{issuer_linked_domain}' resulted in: {validation_result:?}");
        (validation_result.status == crate::state::did::validate_domain_linkage::ValidationStatus::Success)
            .then_some(validation_result)
    }))
    .filter_map(|result| async move { result })
    .collect()
    .await
}

/// The origins of the `LinkedDomains` services of `document`, read by identity-kit: an object with `origins`, a single
/// URL or a set of URLs.
/// https://identity.foundation/.well-known/resources/did-configuration/#linked-domain-service-endpoint
pub(crate) fn get_linked_domains(document: &DidDocument) -> Vec<Url> {
    identity_kit::domain_linkage::services(document)
        .filter_map(|service| {
            service
                .inspect_err(|err| warn!("Skipping an invalid LinkedDomains service: {err}"))
                .ok()
        })
        .flat_map(|service| service.origins)
        .collect()
}

/// Retrieve the issuer's name and issuer logo URI from .well-known/openid-credential-issuer metadata.
async fn get_issuer_info(validated_linked_domains: &[Url]) -> (Option<String>, Option<String>) {
    let mut issuer_name = None;
    let mut issuer_logo_uri = None;

    for domain in validated_linked_domains.iter() {
        let well_known_endpoints = well_known_candidates(domain, OPENID_CREDENTIAL_ISSUER);

        for well_known_endpoint in well_known_endpoints {
            debug!("Trying to fetch issuer info from {well_known_endpoint} endpoint");
            if let Ok(response) = get_http_client().await.get(well_known_endpoint.clone()).send().await {
                debug!("Response from {well_known_endpoint}: {response:#?}");
                if let Ok(metadata) = response.json::<CredentialIssuerMetadata>().await {
                    debug!("Metadata from {well_known_endpoint}: {metadata:#?}");
                    if let Some(display) = metadata.display.as_deref().and_then(|d| d.first()) {
                        if issuer_name.is_none() {
                            issuer_name = display.get("name").and_then(Value::as_str).map(ToString::to_string);
                        }
                        if issuer_logo_uri.is_none() {
                            if let Some(logo_uri_str) = extract_logo_uri_from_display(std::slice::from_ref(display)) {
                                issuer_logo_uri = download_logo(&logo_uri_str).await;
                            }
                        }
                    }
                    if issuer_name.is_some() && issuer_logo_uri.is_some() {
                        break;
                    }
                }
            }
        }
        if issuer_name.is_some() && issuer_logo_uri.is_some() {
            break;
        }
    }

    if issuer_name.is_none() {
        if let Some(domain) = validated_linked_domains.first() {
            if let Some(host) = domain.host_str() {
                issuer_name = Some(host.to_string());
            }
        }
    }

    (issuer_name, issuer_logo_uri)
}

fn extract_logo_uri_from_display(display: &[Value]) -> Option<String> {
    display
        .first()
        .and_then(|display| display.get("logo"))
        .and_then(|logo| {
            if let Some(uri) = logo.get("uri").or_else(|| logo.get("url")).and_then(|url| url.as_str()) {
                Some(uri.to_string())
            } else {
                logo.as_str().map(ToString::to_string)
            }
        })
}

#[cfg(not(feature = "test_utils"))]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::stronghold::{StrongholdManager, ES256_KEY_ID};
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use chrono::Utc;
    use identity_kit::{
        did::Did,
        domain_linkage::{DomainLinkageConfiguration, DomainLinkageCredentialBuilder},
        jose::JwsAlgorithm,
        keys::{KeyId, KeyStore as _},
        linked_vp::LinkedPresentationBuilder,
    };
    use iota_stronghold::engine;
    use jsonwebtoken::{Algorithm, Header};
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::TempDir;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    // 'Entity' struct that represents a digital identity including a DID Document, a domain, and its keys.
    struct TestEntity {
        pub mock_server: MockServer,
        pub domain: url::Url,
        /// The DID document, as served at `.well-known/did.json`.
        pub document: Value,
        pub stronghold_manager: Arc<StrongholdManager>,
        pub subject: Arc<Subject>,
    }

    impl TestEntity {
        // Create a new 'Entity' with a DID Document, mock server, a domain, and its keys.
        async fn new() -> Self {
            engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

            let mock_server = MockServer::start().await;

            let uri = mock_server.uri();
            let port = uri.split(':').next_back().unwrap();
            let domain: url::Url = format!("http://localhost:{port}").parse().unwrap();

            let temp_dir = TempDir::new().unwrap();
            let path = temp_dir.path().join("stronghold.stronghold");
            *crate::persistence::STRONGHOLD.lock().unwrap() = path.clone();
            let stronghold_manager = Arc::new(crate::stronghold::StrongholdManager::create("sup3rSecr3t").unwrap());

            // A `did:web` for the mock server, with the wallet's ES256 key as its only verification method: under
            // `authentication` as a holder, under `assertionMethod` as an issuer.
            let public_key_jwk = stronghold_manager.public_jwk(&KeyId::new(ES256_KEY_ID)).await.unwrap();
            let did = format!("did:web:localhost%3A{port}");
            let kid = format!("{did}#key-0");
            let document = json!({
                "@context": ["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/jws-2020/v1"],
                "id": did,
                "verificationMethod": [{
                    "id": kid,
                    "type": "JsonWebKey2020",
                    "controller": did,
                    "publicKeyJwk": public_key_jwk,
                }],
                "authentication": [kid],
                "assertionMethod": [kid],
                "service": [],
            });

            let subject = crate::subject::subject(stronghold_manager.clone()).await.unwrap();

            TestEntity {
                mock_server,
                domain,
                document,
                stronghold_manager,
                subject,
            }
        }

        fn did(&self) -> String {
            self.document["id"].as_str().unwrap().to_string()
        }

        fn did_document(&self) -> DidDocument {
            serde_json::from_value(self.document.clone()).unwrap()
        }

        fn add_service(&mut self, service_id: &str, type_: &str, service_endpoint: Value) {
            let id = format!("{}#{service_id}", self.did());
            self.document["service"]
                .as_array_mut()
                .unwrap()
                .push(json!({ "id": id, "type": type_, "serviceEndpoint": service_endpoint }));
        }

        // Add the `.well-known/did.json` endpoint to the mock server.
        async fn add_well_known_did_json(&self) {
            Mock::given(method("GET"))
                .and(path(".well-known/did.json"))
                .respond_with(ResponseTemplate::new(200).set_body_json(&self.document))
                .mount(&self.mock_server)
                .await;
        }

        // Add the `.well-known/did-configuration.json` endpoint to the mock server.
        async fn add_well_known_did_configuration_json(&mut self, service_id: &str, origins: &[url::Url]) {
            let origins: Vec<String> = origins
                .iter()
                .map(|origin| origin.origin().ascii_serialization())
                .collect();
            self.add_service(service_id, "LinkedDomains", json!({ "origins": origins }));

            let claims = DomainLinkageCredentialBuilder::new(
                Did::parse(&self.did()).unwrap(),
                &self.domain,
                Utc::now(),
                Utc::now() + chrono::Duration::seconds(60),
            )
            .unwrap()
            .jwt_claims();
            // A domain linkage credential has no `typ` (DIF Well Known DID Configuration).
            let jwt = self.generate_jwt(&serde_json::to_value(claims).unwrap(), None).await;
            let domain_linkage_configuration = DomainLinkageConfiguration::new().with_credential(jwt);

            Mock::given(method("GET"))
                .and(path(".well-known/did-configuration.json"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!(domain_linkage_configuration)))
                .mount(&self.mock_server)
                .await;
        }

        // Add a linked verifiable presentation to the DID Document and the mock server.
        async fn add_linked_verifiable_presentation(
            &mut self,
            service_id: &str,
            linked_verifiable_presentation_data: Vec<(String, Vec<Jwt>)>,
        ) {
            let mut urls: Vec<url::Url> = vec![];

            for (linked_verifiable_presentation_endpoint, linked_verifiable_credential_jwts) in
                linked_verifiable_presentation_data
            {
                let url = format!(
                    "{}/{linked_verifiable_presentation_endpoint}",
                    self.domain.origin().ascii_serialization()
                )
                .parse()
                .unwrap();
                urls.push(url);

                let mut builder = LinkedPresentationBuilder::new(Did::parse(&self.did()).unwrap());
                for linked_verifiable_credential_jwt in linked_verifiable_credential_jwts {
                    builder = builder.with_credential(linked_verifiable_credential_jwt);
                }
                let claims = serde_json::to_value(builder.jwt_claims().unwrap()).unwrap();
                let linked_verifiable_presentation = self.generate_jwt(&claims, Some("JWT")).await;

                Mock::given(method("GET"))
                    .and(path(format!("/{linked_verifiable_presentation_endpoint}")))
                    .respond_with(ResponseTemplate::new(200).set_body_string(linked_verifiable_presentation.as_str()))
                    .mount(&self.mock_server)
                    .await;
            }

            let service_endpoint = match urls.len() {
                // Value::String
                1 => json!(urls[0]),
                // Value::Array
                _ => json!(urls),
            };
            self.add_service(service_id, "LinkedVerifiablePresentation", service_endpoint);
        }

        // 'Issues' a Credential Jwt to a subject.
        async fn issue_credential(
            &mut self,
            subject_id: &str,
            credential_name: &str,
            credential_logo: url::Url,
        ) -> Jwt {
            self.issue_credential_with(subject_id, credential_name, credential_logo, |_| {})
                .await
        }

        /// Like `issue_credential`, with `edit_payload` applied to the JWT payload before it is signed.
        async fn issue_credential_with(
            &mut self,
            subject_id: &str,
            credential_name: &str,
            credential_logo: url::Url,
            edit_payload: impl FnOnce(&mut Value),
        ) -> Jwt {
            // A VCDM 1.1 credential in the JWT encoding: `iss`, `sub` and `nbf` stand for its issuer, subject and
            // issuance date.
            let mut payload = json!({
                "iss": self.did(),
                "sub": subject_id,
                "nbf": 1577836800,
                "vc": {
                    "@context": ["https://www.w3.org/2018/credentials/v1"],
                    "type": ["VerifiableCredential"],
                    "credentialSubject": {},
                    "name": credential_name,
                    "logo_uri": credential_logo,
                },
            });
            edit_payload(&mut payload);

            self.generate_jwt(&payload, Some("JWT")).await
        }

        // Signs `payload` with the entity's key, naming it in `kid`.
        async fn generate_jwt(&self, payload: &Value, typ: Option<&str>) -> Jwt {
            let header = Header {
                alg: Algorithm::ES256,
                typ: typ.map(str::to_string),
                kid: Some(format!("{}#key-0", self.did())),
                ..Default::default()
            };

            let message = [
                URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).unwrap().as_slice()),
                URL_SAFE_NO_PAD.encode(serde_json::to_vec(payload).unwrap()),
            ]
            .join(".");

            let proof_value = self
                .stronghold_manager
                .sign(&KeyId::new(ES256_KEY_ID), JwsAlgorithm::ES256, message.as_bytes())
                .await
                .unwrap();
            let signature = URL_SAFE_NO_PAD.encode(proof_value.as_slice());

            Jwt::new([message, signature].join(".")).unwrap()
        }

        async fn add_logo_endpoint(&self) {
            Mock::given(method("GET"))
                .and(path("logo.png"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_raw(include_bytes!("../../../resources/images/impierce.png"), "image/png"),
                )
                .mount(&self.mock_server)
                .await;
        }
    }

    #[tokio::test]
    async fn validate_linked_verifiable_presentations_successfully_validates_multiple_presentations() {
        let mut holder = TestEntity::new().await;

        let mut issuer_a = TestEntity::new().await;
        issuer_a.add_logo_endpoint().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer A mock server.
        issuer_a
            .add_well_known_did_configuration_json("linked-domain", &[issuer_a.domain.clone().into()])
            .await;
        issuer_a.add_well_known_did_json().await;

        let mut issuer_b = TestEntity::new().await;
        issuer_b.add_logo_endpoint().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer B mock server.
        issuer_b
            .add_well_known_did_configuration_json("linked-domain", &[issuer_b.domain.clone().into()])
            .await;
        issuer_b.add_well_known_did_json().await;

        let logo_uri_a: String = format!("{}logo.png", issuer_a.domain.clone());

        let verifiable_credential_jwt = issuer_a
            .issue_credential(holder.did().as_str(), "Webshop", logo_uri_a.parse().unwrap())
            .await;

        let service_id = "linked-verifiable-presentation";
        let linked_verifiable_presentation_endpoint = "linked-verifiable-presentation.jwt";

        // Add the first linked verifiable presentation endpoint and the service to the holder DID Document.
        holder
            .add_linked_verifiable_presentation(
                service_id,
                vec![(
                    linked_verifiable_presentation_endpoint.to_string(),
                    vec![verifiable_credential_jwt],
                )],
            )
            .await;

        let logo_uri_b: String = format!("{}logo.png", issuer_b.domain.clone());

        let verifiable_credential_jwt_2 = issuer_b
            .issue_credential(holder.did().as_str(), "Webshop", logo_uri_b.parse().unwrap())
            .await;

        let service_id2 = "linked-verifiable-presentation-2";

        // Add the second linked verifiable presentation endpoint and the service to the holder DID Document.
        let linked_verifiable_presentation_endpoint2 = "linked-verifiable-presentation2.jwt";
        holder
            .add_linked_verifiable_presentation(
                service_id2,
                vec![(
                    linked_verifiable_presentation_endpoint2.to_string(),
                    vec![verifiable_credential_jwt_2],
                )],
            )
            .await;

        holder.add_well_known_did_json().await;

        let validated = validate_linked_verifiable_presentations(&holder.subject, holder.did().as_ref()).await;

        assert_eq!(validated.len(), 2);
        assert_eq!(validated[0].len(), 1);
        assert_eq!(validated[1].len(), 1);
        assert_eq!(validated[0][0].credential.display_name, "Webshop");
        assert_eq!(validated[0][0].credential.metadata.icon, Some(logo_uri_a));
        assert_eq!(validated[0][0].issuer_domain_validations.len(), 1);
        assert_eq!(validated[0][0].issuer_domain_validations[0].url, issuer_a.domain);
        assert_eq!(validated[1][0].credential.display_name, "Webshop");
        assert_eq!(validated[1][0].credential.metadata.icon, Some(logo_uri_b));
        assert_eq!(validated[1][0].issuer_domain_validations.len(), 1);
        assert_eq!(validated[1][0].issuer_domain_validations[0].url, issuer_b.domain);
    }

    #[tokio::test]
    async fn validate_linked_verifiable_presentations_successfully_considers_missing_issuer_domain_linkage() {
        let mut holder = TestEntity::new().await;

        let mut issuer = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer
            .add_well_known_did_configuration_json("linked-domain", &[issuer.domain.clone().into()])
            .await;

        // This time we do not add the `/did.json` endpoint to the issuer mock server, which makes it impossible to
        // validate the domain linkage of the issuer.
        // issuer.add_well_known_did_json().await;

        let verifiable_credential_jwt = issuer
            .issue_credential(
                holder.did().as_str(),
                "Webshop",
                "https://webshop.com/logo.jpg".parse().unwrap(),
            )
            .await;

        let service_id = "linked-verifiable-presentation";
        let linked_verifiable_presentation_endpoint = "linked-verifiable-presentation.jwt";

        // Add the linked verifiable presentation endpoint and the service to the holder DID Document.
        holder
            .add_linked_verifiable_presentation(
                service_id,
                vec![(
                    linked_verifiable_presentation_endpoint.to_string(),
                    vec![verifiable_credential_jwt],
                )],
            )
            .await;

        holder.add_well_known_did_json().await;

        assert_eq!(
            validate_linked_verifiable_presentations(&holder.subject, holder.did().as_ref(),).await,
            // The domain linkage validation of the issuer failed, so the linked verifiable credential is not considered.
            vec![vec![]]
        );
    }

    #[tokio::test]
    async fn get_linked_verifiable_presentation_urls_successfully_retrieves_urls() {
        let mut holder = TestEntity::new().await;

        let service_id = "linked-verifiable-presentation";
        let linked_verifiable_presentation_endpoint = "linked-verifiable-presentation.jwt";
        let linked_verifiable_presentation_endpoint2 = "linked-verifiable-presentation2.jwt";
        holder
            .add_linked_verifiable_presentation(
                service_id,
                vec![
                    (
                        linked_verifiable_presentation_endpoint.to_string(),
                        // The content of the presentations does not matter here.
                        vec![],
                    ),
                    (linked_verifiable_presentation_endpoint2.to_string(), vec![]),
                ],
            )
            .await;

        // Assert that the URLs of both linked verifiable presentations are retrieved.
        assert!(get_linked_verifiable_presentation_urls(&holder.did_document())
            .iter()
            .all(|item| [
                format!("{}{}", holder.domain, linked_verifiable_presentation_endpoint)
                    .parse()
                    .unwrap(),
                format!("{}{}", holder.domain, linked_verifiable_presentation_endpoint2)
                    .parse()
                    .unwrap()
            ]
            .contains(item)));
    }

    #[tokio::test]
    async fn get_linked_domains_supports_all_service_endpoint_forms() {
        let mut issuer = TestEntity::new().await;
        let domain_a: url::Url = "https://a.example.com/".parse().unwrap();
        let domain_b: url::Url = "https://b.example.com/".parse().unwrap();
        let domain_c: url::Url = "https://c.example.com/".parse().unwrap();

        for (service_id, service_endpoint) in [
            ("origins", json!({ "origins": [domain_a] })),
            ("url", json!(domain_b)),
            ("urls", json!([domain_c])),
        ] {
            issuer.add_service(service_id, "LinkedDomains", service_endpoint);
        }

        let linked_domains = get_linked_domains(&issuer.did_document());

        assert_eq!(linked_domains.len(), 3);
        assert!([domain_a, domain_b, domain_c]
            .iter()
            .all(|domain| linked_domains.contains(domain)));
    }

    #[tokio::test]
    async fn get_validated_linked_credential_data_successfully_returns_linked_verifiable_credential_data() {
        let mut issuer = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer
            .add_well_known_did_configuration_json("linked-domain", &[issuer.domain.clone().into()])
            .await;
        issuer.add_well_known_did_json().await;
        issuer.add_logo_endpoint().await;

        let mut holder = TestEntity::new().await;

        let issuer_logo = format!("{}logo.png", issuer.domain.clone());

        let verifiable_credential_jwt = issuer
            .issue_credential(holder.did().as_str(), "Webshop", issuer_logo.parse().unwrap())
            .await;

        let service_id = "linked-verifiable-presentation";
        let linked_verifiable_presentation_endpoint = "linked-verifiable-presentation.jwt";
        holder
            .add_linked_verifiable_presentation(
                service_id,
                vec![(
                    linked_verifiable_presentation_endpoint.to_string(),
                    vec![verifiable_credential_jwt],
                )],
            )
            .await;

        let linked_verifiable_presentation_url: url::Url =
            format!("{}{linked_verifiable_presentation_endpoint}", holder.domain)
                .parse()
                .unwrap();

        let validated_linked_presentation_data = get_validated_linked_presentation_data(
            &holder.subject,
            &holder.did_document(),
            linked_verifiable_presentation_url,
        )
        .await
        .unwrap();

        assert_eq!(validated_linked_presentation_data.len(), 1);
        let item = &validated_linked_presentation_data[0];
        assert_eq!(item.credential.display_name, "Webshop");
        assert_eq!(item.credential.metadata.icon, Some(issuer_logo));
        assert_eq!(item.issuer_domain_validations.len(), 1);
        assert_eq!(item.issuer_domain_validations[0].url, issuer.domain);
    }

    /// Validates a linked verifiable presentation holding one credential of an issuer with a valid domain linkage.
    /// `edit_payload` is applied to the credential's JWT payload, with the issuer's domain, which serves no status list.
    async fn validated_linked_credential_data(
        edit_payload: impl FnOnce(&mut Value, &url::Url),
    ) -> Vec<LinkedVerifiableCredentialData> {
        let mut issuer = TestEntity::new().await;
        issuer
            .add_well_known_did_configuration_json("linked-domain", &[issuer.domain.clone().into()])
            .await;
        issuer.add_well_known_did_json().await;

        let mut holder = TestEntity::new().await;

        let issuer_domain = issuer.domain.clone();
        let verifiable_credential_jwt = issuer
            .issue_credential_with(
                holder.did().as_str(),
                "Webshop",
                format!("{issuer_domain}logo.png").parse().unwrap(),
                |payload| edit_payload(payload, &issuer_domain),
            )
            .await;

        let linked_verifiable_presentation_endpoint = "linked-verifiable-presentation.jwt";
        holder
            .add_linked_verifiable_presentation(
                "linked-verifiable-presentation",
                vec![(
                    linked_verifiable_presentation_endpoint.to_string(),
                    vec![verifiable_credential_jwt],
                )],
            )
            .await;

        get_validated_linked_presentation_data(
            &holder.subject,
            &holder.did_document(),
            holder.domain.join(linked_verifiable_presentation_endpoint).unwrap(),
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn get_validated_linked_credential_data_skips_credentials_whose_status_claim_cannot_be_checked() {
        let validated_linked_credential_data = validated_linked_credential_data(|payload, issuer_domain| {
            payload["status"] = json!({
                "status_list": { "idx": 0, "uri": issuer_domain.join("status-lists/1").unwrap() }
            });
        })
        .await;

        assert!(validated_linked_credential_data.is_empty());
    }

    #[tokio::test]
    async fn get_validated_linked_credential_data_skips_credentials_with_a_malformed_status_claim() {
        let validated_linked_credential_data = validated_linked_credential_data(|payload, issuer_domain| {
            payload["status"] = json!({
                "status_list": { "idx": "0", "uri": issuer_domain.join("status-lists/1").unwrap() }
            });
        })
        .await;

        assert!(validated_linked_credential_data.is_empty());
    }

    #[tokio::test]
    async fn get_validated_linked_credential_data_keeps_credentials_with_an_unsupported_status_mechanism() {
        let validated_linked_credential_data = validated_linked_credential_data(|payload, issuer_domain| {
            payload["vc"]["credentialStatus"] = json!({
                "id": issuer_domain.join("status-lists/1#0").unwrap(),
                "type": "BitstringStatusListEntry",
                "statusPurpose": "revocation",
                "statusListIndex": "0",
                "statusListCredential": issuer_domain.join("status-lists/1").unwrap(),
            });
        })
        .await;

        assert_eq!(validated_linked_credential_data.len(), 1);
        assert_eq!(validated_linked_credential_data[0].credential.credential_status, None);
    }

    #[tokio::test]
    async fn get_validated_linked_credential_data_keeps_credentials_without_a_status() {
        let validated_linked_credential_data = validated_linked_credential_data(|_, _| {}).await;

        assert_eq!(validated_linked_credential_data.len(), 1);
        assert_eq!(validated_linked_credential_data[0].credential.display_name, "Webshop");
    }

    /// identity-kit requires every credential of a linked presentation to have the holder as its subject (the DIF
    /// specification's SHOULD); the IOTA validator did not check it. A credential that fails is left out without hiding
    /// the valid one next to it.
    #[tokio::test]
    async fn a_credential_about_someone_else_is_left_out_of_a_linked_presentation() {
        let mut issuer = TestEntity::new().await;
        issuer
            .add_well_known_did_configuration_json("linked-domain", &[issuer.domain.clone()])
            .await;
        issuer.add_well_known_did_json().await;
        let mut holder = TestEntity::new().await;
        let logo: url::Url = format!("{}logo.png", issuer.domain).parse().unwrap();

        let about_the_holder = issuer.issue_credential(&holder.did(), "Holder's", logo.clone()).await;
        let about_someone_else = issuer
            .issue_credential("did:web:someone-else.example", "Someone else's", logo)
            .await;
        holder
            .add_linked_verifiable_presentation(
                "linked-verifiable-presentation",
                vec![(
                    "linked-verifiable-presentation.jwt".to_string(),
                    vec![about_someone_else, about_the_holder],
                )],
            )
            .await;

        let validated = get_validated_linked_presentation_data(
            &holder.subject,
            &holder.did_document(),
            holder.domain.join("linked-verifiable-presentation.jwt").unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(validated.len(), 1);
        assert_eq!(validated[0].credential.display_name, "Holder's");
    }

    /// identity-kit requires the holder to sign its linked presentation with a key under `authentication` (DIF Linked
    /// Verifiable Presentation). The same presentation is rejected once that key is listed only under
    /// `assertionMethod`.
    #[tokio::test]
    async fn a_linked_presentation_signed_with_a_key_outside_authentication_is_rejected() {
        let mut issuer = TestEntity::new().await;
        issuer.add_well_known_did_json().await;
        let mut holder = TestEntity::new().await;
        let logo: url::Url = format!("{}logo.png", issuer.domain).parse().unwrap();

        let credential = issuer.issue_credential(&holder.did(), "Holder's", logo).await;
        holder
            .add_linked_verifiable_presentation(
                "linked-verifiable-presentation",
                vec![("linked-verifiable-presentation.jwt".to_string(), vec![credential])],
            )
            .await;
        let url = holder.domain.join("linked-verifiable-presentation.jwt").unwrap();
        let resolver = holder.subject.resolver().await;

        let validated = validate_linked_verifiable_presentation(&resolver, &holder.did_document(), url.clone()).await;
        assert_eq!(validated.map(|credentials| credentials.len()), Some(1));

        holder.document["authentication"] = json!([]);
        assert!(
            validate_linked_verifiable_presentation(&resolver, &holder.did_document(), url)
                .await
                .is_none()
        );
    }

    /// The indices of a linked presentation's failed credentials count every entry of `verifiableCredential`, including
    /// those that are not JWTs. Each credential that validated is kept with its own JWT.
    #[tokio::test]
    async fn a_linked_presentation_keeps_each_credential_with_its_own_jwt_next_to_entries_that_are_no_jwt() {
        let mut issuer = TestEntity::new().await;
        issuer.add_well_known_did_json().await;
        let holder = TestEntity::new().await;
        let logo: url::Url = format!("{}logo.png", issuer.domain).parse().unwrap();

        let valid = issuer.issue_credential(&holder.did(), "Valid", logo.clone()).await;
        let signed = issuer.issue_credential(&holder.did(), "Tampered", logo).await;
        // The same credential under another credential's signature.
        let (signing_input, _) = signed.as_str().rsplit_once('.').unwrap();
        let (_, signature) = valid.as_str().rsplit_once('.').unwrap();
        let tampered = Jwt::new(format!("{signing_input}.{signature}")).unwrap();

        let claims = LinkedPresentationBuilder::new(Did::parse(&holder.did()).unwrap())
            .with_credential(valid.clone())
            .with_credential(tampered)
            .jwt_claims()
            .unwrap();
        let mut claims = serde_json::to_value(claims).unwrap();
        claims["vp"]["verifiableCredential"]
            .as_array_mut()
            .unwrap()
            .insert(0, Value::Null);
        let presentation = holder.generate_jwt(&claims, Some("JWT")).await;
        Mock::given(method("GET"))
            .and(path("/mixed.jwt"))
            .respond_with(ResponseTemplate::new(200).set_body_string(presentation.as_str()))
            .mount(&holder.mock_server)
            .await;

        let resolver = holder.subject.resolver().await;
        let validated = validate_linked_verifiable_presentation(
            &resolver,
            &holder.did_document(),
            holder.domain.join("mixed.jwt").unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(validated.len(), 1);
        assert_eq!(validated[0].0, valid);
    }

    #[tokio::test]
    async fn get_validated_linked_domains_returns_only_successfully_validated_linked_domains() {
        let mut issuer1 = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer1
            .add_well_known_did_configuration_json("linked-domain", &[issuer1.domain.clone().into()])
            .await;
        issuer1.add_well_known_did_json().await;

        let resolver = Resolver::new();

        // Successfully validate the linked domain.
        let results = get_validated_linked_domains(&resolver, &[issuer1.domain.clone()], issuer1.did().as_ref()).await;
        assert_eq!(
            results.into_iter().map(|r| r.url).collect::<Vec<url::Url>>(),
            vec![issuer1.domain.clone()]
        );

        // Assert that only one domain was validated.
        let results = get_validated_linked_domains(
            &resolver,
            &[issuer1.domain.clone(), "http://invalid-domain.org".parse().unwrap()],
            issuer1.did().as_ref(),
        )
        .await;
        assert_eq!(
            results.into_iter().map(|r| r.url).collect::<Vec<url::Url>>(),
            vec![issuer1.domain.clone()]
        );

        let mut issuer2 = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer2
            .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
            .await;
        issuer2.add_well_known_did_json().await;

        // Assert that only one domain was validated. The second domain cannot be validated because the issuer DID is different.
        let results = get_validated_linked_domains(
            &resolver,
            &[issuer1.domain.clone(), issuer2.domain.clone()],
            issuer1.did().as_ref(),
        )
        .await;
        assert_eq!(
            results.into_iter().map(|r| r.url).collect::<Vec<url::Url>>(),
            vec![issuer1.domain.clone()]
        );

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server. Use the same issuer DID as
        // issuer1, but a different domain.
        let mut issuer2 = TestEntity::new().await;
        issuer2.document = issuer1.document.clone();
        issuer2.stronghold_manager = issuer1.stronghold_manager.clone();

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer2
            .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
            .await;
        issuer2.add_well_known_did_json().await;

        // Assert that both domains were validated (regardless of the order).
        let results = get_validated_linked_domains(
            &resolver,
            &[issuer1.domain.clone(), issuer2.domain.clone()],
            issuer1.did().as_ref(),
        )
        .await;
        assert_eq!(results.len(), 2);
        assert!(results
            .iter()
            .all(|item| [issuer1.domain.clone(), issuer2.domain.clone()].contains(&item.url)));
    }
}
