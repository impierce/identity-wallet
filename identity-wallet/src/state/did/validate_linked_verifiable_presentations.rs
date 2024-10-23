use crate::{
    persistence::{download_asset, hash},
    state::{
        core_utils::helpers::get_unverified_jwt_claims,
        did::validate_domain_linkage::{validate_domain_linkage, ValidationStatus, Verifier},
    },
};

use chrono::DateTime;
use did_manager::Resolver;
use futures::{
    future::OptionFuture,
    stream::{iter, FuturesUnordered},
    StreamExt,
};
use identity_iota::{
    core::{OneOrMany, ToJson},
    credential::{DecodedJwtPresentation, FailFast, Jwt, JwtCredentialValidator, JwtPresentationValidator, Subject},
    document::{CoreDocument, Service},
    verification::jws::Decoder,
};
use identity_jose::jwt::JwtClaims;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;
use url::Url;

#[cfg_attr(not(test), derive(PartialEq))]
#[derive(Clone, Serialize, Deserialize, Debug, TS, Default)]
#[ts(export, export_to = "bindings/user_prompt/LinkedVerifiableCredentialData.ts")]
pub struct LinkedVerifiableCredentialData {
    pub name: Option<String>,
    pub logo_uri: Option<String>,
    pub issuance_date: String,
    #[ts(skip)]
    pub issuer_linked_domains: Vec<Url>,
}

// Skip the partial equality check for `issuance_date` during testing.
#[cfg(test)]
impl PartialEq for LinkedVerifiableCredentialData {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.logo_uri == other.logo_uri
            && self.issuer_linked_domains == other.issuer_linked_domains
    }
}

/// Validate the linked verifiable presentations for the given holder DID. Returns a list of linked verifiable
/// credential data. It starts by resolving the holder DID and then iterates over the linked verifiable presentation
/// URLs. For each linked verifiable presentation, it validates the presentation and then validates the linked
/// verifiable credentials. It only considers linked verifiable credentials with successful domain linkage validation.
pub async fn validate_linked_verifiable_presentations(holder_did: &str) -> Vec<Vec<LinkedVerifiableCredentialData>> {
    info!("Validating linked verifiable presentations for holder DID: {holder_did}");

    let resolver = Resolver::new().await;

    let holder_document = match resolver.resolve(holder_did).await {
        Ok(holder_document) => holder_document,
        _ => {
            warn!("Failed to resolve holder DID: {holder_did}");
            return vec![];
        }
    };

    info!("Holder document: {holder_document:#?}");

    iter(
        // Get all linked verifiable presentation URLs from the holder document
        holder_document
            .service()
            .iter()
            .filter_map(get_linked_verifiable_presentation_urls)
            .flatten(),
    )
    .filter_map(|linked_verifiable_presentation_url| {
        // Validate the linked verifiable presentation and get the linked verifiable credential data
        get_validated_linked_presentation_data(&resolver, &holder_document, linked_verifiable_presentation_url)
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
            info!("Found LinkedVerifiablePresentation service: {service:#?}");
            service.service_endpoint()
        })
        .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
        .and_then(
            // Parse the linked verifiable presentation URLs from the service endpoint. The service endpoint must be
            // either a string or an array of strings: https://identity.foundation/linked-vp/#linked-verifiable-presentation
            |linked_verifiable_presentation_urls| match linked_verifiable_presentation_urls {
                Value::String(url) => url
                    .parse()
                    .inspect_err(|err| warn!("Failed to parse linked verifiable presentation URL: {}", err))
                    .ok()
                    .map(|url| vec![url]),
                Value::Array(array) => Some(
                    array
                        .into_iter()
                        .filter_map(|url| {
                            url.as_str().and_then(|url| {
                                url.parse()
                                    .inspect_err(|err| {
                                        warn!("Failed to parse linked verifiable presentation URL: {}", err)
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
    resolver: &Resolver,
    holder_document: &CoreDocument,
    linked_verifiable_presentation_url: Url,
) -> Option<Vec<LinkedVerifiableCredentialData>> {
    OptionFuture::from(
        validate_linked_verifiable_presentation(holder_document, linked_verifiable_presentation_url)
            .await
            .map(|linked_verifiable_presentation| {
                get_validated_linked_credential_data(resolver, linked_verifiable_presentation)
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
    let response = reqwest::get(linked_verifiable_presentation_url)
        .await
        .inspect_err(|err| {
            warn!("Failed to retrieve linked verifiable presentation: {}", err);
        })
        .ok()?;
    let status = response.status();

    response
        .text()
        .await
        .inspect_err(|err| {
            warn!("Failed to read linked verifiable presentation response: {}", err);
        })
        .ok()
        .and_then(|presentation_jwt| {
            status.is_success().then(|| {
                let validator = JwtPresentationValidator::with_signature_verifier(Verifier);
                validator
                    .validate(&presentation_jwt.into(), &holder_document, &Default::default())
                    .inspect_err(|err| {
                        warn!("Failed to validate linked verifiable presentation: {:#?}", err);
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
    resolver: &Resolver,
    linked_verifiable_presentation: DecodedJwtPresentation<Jwt>,
) -> Vec<LinkedVerifiableCredentialData> {
    iter(linked_verifiable_presentation.presentation.verifiable_credential)
        .filter_map(|linked_verifiable_credential| async move {
            // Resolve the issuer document and issuer DID
            let issuer_document = get_issuer_document(resolver, &linked_verifiable_credential).await?;
            let issuer_did = issuer_document.id().to_string();

            info!("Issuer document: {issuer_document:#?}");

            // Resolve the issuer linked domains from the issuer document
            let issuer_linked_domains = get_issuer_linked_domains(&issuer_document).await;

            info!("Issuer linked domains: {issuer_linked_domains:#?}");

            // Only linked verifiable credentials with at least one successful domain linkage validation are considered
            let mut validated_linked_domains = get_validated_linked_domains(&issuer_linked_domains, &issuer_did).await;

            if validated_linked_domains.is_empty() {
                if let Some(did_web_url) = extract_url_from_did_web(&issuer_did) {
                    validated_linked_domains.insert(0, did_web_url);
                }
            }

            if !validated_linked_domains.is_empty() {
                let validator = JwtCredentialValidator::with_signature_verifier(Verifier);

                // Decode the linked verifiable credential and validate it
                if let Ok(linked_verifiable_credential) = validator.validate::<_, Value>(
                    &linked_verifiable_credential,
                    &issuer_document,
                    &Default::default(),
                    FailFast::FirstError,
                ) {
                    let credential_subject = match &linked_verifiable_credential.credential.credential_subject {
                        OneOrMany::One(subject) => Some(subject),
                        // TODO: how to handle multiple credential subjects?
                        OneOrMany::Many(subjects) => subjects.first(),
                    };

                    OptionFuture::from(credential_subject.map(|credential_subject| async {
                        let name = get_name(credential_subject);
                        let logo_uri = get_logo_uri(credential_subject).await;
                        let issuance_date = linked_verifiable_credential.credential.issuance_date.to_rfc3339();

                        LinkedVerifiableCredentialData {
                            name,
                            logo_uri,
                            issuance_date,
                            issuer_linked_domains: validated_linked_domains,
                        }
                    }))
                    .await
                }
                else {
                    // TODO: Should we add more fine-grained error handling? `None` here means that the linked verifiable credential is invalid.
                    warn!("Failed to validate linked verifiable credential: {linked_verifiable_credential:#?}");

                    info!("Retrieving unverified linked verifiable credential: {linked_verifiable_credential:#?}");

                    if let Ok(linked_verifiable_credential_value) = linked_verifiable_credential.to_json_value() {
                        if let Ok(unverified_jwt_claims) = get_unverified_jwt_claims(&linked_verifiable_credential_value) {

                            info!("Unverified jwt claims: {unverified_jwt_claims:#?}");

                            if let Some(unverified_jwt_claims) = unverified_jwt_claims.get("vc") {

                                // Without the DecodedJwtCredential<Value> we can't get the credential subject as required for the functions get_name and get_logo_uri.
                                // Therefore this code is somewhat repetitive.
                                // Since nowhere in the VC 2.0 does it specify anything about the name, we have to try and catch it in all possible forms.
                                let name = unverified_jwt_claims.get("name")
                                    .or_else(|| unverified_jwt_claims.get("naam"))
                                    .or_else(|| unverified_jwt_claims.get("credentialSubject")
                                        .and_then(|cred_subject| cred_subject.get("name")))
                                    .or_else(|| unverified_jwt_claims.get("credentialSubject")
                                        .and_then(|cred_subject| cred_subject.get("naam")))
                                    .or_else(|| unverified_jwt_claims.get("credentialSubject")
                                        .and_then(|cred_subject| cred_subject.get("legal_person_name")))
                                    .and_then(Value::as_str)
                                    .map(ToString::to_string);

                                let mut logo_uri = OptionFuture::from(
                                    unverified_jwt_claims.get("credentialSubject").and_then(|cred_subject| cred_subject.get("image")
                                        .and_then(Value::as_str)
                                        .map(|image| async {
                                            let url = image.parse().inspect_err(|err| warn!("Failed to parse logo URI: {:#?}", err)).ok()?;
                                            let _ = download_asset(url, &hash(image)).await;

                                            Some(image.to_string())
                                        })))
                                    .await
                                    .flatten();

                                // todo: this actually needs to be straightened out in the frontend.
                                // Currently the frontend only defaults to the fallback icon if the logo_uri is Some but it can't fetch it, if it's None it displays nothing.
                                if logo_uri.is_none() {
                                    warn!("Failed to get logo URI from unverified jwt claims: {unverified_jwt_claims:#?}");
                                    logo_uri = Some("Fallback icon".to_string());
                                }

                                let issuance_date = unverified_jwt_claims["issuanceDate"].as_str().and_then(|date_str| {
                                        DateTime::parse_from_rfc3339(date_str)
                                            .ok()
                                            .map(|dt| dt.to_rfc3339())
                                }).unwrap_or({
                                    warn!("Failed to get rfc3999 compliant issuance date from unverified jwt claims: {unverified_jwt_claims:#?}");
                                    // todo: VC spec doesn't require issuance date, furthermore, it specifies that if none is present, validity is indefinite. 
                                    // issuanceDate doesn't equal validity (thats validFrom and validUntil), 
                                    // but it can easily be mistaken for implying validity there I chose the "indefinite" timestamp.
                                    "0000-00-00T00:00:00Z".to_string()
                                });

                                Some(
                                    LinkedVerifiableCredentialData {
                                    name,
                                    logo_uri,
                                    issuance_date,
                                    issuer_linked_domains: validated_linked_domains,
                                })
                            }
                            else {
                                warn!("Failed to get verifiable credential from unverified jwt claims: {linked_verifiable_credential:#?}");
                                None
                            }
                        }
                        else {
                            warn!("Failed to get unverified jwt claims from linked verifiable credential: {linked_verifiable_credential:#?}");
                            None
                        }
                    }
                    else {
                        warn!("Failed to convert linked verifiable credential to Json Value: {linked_verifiable_credential:#?}");
                        None
                    }
                }
            } else {
                warn!("No validated linked domains for issuer DID: {issuer_did}");
                // TODO: Should we add more fine-grained error handling? `None` here means that the domain linkage
                // validation failed or is unknown.
                None
            }
        })
        .collect::<Vec<_>>()
        .await
}

fn extract_url_from_did_web(did_web: &str) -> Option<Url> {
    if let Some(did) = did_web.strip_prefix("did:web:") {
        let url_str = if let Some(index_colon) = did.find(':') {
            &did[..index_colon]
        } else {
            did
        };

        if let Ok(url) = Url::parse(&format!("https://{}", url_str)) {
            return Some(url);
        }
    }
    None
}

/// Returns a Vec of successfully validated issuer linked domains.
async fn get_validated_linked_domains(issuer_linked_domains: &[Url], issuer_did: &str) -> Vec<Url> {
    FuturesUnordered::from_iter(issuer_linked_domains.iter().map(|issuer_linked_domain| async move {
        let validation_status = validate_domain_linkage(issuer_linked_domain.clone(), issuer_did)
            .await
            .status;

        if validation_status == ValidationStatus::Success {
            info!("Successfully validated domain linkage for issuer linked domain: {issuer_linked_domain}");
            Some(issuer_linked_domain.clone())
        } else {
            warn!("Failed to validate domain linkage for issuer linked domain: {issuer_linked_domain}");
            None
        }
    }))
    .filter_map(|result| async move { result })
    .collect()
    .await
}

/// This function uses the linked verifiable credential to resolve the issuer document.
async fn get_issuer_document(resolver: &Resolver, linked_verifiable_credential: &Jwt) -> Option<CoreDocument> {
    let decoder = Decoder::new();

    // Decode the linked verifiable credential.
    let decoded_linked_verifiable_credential = decoder
        .decode_compact_serialization(linked_verifiable_credential.as_str().as_bytes(), None)
        .inspect_err(|err| warn!("Failed to decode linked verifiable credential: {:#?}", err))
        .ok()?;

    let claims: JwtClaims<Value> = serde_json::from_slice(decoded_linked_verifiable_credential.claims())
        .inspect_err(|err| warn!("Failed to parse linked verifiable credential claims: {:#?}", err))
        .ok()?;

    info!("Linked verifiable credential claims: {:#?}", claims);

    // Resolve the DID
    resolver
        .resolve(claims.iss()?)
        .await
        .inspect_err(|err| warn!("Failed to resolve issuer DID.: {:#?}", err))
        .ok()
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
                                            .inspect_err(|err| warn!("Failed to parse linked domain: {:#?}", err))
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

fn get_name(credential_subject: &Subject) -> Option<String> {
    credential_subject
        .properties
        .get("name")
        .or_else(|| credential_subject.properties.get("naam"))
        .or_else(|| credential_subject.properties.get("legal_person_name"))
        .and_then(Value::as_str)
        .map(ToString::to_string)
}

async fn get_logo_uri(credential_subject: &Subject) -> Option<String> {
    OptionFuture::from(
        credential_subject
            .properties
            .get("image")
            .and_then(Value::as_str)
            .map(|image| async {
                let _ = download_asset(
                    image
                        .parse()
                        .inspect_err(|err| warn!("Failed to parse logo URI: {:#?}", err))
                        .ok()?,
                    &hash(image),
                )
                .await;
                Some(image.to_string())
            }),
    )
    .await
    .flatten()
}

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
    }

    impl TestEntity {
        // Create a new 'Entity' with a DID Document, mock server, a domain, and a secret manager.
        async fn new() -> Self {
            engine::snapshot::try_set_encrypt_work_factor(0).unwrap();

            let mock_server = MockServer::start().await;

            let uri = mock_server.uri();
            let port = uri.split(':').last().unwrap();
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

            TestEntity {
                mock_server,
                domain,
                did_document,
                secret_manager: Arc::new(Mutex::new(secret_manager)),
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
    }

    #[tokio::test]
    async fn validate_linked_verifiable_presentations_successfully_validates_multiple_presentations() {
        let mut holder = TestEntity::new().await;

        let mut issuer_a = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer A mock server.
        issuer_a
            .add_well_known_did_configuration_json("linked-domain", &[issuer_a.domain.clone().into()])
            .await;
        issuer_a.add_well_known_did_json().await;

        let mut issuer_b = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer B mock server.
        issuer_b
            .add_well_known_did_configuration_json("linked-domain", &[issuer_b.domain.clone().into()])
            .await;
        issuer_b.add_well_known_did_json().await;

        let verifiable_credential_jwt = issuer_a
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                "https://webshop.com/logo.jpg".parse().unwrap(),
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

        let verifiable_credential_jwt_2 = issuer_b
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                "https://webshop.com/logo.jpg".parse().unwrap(),
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
            validate_linked_verifiable_presentations(holder.did_document.id().to_string().as_ref()).await,
            vec![
                vec![LinkedVerifiableCredentialData {
                    name: Some("Webshop".to_string()),
                    logo_uri: Some("https://webshop.com/logo.jpg".to_string()),
                    issuer_linked_domains: vec![issuer_a.domain.clone()],
                    ..Default::default()
                }],
                vec![LinkedVerifiableCredentialData {
                    name: Some("Webshop".to_string()),
                    logo_uri: Some("https://webshop.com/logo.jpg".to_string()),
                    issuer_linked_domains: vec![issuer_b.domain.clone()],
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
            validate_linked_verifiable_presentations(holder.did_document.id().to_string().as_ref()).await,
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
    async fn get_validated_linked_credential_data_succesfully_returns_linked_verifiable_credential_data() {
        let mut issuer = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer
            .add_well_known_did_configuration_json("linked-domain", &[issuer.domain.clone().into()])
            .await;
        issuer.add_well_known_did_json().await;

        let mut holder = TestEntity::new().await;

        let verifiable_credential_jwt = issuer
            .issue_credential(
                holder.did_document.id().to_string().as_str(),
                "Webshop",
                "https://webshop.com/logo.jpg".parse().unwrap(),
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

        let resolver = Resolver::new().await;

        let linked_verifiable_presentation_url: url::Url =
            format!("{}{linked_verifiable_presentation_endpoint}", holder.domain)
                .parse()
                .unwrap();

        let validated_linked_presentation_data =
            get_validated_linked_presentation_data(&resolver, &holder.did_document, linked_verifiable_presentation_url)
                .await;

        assert_eq!(
            validated_linked_presentation_data,
            Some(vec![LinkedVerifiableCredentialData {
                name: Some("Webshop".to_string()),
                logo_uri: Some("https://webshop.com/logo.jpg".to_string()),
                issuer_linked_domains: vec![issuer.domain.clone()],
                ..Default::default()
            }])
        );
    }

    #[tokio::test]
    async fn get_validated_linked_domains_returns_only_succesfully_validated_linked_domains() {
        let mut issuer1 = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer1
            .add_well_known_did_configuration_json("linked-domain", &[issuer1.domain.clone().into()])
            .await;
        issuer1.add_well_known_did_json().await;

        // Succesfully validate the linked domain.
        assert_eq!(
            get_validated_linked_domains(
                &[issuer1.domain.clone()],
                issuer1.did_document.id().to_string().as_ref()
            )
            .await,
            vec![issuer1.domain.clone()]
        );

        // Assert that only one domain was validated.
        assert_eq!(
            get_validated_linked_domains(
                &[issuer1.domain.clone(), "http://invalid-domain.org".parse().unwrap()],
                issuer1.did_document.id().to_string().as_ref()
            )
            .await,
            vec![issuer1.domain.clone()]
        );

        let mut issuer2 = TestEntity::new().await;

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer2
            .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
            .await;
        issuer2.add_well_known_did_json().await;

        // Assert that only one domain was validated. The second domain cannot be validated because the issuer DID is different.
        assert_eq!(
            get_validated_linked_domains(
                &[issuer1.domain.clone(), issuer2.domain.clone()],
                issuer1.did_document.id().to_string().as_ref()
            )
            .await,
            vec![issuer1.domain.clone()]
        );

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server. Use the same issuer DID as
        // issuer1, but a different domain.
        let mut issuer2 = TestEntity::new().await;
        issuer2.did_document = issuer1.did_document.clone();
        issuer2.secret_manager = issuer1.secret_manager.clone();

        // Add the `/did_configuration.json` and `/did.json` endpoints to the issuer mock server.
        issuer2
            .add_well_known_did_configuration_json("linked-domain-2", &[issuer2.domain.clone().into()])
            .await;
        issuer2.add_well_known_did_json().await;

        // Assert that both domains were validated (regardless of the order).
        assert!(get_validated_linked_domains(
            &[issuer1.domain.clone(), issuer2.domain.clone()],
            issuer1.did_document.id().to_string().as_ref()
        )
        .await
        .iter()
        .all(|item| [issuer1.domain.clone(), issuer2.domain.clone()].contains(item)));
    }
}
