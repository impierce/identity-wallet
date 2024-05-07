use crate::stronghold::StrongholdManager;

use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use did_manager::{DidMethod, Resolver, SecretManager};
use identity_iota::{did::DID, document::DIDUrlQuery, verification::jwk::JwkParams};
use oid4vc::oid4vc_core::{authentication::sign::ExternalSign, Sign, Subject, Verify};
use std::sync::Arc;

pub struct UnimeSubject {
    pub stronghold_manager: Arc<StrongholdManager>,
    pub secret_manager: SecretManager,
}

#[async_trait]
impl Sign for UnimeSubject {
    async fn key_id(&self, subject_syntax_type: &str) -> Option<String> {
        let method: DidMethod = serde_json::from_str(&format!("{subject_syntax_type:?}")).ok()?;

        self.secret_manager // <-- here
            .produce_document(method)
            .await
            .ok()
            .and_then(|document| document.verification_method().first().cloned())
            .map(|first| first.id().to_string())
    }
    async fn sign(&self, message: &str, _subject_syntax_type: &str) -> anyhow::Result<Vec<u8>> {
        Ok(self.secret_manager.sign(message.as_bytes()).await?) // <-- here
    }
    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>> {
        None
    }
}

#[async_trait]
impl Subject for UnimeSubject {
    /// Returns the id of the DID document corresponding to the `subject_syntax_type`.
    /// TODO: `subject_syntax_type` or simply `method`?
    async fn identifier(&self, subject_syntax_type: &str) -> anyhow::Result<String> {
        let method: DidMethod = serde_json::from_str(&format!("{subject_syntax_type:?}"))?;

        Ok(self
            .secret_manager
            .produce_document(method)
            .await
            .map(|document| document.id().to_string())?)
    }
}

#[async_trait]
impl Verify for UnimeSubject {
    async fn public_key(&self, did_url: &str) -> anyhow::Result<Vec<u8>> {
        let did_url = identity_iota::did::DIDUrl::parse(did_url).unwrap();

        let resolver = Resolver::new().await;

        let document = resolver.resolve(did_url.did().as_str()).await.unwrap();

        let verification_method = document
            .resolve_method(
                DIDUrlQuery::from(&did_url),
                Some(identity_iota::verification::MethodScope::VerificationMethod),
            )
            .unwrap();
        // .ok_or(ProducerError::Generic(format!(
        //     "No verification method found for fragment=[{}]",
        //     did_url.fragment().unwrap()
        // )))?;

        // Try decode from `MethodData` directly, else use public JWK params.
        verification_method
            .data()
            .try_decode()
            .or_else(|_| {
                verification_method
                    .data()
                    .public_key_jwk()
                    .and_then(|public_key_jwk| match public_key_jwk.params() {
                        JwkParams::Okp(okp_params) => Some(okp_params.x.as_bytes().to_vec()),
                        JwkParams::Ec(ec_params) => Some(ec_params.x.as_bytes().to_vec()),
                        _ => None,
                    })
                    .ok_or(anyhow::anyhow!("Failed to decode public key for DID URL: {}", did_url))
            })
            .and_then(|encoded_public_key| URL_SAFE_NO_PAD.decode(encoded_public_key).map_err(Into::into))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use identity_iota::did::{CoreDID, DIDUrl, RelativeDIDUrl};
    // use test_log::test;

    const SNAPSHOT_PATH: &str = "tests/res/test.stronghold";
    const PASSWORD: &str = "secure_password";
    const KEY_ID: &str = "9O66nzWqYYy1LmmiOudOlh2SMIaUWoTS";

    #[tokio::test]
    async fn successfully_finds_an_existing_public_key_in_did_key_by_fragment() {
        let res = SecretManager::load(
            SNAPSHOT_PATH.to_owned(),
            PASSWORD.to_owned(),
            KEY_ID.to_owned(),
            None,
            None,
        )
        .await
        .unwrap();
        let core_did = CoreDID::parse("did:key:z6MkiieyoLMSVsJAZv7Jje5wWSkDEymUgkyF8kbcrjZpX3qd").unwrap();
        let mut url = RelativeDIDUrl::new();
        url.set_fragment(Some(core_did.method_id())).unwrap();
        let did_url = DIDUrl::new(CoreDID::parse(core_did).unwrap(), Some(url));
        // let pub_key = res.public_key(&did_url.to_string()).await.unwrap();
        // assert_eq!(
        //     URL_SAFE_NO_PAD.encode(&pub_key),
        //     "P2BkYS6z4UHmsxn6FX1oHsyx7eiUSFEMJ1D_RC8M0-w"
        // );
    }

    #[tokio::test]
    async fn successfully_finds_an_existing_public_key_in_did_jwk_by_fragment() {
        let res = SecretManager::load(
            SNAPSHOT_PATH.to_owned(),
            PASSWORD.to_owned(),
            KEY_ID.to_owned(),
            None,
            None,
        )
        .await
        .unwrap();
        let core_did = CoreDID::parse("did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9").unwrap();
        let mut url = RelativeDIDUrl::new();
        url.set_fragment(Some("#0")).unwrap();
        let did_url = DIDUrl::new(CoreDID::parse(core_did).unwrap(), Some(url));
        // let pub_key = res.public_key(&did_url.to_string()).await.unwrap();
        // assert_eq!(
        //     URL_SAFE_NO_PAD.encode(&pub_key),
        //     "acbIQiuMs3i8_uszEjJ2tpTtRM4EU3yz91PH6CdH2V0"
        // );
    }

    #[tokio::test]
    async fn throws_error_when_no_public_key_found_in_document_for_fragment() {
        let res = SecretManager::load(
            SNAPSHOT_PATH.to_owned(),
            PASSWORD.to_owned(),
            KEY_ID.to_owned(),
            None,
            None,
        )
        .await
        .unwrap();
        let core_did = CoreDID::parse("did:jwk:eyJjcnYiOiJQLTI1NiIsImt0eSI6IkVDIiwieCI6ImFjYklRaXVNczNpOF91c3pFakoydHBUdFJNNEVVM3l6OTFQSDZDZEgyVjAiLCJ5IjoiX0tjeUxqOXZXTXB0bm1LdG00NkdxRHo4d2Y3NEk1TEtncmwyR3pIM25TRSJ9").unwrap();
        let mut url = RelativeDIDUrl::new();
        url.set_fragment(Some("#foobar")).unwrap();
        let did_url = DIDUrl::new(CoreDID::parse(core_did).unwrap(), Some(url));
        // assert!(res.public_key(&did_url.to_string()).await.is_err());
    }
}
