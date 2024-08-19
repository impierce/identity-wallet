use crate::{state::core_utils::helpers::EcodedPublicKey, stronghold::StrongholdManager};

use async_trait::async_trait;
use did_manager::{DidMethod, Resolver, SecretManager};
use identity_iota::{did::DID, document::DIDUrlQuery, verification::jws::JwsAlgorithm};
use jsonwebtoken::Algorithm;
use oid4vc::oid4vc_core::{authentication::sign::ExternalSign, Sign, Verify};
use std::sync::Arc;

/// A `Subject` implements functions required for signatures and verification.
/// In UniMe, it serves as the "binding link" between the protocol libraries (OID4VC) and the secret management (DID Manager).
#[derive(Debug)]
pub struct Subject {
    pub stronghold_manager: Arc<StrongholdManager>,
    pub secret_manager: SecretManager,
}

#[async_trait]
impl Sign for Subject {
    async fn key_id(&self, subject_syntax_type: &str, algorithm: Algorithm) -> Option<String> {
        let method: DidMethod = serde_json::from_str(&format!("{subject_syntax_type:?}")).ok()?;

        self.secret_manager
            .produce_document(method, None, algorithm.into_jws_algorithm())
            .await
            .ok()
            .and_then(|document| document.verification_method().first().cloned())
            .map(|first| first.id().to_string())
    }

    async fn sign(&self, message: &str, _subject_syntax_type: &str, algorithm: Algorithm) -> anyhow::Result<Vec<u8>> {
        Ok(self
            .secret_manager
            .sign(message.as_bytes(), algorithm.into_jws_algorithm())
            .await?)
    }

    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>> {
        None
    }
}

#[async_trait]
impl oid4vc::oid4vc_core::Subject for Subject {
    async fn identifier(&self, subject_syntax_type: &str, algorithm: Algorithm) -> anyhow::Result<String> {
        let method: DidMethod = serde_json::from_str(&format!("{subject_syntax_type:?}"))?;

        Ok(self
            .secret_manager
            .produce_document(method, None, algorithm.into_jws_algorithm())
            .await
            .map(|document| document.id().to_string())?)
    }
}

#[async_trait]
impl Verify for Subject {
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

        // Try decode from `MethodData` directly, else use public JWK params.
        verification_method.data().try_decode().or_else(|_| {
            verification_method
                .data()
                .public_key_jwk()
                .and_then(|public_key_jwk| public_key_jwk.encoded_public_key().ok())
                .ok_or(anyhow::anyhow!("Invalid public key"))
        })
    }
}

// Helper function: load a `Subject`
pub async fn subject(stronghold_manager: Arc<StrongholdManager>, password: String) -> Arc<Subject> {
    let client_path = crate::persistence::STRONGHOLD
        .lock()
        .unwrap()
        .to_str()
        .expect("failed to get stronghold path")
        .to_owned();

    Arc::new(Subject {
        stronghold_manager: stronghold_manager.clone(),
        secret_manager: SecretManager::load(
            client_path,
            password,
            Some("ed25519-0".to_owned()),
            Some("es256-0".to_owned()),
            Some("es256k-0".to_owned()),
            None,
            None,
        )
        .await
        .unwrap(),
    })
}

trait IntoJwsAlgorithm {
    fn into_jws_algorithm(self) -> JwsAlgorithm;
}

impl IntoJwsAlgorithm for Algorithm {
    fn into_jws_algorithm(self) -> JwsAlgorithm {
        match self {
            Algorithm::HS256 => JwsAlgorithm::HS256,
            Algorithm::HS384 => JwsAlgorithm::HS384,
            Algorithm::HS512 => JwsAlgorithm::HS512,
            Algorithm::RS256 => JwsAlgorithm::RS256,
            Algorithm::RS384 => JwsAlgorithm::RS384,
            Algorithm::RS512 => JwsAlgorithm::RS512,
            Algorithm::ES256 => JwsAlgorithm::ES256,
            Algorithm::ES384 => JwsAlgorithm::ES384,
            Algorithm::PS256 => JwsAlgorithm::PS256,
            Algorithm::PS384 => JwsAlgorithm::PS384,
            Algorithm::PS512 => JwsAlgorithm::PS512,
            Algorithm::EdDSA => JwsAlgorithm::EdDSA,
        }
    }
}
