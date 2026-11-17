use crate::did_resolver::Resolver;
use crate::state::SUPPORTED_DID_METHODS;
use crate::stronghold::{StrongholdManager, ED25519_KEY_ID, ES256_KEY_ID};
use anyhow::anyhow;
use async_trait::async_trait;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use identity_iota::verification::jwk::Jwk;
use identity_iota::{did::DID, document::DIDUrlQuery, verification::jwk::JwkParams};
use identity_kit::{
    did::DidMethod,
    identity::{Identity, IdentityKind, KeyStoreIdentity},
    jose::JwsAlgorithm,
    keys::KeyId,
    methods::{DidJwk, DidKey},
};
use jsonwebtoken::Algorithm;
use log::debug;
use oid4vc_core::{authentication::sign::ExternalSign, Sign, Verify};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// A `Subject` implements functions required for signatures and verification.
/// In UniMe, it serves as the "binding link" between the protocol libraries (OID4VC) and the wallet's keys, which
/// identity-kit reads from the Stronghold (see [`crate::stronghold::ED25519_KEY_ID`]).
#[derive(Debug)]
pub struct Subject {
    pub stronghold_manager: Arc<StrongholdManager>,
    identity: KeyStoreIdentity,
    pub resolver: OnceCell<Arc<Resolver>>,
}

impl Subject {
    /// A subject for the keys in `stronghold_manager`, with a `did:jwk` and a `did:key` for each of them.
    pub async fn new(stronghold_manager: Arc<StrongholdManager>) -> anyhow::Result<Self> {
        let identity = KeyStoreIdentity::new(stronghold_manager.clone())
            .with_producer(DidJwk)
            .with_producer(DidKey::default());

        for method in SUPPORTED_DID_METHODS {
            let kind = identity_kind(method)?;
            for (alg, key) in [
                (JwsAlgorithm::EdDSA, ED25519_KEY_ID),
                (JwsAlgorithm::ES256, ES256_KEY_ID),
            ] {
                identity.bind(kind.clone(), alg, KeyId::new(key)).await?;
            }
        }

        Ok(Self {
            stronghold_manager,
            identity,
            resolver: OnceCell::new(),
        })
    }

    /// Asynchronously gets a reference to the initialized resolver.
    ///
    /// The first time this is called, it will perform the async initialization.
    /// Subsequent calls will return the already-initialized instance instantly.
    pub async fn resolver(&self) -> Arc<Resolver> {
        self.resolver.get_or_init(Self::initialize_resolver).await.clone()
    }

    /// The private async function that contains the actual initialization logic.
    async fn initialize_resolver() -> Arc<Resolver> {
        debug!("Initializing resolver for Subject");
        let resolver = Resolver::new();
        debug!("Resolver initialized");
        Arc::new(resolver)
    }

    // TODO(ssi-agent): Duplicate of ssi-agent `Subject::resolve_public_key`.
    // Replace this local implementation with the original implementation from `ssi-agent`:
    // https://github.com/impierce/ssi-agent/blob/beta/agent_secret_manager/src/subject.rs
    /// Resolves the public key for a given DID URL: a method listed under `verificationMethod` or embedded in a
    /// verification relationship.
    pub async fn resolve_public_key(&self, did_url: &str) -> anyhow::Result<Jwk> {
        self.resolve_public_key_in(did_url, None).await
    }

    /// Resolves the public key of the method `did_url` listed under `verificationMethod`, the scope
    /// [`Verify::public_key`] verifies signatures with. A method embedded only in a relationship, such as
    /// `authentication`, is not found.
    pub async fn resolve_listed_public_key(&self, did_url: &str) -> anyhow::Result<Jwk> {
        self.resolve_public_key_in(
            did_url,
            Some(identity_iota::verification::MethodScope::VerificationMethod),
        )
        .await
    }

    async fn resolve_public_key_in(
        &self,
        did_url: &str,
        scope: Option<identity_iota::verification::MethodScope>,
    ) -> anyhow::Result<Jwk> {
        let did_url =
            identity_iota::did::DIDUrl::parse(did_url).map_err(|err| anyhow!("Failed to parse DID URL: {err}"))?;

        let document = self
            .resolver()
            .await
            .resolve(did_url.did().as_str())
            .await
            .map_err(|err| anyhow!("Failed to resolve DID Document for DID: `{did_url}`, error: {err}"))?;

        let verification_method = document
            .resolve_method(DIDUrlQuery::from(&did_url), scope)
            .ok_or(anyhow!(
                "Failed to resolve verification method for DID URL: `{did_url}`"
            ))?;

        verification_method
            .data()
            .public_key_jwk()
            .ok_or_else(|| anyhow!("Failed to resolve public key for DID URL: `{did_url}`"))
            .cloned()
    }
}

#[async_trait]
impl Sign for Subject {
    async fn key_id(&self, subject_syntax_type: &str, algorithm: Algorithm) -> Option<String> {
        let signer = self
            .identity
            .signer(
                &identity_kind(subject_syntax_type).ok()?,
                jws_algorithm(algorithm).ok()?,
            )
            .await
            .ok()?;
        signer.kid().map(str::to_owned)
    }

    async fn sign(&self, message: &str, subject_syntax_type: &str, algorithm: Algorithm) -> anyhow::Result<Vec<u8>> {
        let signer = self
            .identity
            .signer(&identity_kind(subject_syntax_type)?, jws_algorithm(algorithm)?)
            .await?;
        Ok(signer.sign(message.as_bytes()).await?)
    }

    fn external_signer(&self) -> Option<Arc<dyn ExternalSign>> {
        None
    }
}

#[async_trait]
impl oid4vc_core::Subject for Subject {
    async fn identifier(&self, subject_syntax_type: &str, algorithm: Algorithm) -> anyhow::Result<String> {
        Ok(self
            .identity
            .identifier(&identity_kind(subject_syntax_type)?, jws_algorithm(algorithm)?)
            .await?
            .to_string())
    }
}

#[async_trait]
impl Verify for Subject {
    async fn public_key(&self, did_url: &str) -> anyhow::Result<Vec<u8>> {
        let did_url = identity_iota::did::DIDUrl::parse(did_url)?;

        let document = self.resolver().await.resolve(did_url.did().as_str()).await?;

        let verification_method = document
            .resolve_method(
                DIDUrlQuery::from(&did_url),
                Some(identity_iota::verification::MethodScope::VerificationMethod),
            )
            .ok_or(anyhow::anyhow!(
                "Failed to resolve verification method for DID URL: {did_url}"
            ))?;

        // Try decode from `MethodData` directly, else use public JWK params.
        verification_method.data().try_decode().or_else(|_| {
            verification_method
                .data()
                .public_key_jwk()
                .and_then(|public_key_jwk| match public_key_jwk.params() {
                    JwkParams::Okp(okp_params) => URL_SAFE_NO_PAD.decode(&okp_params.x).ok(),
                    JwkParams::Ec(ec_params) => {
                        let x_bytes = URL_SAFE_NO_PAD.decode(&ec_params.x).ok()?;
                        let y_bytes = URL_SAFE_NO_PAD.decode(&ec_params.y).ok()?;

                        let encoded_point = p256::EncodedPoint::from_affine_coordinates(
                            p256::FieldBytes::from_slice(&x_bytes),
                            p256::FieldBytes::from_slice(&y_bytes),
                            false, // false for uncompressed point
                        );

                        let verifying_key = p256::ecdsa::VerifyingKey::from_encoded_point(&encoded_point)
                            .expect("Failed to create verifying key from encoded point");

                        Some(verifying_key.to_encoded_point(false).as_bytes().to_vec())
                    }
                    _ => None,
                })
                .ok_or(anyhow::anyhow!("Failed to decode public key for DID URL: {did_url}"))
        })
    }
}

/// Loads the `Subject` for the keys in `stronghold_manager`.
pub async fn subject(stronghold_manager: Arc<StrongholdManager>) -> anyhow::Result<Arc<Subject>> {
    Ok(Arc::new(Subject::new(stronghold_manager).await?))
}

/// The kind of identifier for an OID4VC subject syntax type such as `did:jwk`.
fn identity_kind(subject_syntax_type: &str) -> anyhow::Result<IdentityKind> {
    Ok(IdentityKind::Did(DidMethod::parse(subject_syntax_type).map_err(
        |err| anyhow!("Unsupported subject syntax type `{subject_syntax_type}`: {err}"),
    )?))
}

/// The identity-kit algorithm for one of the wallet's [`SUPPORTED_SIGNING_ALGORITHMS`](crate::state::SUPPORTED_SIGNING_ALGORITHMS).
fn jws_algorithm(algorithm: Algorithm) -> anyhow::Result<JwsAlgorithm> {
    match algorithm {
        Algorithm::EdDSA => Ok(JwsAlgorithm::EdDSA),
        Algorithm::ES256 => Ok(JwsAlgorithm::ES256),
        unsupported => Err(anyhow!("Unsupported signing algorithm: {unsupported:?}")),
    }
}
