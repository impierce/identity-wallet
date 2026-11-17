//! DID resolution with identity-kit.

use crate::http_client::IdentityKitHttpClient;
use anyhow::{anyhow, Context as _};
use identity_kit::{
    did::{Did, DidDocument, DidUrl},
    jose::Jwk,
    methods::{key::KeyFormat, DidJwk, DidKey, DidWeb},
};
use std::sync::Arc;

/// Where to look for a verification method in a DID document.
enum MethodScope {
    /// Under `verificationMethod`, or embedded in a verification relationship.
    Any,
    /// Under `verificationMethod` only.
    Listed,
}

/// Resolves `did:key`, `did:jwk` and `did:web` DIDs.
///
/// `did:key` methods carry the key as `publicKeyJwk`, as they did with `did_manager`.
#[derive(Debug)]
pub struct Resolver(Arc<identity_kit::resolver::Resolver>);

impl Resolver {
    pub fn new() -> Self {
        Self(Arc::new(
            identity_kit::resolver::Resolver::builder()
                .with(DidKey::new().with_format(KeyFormat::JsonWebKey))
                .with(DidJwk)
                .with(DidWeb::new(IdentityKitHttpClient))
                .build(),
        ))
    }

    /// The identity-kit resolver, which also resolves the keys of DID URLs for identity-kit's validators.
    pub fn inner(&self) -> &identity_kit::resolver::Resolver {
        &self.0
    }

    /// The identity-kit resolver, shared, for a key resolver that combines it with others.
    pub fn shared(&self) -> Arc<identity_kit::resolver::Resolver> {
        self.0.clone()
    }

    /// The DID document of `did`.
    pub async fn resolve(&self, did: &str) -> anyhow::Result<DidDocument> {
        let did = Did::parse(did).context("Invalid DID")?;
        Ok(self.0.resolve_document(&did).await?)
    }

    /// The public key of the verification method `did_url`, listed under `verificationMethod` or embedded in a
    /// verification relationship, from its `publicKeyJwk` or its `publicKeyMultibase`.
    pub async fn public_key(&self, did_url: &str) -> anyhow::Result<Jwk> {
        self.method_public_key(did_url, MethodScope::Any).await
    }

    /// The public key of the verification method `did_url` listed under `verificationMethod`. A method embedded only
    /// in a relationship, such as `authentication`, is not found. Signatures on credentials, status lists and requests
    /// are verified with this key, as with IOTA's `MethodScope::VerificationMethod` before.
    pub async fn listed_public_key(&self, did_url: &str) -> anyhow::Result<Jwk> {
        self.method_public_key(did_url, MethodScope::Listed).await
    }

    async fn method_public_key(&self, did_url: &str, scope: MethodScope) -> anyhow::Result<Jwk> {
        let did_url = DidUrl::parse(did_url).context("Invalid DID URL")?;
        let document = self.0.resolve_document(did_url.did()).await?;
        let method = match scope {
            MethodScope::Any => document.verification_method(&did_url),
            MethodScope::Listed => document.verification_method.iter().find(|method| method.id == did_url),
        }
        .ok_or_else(|| anyhow!("No verification method `{did_url}`"))?;

        match (method.public_key_jwk(), method.public_key_multibase()) {
            (Some(jwk), _) => Ok(jwk.public()),
            (None, Some(multikey)) => Ok(Jwk::from_multikey(multikey)?),
            (None, None) => Err(anyhow!("The verification method `{did_url}` has no public key")),
        }
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
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    const DID_KEY: &str = "did:key:z6MkhbvNd9m3jLBCv7WhPVM2WxMcCEECEbFEMn7Cy8wCkSMx";

    #[tokio::test]
    async fn resolves_a_did_key_with_a_public_key_jwk() {
        let document = Resolver::new().resolve(DID_KEY).await.unwrap();

        assert_eq!(document.id.as_str(), DID_KEY);
        let method = document.verification_method.first().unwrap();
        assert_eq!(
            method.id.to_string(),
            format!("{DID_KEY}#{}", &DID_KEY["did:key:".len()..])
        );
        assert_eq!(method.public_key_jwk().unwrap().alg(), None);
        assert_eq!(method.public_key_jwk().unwrap().kty(), "OKP");

        let key = Resolver::new().public_key(method.id.as_str()).await.unwrap();
        assert_eq!(key.crv(), Some("Ed25519"));
    }

    #[tokio::test]
    async fn resolves_a_did_jwk() {
        let did = "did:jwk:eyJhbGciOiJFZERTQSIsImNydiI6IkVkMjU1MTkiLCJraWQiOiJwZVFoTEdqcUd2b1hEUlNIZ2E0RWpvU3NKLWFuaUlrUXdmOUdTOFN5b3o4Iiwia3R5IjoiT0tQIiwieCI6Ikxzc0xoemNOU0VwTXFiWm9Mc1VPejVXQ0t6UjRuYno3WVVOcE1HcmR0Q3MifQ";
        let document = Resolver::new().resolve(did).await.unwrap();

        let method = document.verification_method.first().unwrap();
        assert_eq!(method.id.to_string(), format!("{did}#0"));
        assert_eq!(method.public_key_jwk().unwrap().alg(), Some("EdDSA"));
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

        assert_eq!(document.id.as_str(), did);
        assert_eq!(document.verification_method.len(), 1);
    }

    /// A key embedded only in `authentication` is the holder's, so it is found for holder keys but not for signatures
    /// on credentials, which use the methods listed under `verificationMethod`.
    #[tokio::test]
    async fn a_method_embedded_in_a_relationship_is_not_a_listed_method() {
        let mock_server = MockServer::start().await;
        let did = format!("did:web:localhost%3A{}", mock_server.address().port());
        let jwk = json!({ "kty": "OKP", "crv": "Ed25519", "x": "G80iskrv_nE69qbGLSpeOHJgmV4MKIzsy5l5iT6pCww" });
        Mock::given(method("GET"))
            .and(path("/.well-known/did.json"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "@context": ["https://www.w3.org/ns/did/v1", "https://w3id.org/security/suites/jws-2020/v1"],
                "id": did,
                "verificationMethod": [{
                    "id": format!("{did}#listed"),
                    "type": "JsonWebKey2020",
                    "controller": did,
                    "publicKeyJwk": jwk
                }],
                "authentication": [{
                    "id": format!("{did}#embedded"),
                    "type": "JsonWebKey2020",
                    "controller": did,
                    "publicKeyJwk": jwk
                }]
            })))
            .mount(&mock_server)
            .await;
        let resolver = Resolver::new();

        assert!(resolver.listed_public_key(&format!("{did}#listed")).await.is_ok());
        assert!(resolver.public_key(&format!("{did}#embedded")).await.is_ok());
        assert!(resolver.listed_public_key(&format!("{did}#embedded")).await.is_err());
    }

    #[tokio::test]
    async fn rejects_unsupported_methods() {
        let did = "did:iota:0xe4edef97da1257e83cbeb49159cfdd2da6ac971ac447f233f8439cf29376ebfe";
        assert!(Resolver::new().resolve(did).await.is_err());
    }
}
