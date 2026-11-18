use crate::did_resolver::Resolver;
use crate::state::SUPPORTED_DID_METHODS;
use crate::stronghold::{StrongholdManager, ED25519_KEY_ID, ES256_KEY_ID};
use anyhow::anyhow;
use async_trait::async_trait;
use identity_kit::{
    did::DidMethod,
    identity::{Identity, IdentityKind, KeyStoreIdentity},
    jose::{Jwk, JwsAlgorithm},
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
        self.resolver()
            .await
            .public_key(did_url)
            .await
            .map_err(|err| anyhow!("Failed to resolve the public key of `{did_url}`: {err:#}"))
    }

    /// The algorithm the key of `did_url` signs with: the `alg` of its JWK, or else the one its key type implies.
    pub async fn signing_algorithm(&self, did_url: &str) -> anyhow::Result<Algorithm> {
        let jwk = self.resolve_public_key(did_url).await?;
        match (jwk.alg(), jwk.kty(), jwk.crv()) {
            (Some(alg), _, _) => alg
                .parse()
                .map_err(|_| anyhow!("Unsupported algorithm `{alg}` for `{did_url}`")),
            (None, "OKP", Some("Ed25519")) => Ok(Algorithm::EdDSA),
            (None, "EC", Some("P-256")) => Ok(Algorithm::ES256),
            (None, kty, crv) => Err(anyhow!("Unsupported key type `{kty}` ({crv:?}) for `{did_url}`")),
        }
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
    /// The public key of `did_url` in the form `jsonwebtoken` verifies with: an Ed25519 key as its 32 bytes, a P-256
    /// key as its uncompressed SEC1 point.
    async fn public_key(&self, did_url: &str) -> anyhow::Result<Vec<u8>> {
        // Only a method listed under `verificationMethod`, as with IOTA's `MethodScope::VerificationMethod` before.
        let jwk = self
            .resolver()
            .await
            .listed_public_key(did_url)
            .await
            .map_err(|err| anyhow!("Failed to resolve the public key of `{did_url}`: {err:#}"))?;
        match (jwk.kty(), jwk.crv()) {
            ("OKP", Some("Ed25519")) => Ok(jwk.ed25519_public_bytes()?.to_vec()),
            ("EC", Some("P-256")) => Ok(jwk.p256_public_bytes()?.to_vec()),
            (kty, crv) => Err(anyhow!("Unsupported key type `{kty}` ({crv:?}) for `{did_url}`")),
        }
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
