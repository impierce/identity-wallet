use crate::{
    http_client::get_http_client,
    state::{
        core_utils::helpers::{download_logo, get_issuer_document},
        credentials::{
            reducers::send_token_request::get_credential_status, DisplayCredential, VerifiableCredentialRecord,
        },
        did::validate_domain_linkage::{ValidationResult, ValidationStatus, Verifier},
    },
    subject::Subject,
};
use did_manager::Resolver;
use futures::{
    future::OptionFuture,
    stream::{iter, FuturesUnordered},
    StreamExt,
};
use identity_iota::{
    core::{OneOrMany, ToJson},
    credential::{
        DecodedJwtCredential, DecodedJwtPresentation, FailFast, Jwt, JwtCredentialValidationOptions,
        JwtCredentialValidator, JwtPresentationValidator, StatusCheck, Subject as CredentialSubject,
    },
    document::{CoreDocument, Service},
};
use log::{debug, info, warn};
use oid4vc::oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;
use url::Url;

#[cfg_attr(not(test), derive(PartialEq))]
#[derive(Clone, Serialize, Deserialize, Debug, TS, Default)]
#[ts(export, export_to = "bindings/user_prompt/LinkedVerifiableCredentialData.ts")]
pub struct LinkedVerifiableCredentialData {
    pub credential: DisplayCredential,
    pub issuer_domain_validations: Vec<ValidationResult>,
    // pub issuer_linked_domains: Vec<Url>,
}

// Skip the partial equality check for `issuance_date` during testing.
#[cfg(test)]
impl PartialEq for LinkedVerifiableCredentialData {
    fn eq(&self, _other: &Self) -> bool {
        // self.name == other.name
        //     && self.logo_uri == other.logo_uri
        //     && self.issuer_linked_domains == other.issuer_linked_domains
        todo!()
    }
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

    iter(
        // Get all linked verifiable presentation URLs from the holder document
        holder_document
            .service()
            .iter()
            .filter_map(get_linked_verifiable_presentation_urls)
            .flatten(),
    )
    .filter_map(|linked_verifiable_presentation_url| {
        debug!("Processing linked verifiable presentation URL: {linked_verifiable_presentation_url}");
        // Validate the linked verifiable presentation and get the linked verifiable credential data
        get_validated_linked_presentation_data(subject, &holder_document, linked_verifiable_presentation_url)
    })
    .collect::<Vec<_>>()
    .await
}

/// Get the linked verifiable presentation URLs from the service. It returns a list of URLs if the service type is a
/// `LinkedVerifiablePresentation`.
fn get_linked_verifiable_presentation_urls(service: &Service) -> Option<Vec<Url>> {
    service
        .type_()
        .contains("LinkedVerifiablePresentation")
        .then(|| {
            debug!("Found LinkedVerifiablePresentation service: {service:#?}");
            service.service_endpoint()
        })
        .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
        .and_then(
            // Parse the linked verifiable presentation URLs from the service endpoint. The service endpoint must be
            // either a string or an array of strings: https://identity.foundation/linked-vp/#linked-verifiable-presentation
            |linked_verifiable_presentation_urls| match linked_verifiable_presentation_urls {
                Value::String(url) => url
                    .parse()
                    .inspect_err(|err| warn!("Failed to parse linked verifiable presentation URL: {err}"))
                    .ok()
                    .map(|url| vec![url]),
                Value::Array(array) => Some(
                    array
                        .into_iter()
                        .filter_map(|url| {
                            url.as_str().and_then(|url| {
                                url.parse()
                                    .inspect_err(|err| {
                                        warn!("Failed to parse linked verifiable presentation URL: {err}")
                                    })
                                    .ok()
                            })
                        })
                        .collect(),
                ),
                _ => None,
            },
        )
}

/// Validate the linked verifiable presentations for the given holder document and linked verifiable presentation URL.
/// It returns a list of linked verifiable credential data.
async fn get_validated_linked_presentation_data(
    subject: &Subject,
    holder_document: &CoreDocument,
    linked_verifiable_presentation_url: Url,
) -> Option<Vec<LinkedVerifiableCredentialData>> {
    OptionFuture::from(
        validate_linked_verifiable_presentation(holder_document, linked_verifiable_presentation_url)
            .await
            .map(|linked_verifiable_presentation| {
                get_validated_linked_credential_data(subject, linked_verifiable_presentation)
            }),
    )
    .await
}

/// Retrieves the linked verifiable presentation from the given URL and validates it against the holder document.
/// Returns the decoded linked verifiable presentation if successful.
async fn validate_linked_verifiable_presentation(
    holder_document: &CoreDocument,
    linked_verifiable_presentation_url: Url,
) -> Option<DecodedJwtPresentation<Jwt>> {
    let response = get_http_client()
        .await
        .get(linked_verifiable_presentation_url)
        .send()
        .await
        .inspect_err(|err| {
            warn!("Failed to retrieve linked verifiable presentation: {err}");
        })
        .ok()?;
    let status = response.status();

    response
        .text()
        .await
        .inspect_err(|err| {
            warn!("Failed to read linked verifiable presentation response: {err}");
        })
        .ok()
        .and_then(|presentation_jwt| {
            status.is_success().then(|| {
                debug!("Validating linked verifiable presentation JWT: {presentation_jwt}");

                let validator = JwtPresentationValidator::with_signature_verifier(Verifier);
                validator
                    .validate(&presentation_jwt.into(), &holder_document, &Default::default())
                    .inspect_err(|err| {
                        warn!("Failed to validate linked verifiable presentation: {err:#?}");
                    })
                    .ok()
            })?
        })
}

/// Validate the linked verifiable credentials in the linked verifiable presentation. Skips invalid credentials or credentials with invalid domain linkage.
/// Since anyone can host a linked verifiable presentation, it is important to validate the linked verifiable
/// credentials. The `issuer` field in the linked verifiable credential is used to resolve the issuer document and which
/// is then used to retrieve the linked domains. The linked domains then are used to validate the domain linkage.
async fn get_validated_linked_credential_data(
    subject: &Subject,
    linked_verifiable_presentation: DecodedJwtPresentation<Jwt>,
) -> Vec<LinkedVerifiableCredentialData> {
    let resolver = subject.resolver().await;
    iter(linked_verifiable_presentation.presentation.verifiable_credential)
        .filter_map(|linked_verifiable_credential_jwt| {
            let resolver = resolver.clone();
            async move {
                // Resolve the issuer document and issuer DID
                let issuer_document = get_issuer_document(&resolver, &linked_verifiable_credential_jwt).await?;
                let issuer_did = issuer_document.id().to_string();

                info!("Issuer document: {issuer_document:#?}");

                // Resolve the issuer linked domains from the issuer document
                let issuer_linked_domains = get_issuer_linked_domains(&issuer_document).await;

                info!("Issuer linked domains: {issuer_linked_domains:#?}");

                // Only linked verifiable credentials with at least one successful domain linkage validation are considered
                let validated_linked_domains = get_validated_linked_domains(&resolver, &issuer_linked_domains, &issuer_did).await;

                if !validated_linked_domains.is_empty() {
                    let validator = JwtCredentialValidator::with_signature_verifier(Verifier);

                    // `SkipUnsupported` allows for custom credential types, such as the StatusList2021Entry (https://www.w3.org/TR/2023/WD-vc-status-list-20230427/#statuslist2021entry)
                    let options = JwtCredentialValidationOptions::new().status_check(StatusCheck::SkipUnsupported);

                    // Decode the linked verifiable credential and validate the jwt_vc_json, checks the JWT and the Issuer DID
                    if let Ok(linked_verifiable_credential) = validator.validate::<_, Value>(
                        &linked_verifiable_credential_jwt,
                        &issuer_document,
                        &options,
                        FailFast::FirstError,
                    ) {
                        info!("Validated linked verifiable credential JWT: {linked_verifiable_credential:#?}");

                        let credential_subject = match &linked_verifiable_credential.credential.credential_subject {
                            OneOrMany::One(subject) => Some(subject),
                            // TODO: how to handle multiple credential subjects?
                            OneOrMany::Many(subjects) => subjects.first(),
                        };

                        if let Some(credential_subject) = credential_subject {
                            let name = get_name(credential_subject);

                            let linked_domains = validated_linked_domains.iter().map(|result| result.url.clone()).collect::<Vec<url::Url>>();
                            let logo_uri = get_logo_uri(credential_subject, &linked_verifiable_credential, &linked_domains).await;
                            let issuance_date = linked_verifiable_credential.credential.issuance_date.to_rfc3339();

                            debug!("LinkedVerifiableCredentialData: name: {name:?}, logo_uri: {logo_uri:?}, issuance_date: {issuance_date}, validated_linked_domains: {linked_domains:#?}");

                            let mut verifiable_credential_record = VerifiableCredentialRecord::try_new(CredentialFormats::JwtVcJson(()), serde_json::json!(linked_verifiable_credential_jwt), vec![]).unwrap();

                            verifiable_credential_record.display_credential.credential_status = get_credential_status(&verifiable_credential_record, subject).await;
                            verifiable_credential_record.display_credential.issuer_name = name.unwrap_or_default();
                            verifiable_credential_record.display_credential.issuer_logo_uri = logo_uri;

                            Some(LinkedVerifiableCredentialData {
                                credential: verifiable_credential_record.display_credential,
                                issuer_domain_validations: validated_linked_domains,
                            })
                        }
                        else {
                            warn!("Failed to get credential_subject from linked_verifiable_credential: {linked_verifiable_credential:#?}");
                            None
                        }
                    } else {
                        warn!("Failed to validate linked verifiable credential: {linked_verifiable_credential_jwt:#?}");
                        None
                    }
                } else {
                    warn!("No validated linked domains for issuer DID: {issuer_did}");
                    None
                }
            }
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
                // Silence unused variable warning
                let _issuer_did = issuer_did;
                // Skip validation during tests
                ValidationResult {
                    status: ValidationStatus::default(),
                    url: issuer_linked_domain.clone(),
                    name: None,
                    logo_uri: None,
                    issuance_date: None,
                    message: None,
                }
            }
        };

        if validation_result.status == ValidationStatus::Success {
            info!("Successfully validated domain linkage for issuer linked domain: {issuer_linked_domain}");
            Some(validation_result)
        } else {
            warn!("Failed to validate domain linkage for issuer linked domain: {issuer_linked_domain}");
            None
        }
    }))
    .filter_map(|result| async move { result })
    .collect()
    .await
}

/// Get the linked domains from the issuer document. It returns a list of URLs if the service type is `LinkedDomains`.
async fn get_issuer_linked_domains(issuer_document: &CoreDocument) -> Vec<Url> {
    issuer_document
        .service()
        .iter()
        .filter_map(|service| {
            service
                .type_()
                .contains("LinkedDomains")
                .then(|| service.service_endpoint())
                .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
                .and_then(|linked_domain| {
                    linked_domain.get("origins").and_then(|origins| {
                        origins.as_array().and_then(|origins| {
                            origins
                                .iter()
                                .map(|origin| {
                                    origin.as_str().and_then(|origin| {
                                        origin
                                            .parse()
                                            .inspect_err(|err| warn!("Failed to parse linked domain: {err:#?}"))
                                            .ok()
                                    })
                                })
                                .collect::<Option<Vec<Url>>>()
                        })
                    })
                })
        })
        .flatten()
        .collect()
}

fn get_name(credential_subject: &CredentialSubject) -> Option<String> {
    credential_subject
        .properties
        .get("name")
        .or_else(|| credential_subject.properties.get("naam")) // TODO: "naam" is expected to be used in Dutch credentials
        .or_else(|| credential_subject.properties.get("legal_person_name")) // This is another valid property name according to the following spec:
        // EWC RFC005: Issue Legal Person Identification Data (LPID) - v1.0
        // https://github.com/EWC-consortium/eudi-wallet-rfcs/blob/49faa8b0ba5e5e79836e247fd07cc0447c1ae98b/ewc-rfc005-issue-legal-person-identification-data.md#51031-lpid-attributes-specification
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

/// First, try to get the logo URI from the credential subject.
/// If this doesn't succeed, iterate through the validated linked domains and try to fetch it from the well-known/openid-credential-issuer endpoint.
/// In this endpoint, first we look inside the Display field, at the root.
/// If we can't find a logo there, we look inside the Credential Configurations Supported field at the root.
/// We try to match keys inside the Credential Configurations Supported object against the credential `type` array of the linked verifiable credential, in reverse order.
/// At first success the loop breaks and we download the image.
/// Otherwise, we use a fallback icon.
async fn get_logo_uri(
    credential_subject: &CredentialSubject,
    linked_verifiable_credential: &DecodedJwtCredential<Value>,
    validated_linked_domains: &[Url],
) -> Option<String> {
    debug!("Trying to fetch image uri from credential subject");
    let mut logo_uri = credential_subject
        .properties
        .get("image")
        .and_then(Value::as_str)
        .map(ToString::to_string);

    // Check if logo URI was retrieved, if not then attempt to retrieve from a well-known endpoint
    if logo_uri.is_none() {
        debug!("Failed to fetch image uri from credential subject");
        for domain in validated_linked_domains.iter() {
            let well_known_endpoint = format!("{domain}.well-known/openid-credential-issuer");
            debug!("Trying to fetch image uri from {well_known_endpoint} endpoint");
            if let Ok(response) = get_http_client().await.get(&well_known_endpoint).send().await {
                debug!("Response from {well_known_endpoint}: {response:#?}");
                if let Ok(metadata) = response.json::<CredentialIssuerMetadata>().await {
                    debug!("Metadata from {well_known_endpoint}: {metadata:#?}");
                    logo_uri = metadata.display.as_deref().and_then(extract_logo_uri_from_display);

                    debug!("Logo uri from {well_known_endpoint}: {logo_uri:?}");
                    if logo_uri.is_some() {
                        break;
                    }
                }
            }
            // TODO: Due to mixing 2 specs here, the oid4vci and linked verifiable presentation spec, we lose the Credential Issuer Identifier (CII) during the linked vp flow.
            // The CII tells us where exactly we can add "/.well-known/openid-credential-issuer" to fetch the Credential Issuer Metadata, in which we might find the logo.
            // For now we assume it's the same domain as the linked domain.
            // But this is no guarantee and the code below is one such workaround.
            let well_known_endpoint = format!("{domain}oid4vci/.well-known/openid-credential-issuer");
            debug!("Trying to fetch image uri from {well_known_endpoint} endpoint");
            if let Ok(response) = get_http_client().await.get(&well_known_endpoint).send().await {
                if let Ok(metadata) = response.json::<CredentialIssuerMetadata>().await {
                    logo_uri = linked_verifiable_credential.credential.types.iter().find_map(|type_| {
                        debug!("Trying to fetch image uri from Credential Configuration Supported: {type_}");
                        metadata
                            .credential_configurations_supported
                            .get(type_)
                            .and_then(|credential_configuration| {
                                credential_configuration
                                    .credential_metadata
                                    .as_ref()?
                                    .display
                                    .as_ref()?
                                    .first()
                            })
                            .and_then(|display| display.logo.clone())
                            .map(|logo| logo.uri.to_string())
                    });

                    if logo_uri.is_some() {
                        break;
                    }
                }
            }
        }
    }

    if let Some(logo_uri_str) = logo_uri {
        download_logo(&logo_uri_str).await
    } else {
        warn!("No logo URI found");
        None
    }
}

fn extract_logo_uri_from_display(display: &[Value]) -> Option<String> {
    display
        .first()
        .and_then(|display| display.get("logo"))
        .and_then(|logo| logo.get("uri").or(logo.get("url")))
        .and_then(|url| url.as_str())
        .map(ToString::to_string)
}

#[cfg(not(feature = "test_utils"))]
#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::URL_SAFE_NO_PAD;
    use base64::Engine;
    use did_manager::SecretManager;
    use identity_credential::domain_linkage::{DomainLinkageConfiguration, DomainLinkageCredentialBuilder};
    use identity_iota::{
        core::{Duration, FromJson as _, Object, OrderedSet, Timestamp, Url},
        credential::{Credential, CredentialBuilder, Presentation},
        document::{CoreDocument, Service, ServiceEndpoint},
        verification::jws::JwsAlgorithm,
    };
    use jsonwebtoken::{Algorithm, Header};
    use serde_json::json;
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::sync::Mutex;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    // 'Entity' struct that represents a digital identity including a DID Document, a domain, and a secret manager.
    struct TestEntity {
        pub mock_server: MockServer,
        pub domain: url::Url,
        pub did_document: CoreDocument,
        pub secret_manager: Arc<Mutex<SecretManager>>,
        pub subject: Arc<Subject>,
    }

    impl TestEntity {
        // Create a new 'Entity' with a DID Document, mock server, a domain, and a secret manager.
        async fn new() -> Self {
            engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

            let mock_server = MockServer::start().await;

            let uri = mock_server.uri();
            let port = uri.split(':').next_back().unwrap();
            let domain: url::Url = format!("http://localhost:{port}").parse().unwrap();

            let temp_dir = TempDir::new().unwrap();
            let path = temp_dir.path().join("stronghold.stronghold");
            let snapshot_path = path.as_os_str().to_str().unwrap();

            let mut secret_manager = SecretManager::builder()
                .snapshot_path(snapshot_path)
                .password("sup3rSecr3t")
                .build()
                .await
                .unwrap();

            let did_document = secret_manager
                .produce_document(
                    did_manager::DidMethod::Web,
                    Some(did_manager::MethodSpecificParameters::Web {
                        origin: domain.origin(),
                    }),
                    identity_iota::verification::jws::JwsAlgorithm::ES256,
                )
                .await
                .unwrap();

            *crate::persistence::STRONGHOLD.lock().unwrap() = path.clone();
            let stronghold_manager = Arc::new(crate::stronghold::StrongholdManager::create("sup3rSecr3t").unwrap());
            let secret_manager = Arc::new(Mutex::new(secret_manager));
            let subject = Arc::new(Subject {
                stronghold_manager,
                secret_manager: secret_manager.clone(),
                resolver: tokio::sync::OnceCell::new(),
            });

            TestEntity {
                mock_server,
                domain,
                did_document,
                secret_manager,
                subject,
            }
        }

        // Add the `.well-known/did.json` endpoint to the mock server.
        async fn add_well_known_did_json(&self) {
            Mock::given(method("GET"))
                .and(path(".well-known/did.json"))
                .respond_with(ResponseTemplate::new(200).set_body_json(json!(self.did_document)))
                .mount(&self.mock_server)
                .await;
        }

        // Add the `.well-known/did-configuration.json` endpoint to the mock server.
        async fn add_well_known_did_configuration_json(&mut self, service_id: &str, origins: &[Url]) {
            let service = Service::builder(Default::default())
                .id(format!("{}#{service_id}", self.did_document.id()).parse().unwrap())
                .type_("LinkedDomains")
                .service_endpoint(
                    serde_json::from_value::<ServiceEndpoint>(serde_json::json!(
                        {
                            "origins": origins
                        }
                    ))
                    .unwrap(),
                )
                .build()
                .expect("Failed to create DID Configuration Resource");
            self.did_document
                .insert_service(service)
                .expect("Service already exists in DID Document");

            let domain_linkage_configuration = {
                let origin = Url::parse(self.domain.origin().ascii_serialization()).unwrap();
                let payload = DomainLinkageCredentialBuilder::new()
                    .issuer(self.did_document.id().clone())
                    .origin(origin)
                    .issuance_date(Timestamp::now_utc())
                    .expiration_date(Timestamp::now_utc().checked_add(Duration::seconds(60)).unwrap())
                    .build()
                    .and_then(|credential| credential.serialize_jwt(Default::default()))
                    .unwrap();

                DomainLinkageConfiguration::new(vec![self.generate_jwt(payload).await])
            };

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
            let mut urls: Vec<Url> = vec![];

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

                let linked_verifiable_presentation = {
                    let presentation = {
                        let mut builder =
                            Presentation::builder(self.did_document.id().to_string().parse().unwrap(), Object::new());
                        for linked_verifiable_credential_jwt in linked_verifiable_credential_jwts {
                            builder = builder.credential(linked_verifiable_credential_jwt);
                        }
                        builder.build().unwrap()
                    };

                    self.generate_jwt(presentation.serialize_jwt(&Default::default()).unwrap())
                        .await
                };

                Mock::given(method("GET"))
                    .and(path(format!("/{linked_verifiable_presentation_endpoint}")))
                    .respond_with(ResponseTemplate::new(200).set_body_string(linked_verifiable_presentation.as_str()))
                    .mount(&self.mock_server)
                    .await;
            }

            let service_endpoint = match urls.len() {
                // Value::String
                1 => ServiceEndpoint::from(urls[0].clone()),
                // Value::Array
                _ => ServiceEndpoint::from(OrderedSet::from_iter(urls)),
            };
            let service = Service::builder(Default::default())
                .id(format!("{}#{service_id}", self.did_document.id()).parse().unwrap())
                .type_("LinkedVerifiablePresentation")
                .service_endpoint(service_endpoint)
                .build()
                .unwrap();

            self.did_document
                .insert_service(service)
                .expect("Service already exists in DID Document");
        }

        // 'Issues' a Credential Jwt to a subject.
        async fn issue_credential(&mut self, subject_id: &str, subject_name: &str, subject_image: Url) -> Jwt {
            let subject = identity_credential::credential::Subject::from_json_value(json!({
                "id": subject_id,
                "name": subject_name,
                "image": subject_image
            }))
            .unwrap();

            let issuer = identity_iota::credential::Issuer::Url(self.did_document.id().to_string().parse().unwrap());

            let credential: Credential = CredentialBuilder::default()
                .issuer(issuer)
                .subject(subject)
                .build()
                .unwrap();

            self.generate_jwt(credential.serialize_jwt(Default::default()).unwrap())
                .await
        }

        // Generates a JWT with the given payload.
        async fn generate_jwt(&mut self, payload: String) -> Jwt {
            let subject_did = self.did_document.id().to_string();

            // Compose JWT
            let header = Header {
                alg: Algorithm::ES256,
                typ: Some("JWT".to_string()),
                kid: Some(format!("{subject_did}#key-0")),
                ..Default::default()
            };

            let message = [
                URL_SAFE_NO_PAD.encode(serde_json::to_vec(&header).unwrap().as_slice()),
                URL_SAFE_NO_PAD.encode(payload.as_bytes()),
            ]
            .join(".");

            let secret_manager = self.secret_manager.lock().await;

            let proof_value = secret_manager
                .sign(message.as_bytes(), JwsAlgorithm::ES256)
                .await
                .unwrap();
            let signature = URL_SAFE_NO_PAD.encode(proof_value.as_slice());
            let message = [message, signature].join(".");

            Jwt::from(message)
        }

        async fn add_logo_endpoint(&self) {
            Mock::given(method("GET"))
                .and(path("logo.png"))
                .respond_with(ResponseTemplate::new(200).set_body_raw(
                    include_bytes!("../../../resources/images/impierce_white.png"),
                    "image/png",
                ))
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
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                logo_uri_a.parse().unwrap(),
            )
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
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                logo_uri_b.parse().unwrap(),
            )
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

        assert_eq!(
            validate_linked_verifiable_presentations(&holder.subject, holder.did_document.id().to_string().as_ref(),)
                .await,
            vec![
                vec![LinkedVerifiableCredentialData {
                    // name: Some("Webshop".to_string()),
                    // logo_uri: Some(logo_uri_a),
                    // issuer_linked_domains: vec![issuer_a.domain.clone()],
                    ..Default::default()
                }],
                vec![LinkedVerifiableCredentialData {
                    // name: Some("Webshop".to_string()),
                    // logo_uri: Some(logo_uri_b),
                    // issuer_linked_domains: vec![issuer_b.domain.clone()],
                    ..Default::default()
                }]
            ]
        );
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
                holder.did_document.id().to_string().as_str(),
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
            validate_linked_verifiable_presentations(&holder.subject, holder.did_document.id().to_string().as_ref(),)
                .await,
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
                        // Linked verifiable presentation can include multiple linked verifiable credentials.
                        vec![Jwt::from("test1".to_string()), Jwt::from("test2".to_string())],
                    ),
                    (
                        linked_verifiable_presentation_endpoint2.to_string(),
                        vec![Jwt::from("test3".to_string())],
                    ),
                ],
            )
            .await;

        // Assert that the URLs of both linked verifiable presentations are retrieved.
        assert!(
            get_linked_verifiable_presentation_urls(&holder.did_document.service()[0])
                .unwrap()
                .iter()
                .all(|item| [
                    format!("{}{}", holder.domain, linked_verifiable_presentation_endpoint)
                        .parse()
                        .unwrap(),
                    format!("{}{}", holder.domain, linked_verifiable_presentation_endpoint2)
                        .parse()
                        .unwrap()
                ]
                .contains(item))
        );
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
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                issuer_logo.parse().unwrap(),
            )
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
            &holder.did_document,
            linked_verifiable_presentation_url,
        )
        .await;

        assert_eq!(
            validated_linked_presentation_data,
            Some(vec![LinkedVerifiableCredentialData {
                // name: Some("Webshop".to_string()),
                // logo_uri: Some(issuer_logo),
                // issuer_linked_domains: vec![issuer.domain.clone()],
                ..Default::default()
            }])
        );
    }

    // #[tokio::test]
    // async fn get_validated_linked_domains_returns_only_successfully_validated_linked_domains() {
    //     let mut issuer1 = TestEntity::new().await;

    //     // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
    //     issuer1
    //         .add_well_known_did_configuration_json("linked-domain", &[issuer1.domain.clone().into()])
    //         .await;
    //     issuer1.add_well_known_did_json().await;

    //     let resolver = Resolver::new();

    //     // Successfully validate the linked domain.
    //     assert_eq!(
    //         get_validated_linked_domains(
    //             &resolver,
    //             &[issuer1.domain.clone()],
    //             issuer1.did_document.id().to_string().as_ref()
    //         )
    //         .await,
    //         vec![issuer1.domain.clone()]
    //     );

    //     // Assert that only one domain was validated.
    //     assert_eq!(
    //         get_validated_linked_domains(
    //             &resolver,
    //             &[issuer1.domain.clone(), "http://invalid-domain.org".parse().unwrap()],
    //             issuer1.did_document.id().to_string().as_ref()
    //         )
    //         .await,
    //         vec![issuer1.domain.clone()]
    //     );

    //     let mut issuer2 = TestEntity::new().await;

    //     // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
    //     issuer2
    //         .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
    //         .await;
    //     issuer2.add_well_known_did_json().await;

    //     // Assert that only one domain was validated. The second domain cannot be validated because the issuer DID is different.
    //     assert_eq!(
    //         get_validated_linked_domains(
    //             &resolver,
    //             &[issuer1.domain.clone(), issuer2.domain.clone()],
    //             issuer1.did_document.id().to_string().as_ref()
    //         )
    //         .await,
    //         vec![issuer1.domain.clone()]
    //     );

    //     // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server. Use the same issuer DID as
    //     // issuer1, but a different domain.
    //     let mut issuer2 = TestEntity::new().await;
    //     issuer2.did_document = issuer1.did_document.clone();
    //     issuer2.secret_manager = issuer1.secret_manager.clone();

    //     // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
    //     issuer2
    //         .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
    //         .await;
    //     issuer2.add_well_known_did_json().await;

    //     // Assert that both domains were validated (regardless of the order).
    //     assert!(get_validated_linked_domains(
    //         &resolver,
    //         &[issuer1.domain.clone(), issuer2.domain.clone()],
    //         issuer1.did_document.id().to_string().as_ref()
    //     )
    //     .await
    //     .iter()
    //     .all(|item| [issuer1.domain.clone(), issuer2.domain.clone()].contains(item)));
    // }
}
