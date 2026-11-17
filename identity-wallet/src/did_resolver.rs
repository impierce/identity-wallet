//! DID resolution with identity-kit.

use crate::http_client::DidWebHttpClient;
use anyhow::Context as _;
use identity_iota::{core::FromJson as _, document::CoreDocument};
use identity_kit::{
    did::Did,
    methods::{key::KeyFormat, DidJwk, DidKey, DidWeb},
};

/// Resolves `did:key`, `did:jwk` and `did:web` DIDs.
///
/// Documents are returned as IOTA `CoreDocument`s for the credential, domain linkage and linked VP validators, which
/// still use the IOTA types. `did:key` methods carry the key as `publicKeyJwk`, as they did with `did_manager`, since
/// those validators only verify with a JWK.
#[derive(Debug)]
pub struct Resolver(identity_kit::resolver::Resolver);

impl Resolver {
    pub fn new() -> Self {
        Self(
            identity_kit::resolver::Resolver::builder()
                .with(DidKey::new().with_format(KeyFormat::JsonWebKey))
                .with(DidJwk)
                .with(DidWeb::new(DidWebHttpClient))
                .build(),
        )
    }

    /// The DID document of `did`.
    pub async fn resolve(&self, did: &str) -> anyhow::Result<CoreDocument> {
        let did = Did::parse(did).context("Invalid DID")?;
        let document = self.0.resolve_document(&did).await?;
        CoreDocument::from_json_value(serde_json::to_value(document)?).context("Unsupported DID document")
    }
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use identity_iota::did::DID as _;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const DID_KEY: &str = "did:key:z6MkhbvNd9m3jLBCv7WhPVM2WxMcCEECEbFEMn7Cy8wCkSMx";

    #[tokio::test]
    async fn resolves_a_did_key_with_a_public_key_jwk() {
        let document = Resolver::new().resolve(DID_KEY).await.unwrap();

        assert_eq!(document.id().as_str(), DID_KEY);
        let method = document.verification_method().first().unwrap();
        assert_eq!(
            method.id().to_string(),
            format!("{DID_KEY}#{}", &DID_KEY["did:key:".len()..])
        );
        assert_eq!(method.data().public_key_jwk().unwrap().alg(), None);
        assert_eq!(method.data().public_key_jwk().unwrap().kty().name(), "OKP");
    }

    #[tokio::test]
    async fn resolves_a_did_jwk() {
        let did = "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJraWQiOiJwZVFoTEdqcUd2b1hEUlNIZ2E0RWpvU3NKLWFuaUlrUXdmOUdTOFN5b3o4Iiwia3R5IjoiT0tQIiwieCI6Ikxzc0xoemNOU0VwTXFiWm9Mc1VPejVXQ0t6UjRuYno3WVVOcE1HcmR0Q3MifQ";
        let document = Resolver::new().resolve(did).await.unwrap();

        let method = document.verification_method().first().unwrap();
        assert_eq!(method.id().to_string(), format!("{did}#0"));
        assert_eq!(method.data().public_key_jwk().unwrap().alg(), Some("EdDSA"));
    }

    #[tokio::test]
    async fn resolves_a_did_web_of_localhost_over_http_in_debug_builds() {
        let mock_server = MockServer::start().await;
        let did = format!("did:web:localhost%3A{}", mock_server.address().port());
        Mock::given(method("GET"))
            .and(path("/.well-known/did.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "@context": ["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/jws-2020/v1"],
                "id": did,
                "verificationMethod": [{
                    "id": format!("{did}#key-0"),
                    "type": "JsonWebKey2020",
                    "controller": did,
                    "publicKeyJwk": { "kty": "OKP", "crv": "Ed25519", "x": "G80iskrv_nE69qbGLSpeOHJgmV4MKIzsy5l5iT6pCww" }
                }],
                "assertionMethod": [format!("{did}#key-0")]
            })))
            .mount(&mock_server)
            .await;

        let document = Resolver::new().resolve(&did).await.unwrap();

        assert_eq!(document.id().as_str(), did);
        assert_eq!(document.verification_method().len(), 1);
    }

    #[tokio::test]
    async fn rejects_unsupported_methods() {
        let did = "did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe";
        assert!(Resolver::new().resolve(did).await.is_err());
    }
}
