use did_manager::Resolver;
use identity_credential::domain_linkage::{DomainLinkageConfiguration, JwtDomainLinkageValidator};
use identity_eddsa_verifier::EdDSAJwsVerifier;
use identity_iota::{
    core::{FromJson, ToJson},
    credential::JwtCredentialValidationOptions,
};
use log::info;
use std::str::FromStr;

// https://wiki.iota.org/identity.rs/how-tos/domain-linkage/create-and-verify/#verifying-a-did-and-domain-linkage
pub async fn validate_domain_linkage(url: url::Url, did: &str) -> bool {
    let scheme_host = format!("{}://{}", url.scheme(), url.host_str().unwrap());
    let mut url = identity_iota::core::Url::from_str(&scheme_host).unwrap();

    info!("url: {:?}", url);
    info!("did: {:?}", did);

    // Option 1: identity.rs only supports encoded jwt strings in the `linked_dids` array
    let domain_linkage_configuration = DomainLinkageConfiguration::fetch_configuration(url.clone()).await;

    // Option 2: use reqwest directly to parse the `linked_dids` exhaustively
    url.set_path(".well-known/did-configuration.json");
    info!("url: {:?}", url.to_string());
    let mut did_configuration = reqwest::get(url.to_string())
        .await
        .inspect(|r| info!("{:?}", r))
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
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
        return false;
    }

    let validator = JwtDomainLinkageValidator::with_signature_verifier(EdDSAJwsVerifier::default());

    let resolver = Resolver::new().await;
    let document = resolver.resolve(did).await.unwrap();

    info!("document: {:?}", document);

    let res = validator.validate_linkage(
        &document,
        &domain_linkage_configuration.expect("Error should have been handled already"),
        &url,
        &JwtCredentialValidationOptions::default(),
    );

    info!("res: {:?}", res);

    res.is_ok()
}
