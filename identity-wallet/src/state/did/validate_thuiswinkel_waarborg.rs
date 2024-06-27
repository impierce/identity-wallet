use std::str::FromStr;

use crate::{
    persistence::{download_asset, hash},
    state::{
        core_utils::helpers::get_unverified_jwt_claims,
        did::validate_domain_linkage::{ValidationResult, ValidationStatus},
    },
};
use did_manager::Resolver;
use identity_iota::core::ToJson;
use log::info;

pub async fn validate_thuiswinkel_waarborg(did: &str) -> ValidationResult {
    let resolver = Resolver::new().await;

    info!("Validating Thuiswinkel Waarborg");
    info!("DID: {}", did);

    // Resolve the Document from the DID.
    let document = match resolver.resolve(did).await {
        Ok(document) => document,
        Err(e) => {
            return ValidationResult {
                status: ValidationStatus::Unknown,
                message: Some(e.to_string()),
                ..Default::default()
            };
        }
    };

    info!("Document: {:?}", document);

    // Extract the URL of the Linked Verifiable Presentation from the Docoment.
    let linked_verifiable_presentation_url = match document
        .service()
        .iter()
        .find_map(|service| {
            service
                .type_()
                .contains("LinkedVerifiablePresentation")
                .then(|| service.service_endpoint())
        })
        .and_then(|service_endpoint| service_endpoint.to_json_value().ok())
        .and_then(|service_endpoint| service_endpoint.get("origins").cloned())
        .and_then(|origins| {
            origins.as_array().and_then(|origins| {
                origins
                    .first()
                    .and_then(|origin| origin.as_str().map(url::Url::from_str))
            })
        }) {
        Some(Ok(linked_verifiable_presentation_url)) => linked_verifiable_presentation_url,
        _ => {
            return ValidationResult {
                status: ValidationStatus::Unknown,
                ..Default::default()
            }
        }
    };

    info!(
        "Linked Verifiable Presentation URL: {}",
        linked_verifiable_presentation_url
    );

    // Fetch the actual Linked Verifiable Presentation from the service endpoint.
    let linked_verifiable_presentation_result =
        fetch_linked_verifiable_presentation(linked_verifiable_presentation_url).await;

    let linked_verifiable_presentation = match linked_verifiable_presentation_result {
        Ok(linked_verifiable_presentation) => linked_verifiable_presentation,
        Err(e) => {
            return ValidationResult {
                status: ValidationStatus::Unknown,
                message: Some(e),
                ..Default::default()
            }
        }
    };

    info!("Linked Verifiable Presentation: {}", linked_verifiable_presentation);

    // Extract the `name` and `thuiswinkel_waarborg_image` from the Linked Verifiable Presentation to be displayed in
    // the frontend.
    let (name, thuiswinkel_waarborg_image) =
        match get_unverified_jwt_claims(&serde_json::json!(linked_verifiable_presentation))
            .get("vp")
            .and_then(|vp| {
                vp.get("verifiableCredential")
                    .and_then(|verifiable_credentials| verifiable_credentials.as_array())
            })
            .and_then(|verifiable_credential| verifiable_credential.first().cloned())
            .map(|verifiable_credential| get_unverified_jwt_claims(&verifiable_credential))
            .and_then(|verifiable_credential| {
                verifiable_credential.get("vc").and_then(|vc| {
                    vc.get("credentialSubject").and_then(|credential_subject| {
                        credential_subject
                            .get("name")
                            .and_then(serde_json::Value::as_str)
                            .map(ToString::to_string)
                            .and_then(|name| {
                                Some((
                                    Some(name),
                                    credential_subject
                                        .get("thuiswinkel_waarborg_image")
                                        .and_then(serde_json::Value::as_str)
                                        .map(url::Url::from_str),
                                ))
                            })
                    })
                })
            }) {
            Some(display_properties) => {
                if let Some(Ok(thuiswinkel_waarborg_image)) = display_properties.1 {
                    let _ = download_asset(
                        thuiswinkel_waarborg_image.clone(),
                        &hash(thuiswinkel_waarborg_image.as_str()),
                    )
                    .await;
                    (display_properties.0, Some(thuiswinkel_waarborg_image))
                } else {
                    (display_properties.0, None)
                }
            }
            None => {
                return ValidationResult {
                    status: ValidationStatus::Unknown,
                    ..Default::default()
                }
            }
        };

    info!("Thuiswinkel Waarborg Name: {:?}", name);
    info!("Thuiswinkel Waarborg Image: {:?}", thuiswinkel_waarborg_image);

    ValidationResult {
        status: ValidationStatus::Success,
        name,
        logo_uri: thuiswinkel_waarborg_image,
        ..Default::default()
    }
}

async fn fetch_linked_verifiable_presentation(url: url::Url) -> Result<String, String> {
    // 1. Fetch the resource
    let response = reqwest::get(url).await.map_err(|e| e.to_string())?;

    // 2. Return the Linked Verifiable Presentation (as Jwt)
    response.text().await.map_err(|e| e.to_string())
}
