//! The wallet's signing keys, served to identity-kit as a [`KeyStore`].
//!
//! [`StrongholdManager::create`] generates one Ed25519 and one P-256 key in the Stronghold vault, under the fixed ids
//! [`ED25519_KEY_ID`] and [`ES256_KEY_ID`]. Wallets created by earlier versions, through `did_manager`, use the same
//! vault and ids, so their keys, and the DIDs derived from them, stay the same.

use super::{StrongholdManager, STRONGHOLD_VAULT_PATH};
use async_trait::async_trait;
use identity_kit::{
    jose::{Jwk, JwsAlgorithm},
    keys::{KeyId, KeyStore, KeyStoreCapabilities, KeyStoreError},
};
use iota_stronghold::{
    procedures::{Ed25519Sign, KeyType, PublicKey, StrongholdProcedure},
    Location,
};
use stronghold_ext::{
    execute_procedure_ext,
    procs::{self, es256::Es256Procs},
};

/// The id of the wallet's Ed25519 key, which signs with `EdDSA`.
pub const ED25519_KEY_ID: &str = "ed25519-0";
/// The id of the wallet's P-256 key, which signs with `ES256`.
pub const ES256_KEY_ID: &str = "es256-0";

/// The algorithm of each key in the vault.
const KEYS: [(&str, JwsAlgorithm); 2] = [
    (ED25519_KEY_ID, JwsAlgorithm::EdDSA),
    (ES256_KEY_ID, JwsAlgorithm::ES256),
];

impl StrongholdManager {
    /// The algorithm `key` signs with, or [`KeyStoreError::NotFound`] when it is not one of the wallet's keys.
    fn key_algorithm(key: &KeyId) -> Result<JwsAlgorithm, KeyStoreError> {
        KEYS.iter()
            .find(|(id, _)| *id == key.as_str())
            .map(|(_, alg)| *alg)
            .ok_or_else(|| KeyStoreError::NotFound(key.clone()))
    }

    fn key_location(key: &KeyId) -> Location {
        Location::generic(
            STRONGHOLD_VAULT_PATH.as_bytes().to_vec(),
            key.as_str().as_bytes().to_vec(),
        )
    }

    /// The location of `key` in the vault, after checking that the vault holds it.
    fn existing_key_location(&self, key: &KeyId) -> Result<Location, KeyStoreError> {
        let location = Self::key_location(key);
        if self.client.record_exists(&location).map_err(backend)? {
            Ok(location)
        } else {
            Err(KeyStoreError::NotFound(key.clone()))
        }
    }
}

#[async_trait]
impl KeyStore for StrongholdManager {
    fn capabilities(&self) -> KeyStoreCapabilities {
        KeyStoreCapabilities::new(KEYS.map(|(_, alg)| alg))
    }

    /// The public key, with `alg` and its RFC 7638 thumbprint as `kid`, as `did_manager` returned it. Both are part of
    /// the wallet's `did:jwk`.
    async fn public_jwk(&self, key: &KeyId) -> Result<Jwk, KeyStoreError> {
        let alg = Self::key_algorithm(key)?;
        let location = self.existing_key_location(key)?;

        let jwk = match alg {
            JwsAlgorithm::EdDSA => {
                let public_key: Vec<u8> = self
                    .client
                    .execute_procedure(StrongholdProcedure::PublicKey(PublicKey {
                        ty: KeyType::Ed25519,
                        private_key: location,
                    }))
                    .map_err(backend)?
                    .into();
                Jwk::from_ed25519_public(&public_key)
            }
            _ => {
                let public_key: Vec<u8> = execute_procedure_ext(
                    &self.client,
                    Es256Procs::PublicKey(procs::es256::PublicKey { private_key: location }),
                )
                .map_err(backend)?
                .into();
                Jwk::from_p256_public(&public_key)
            }
        }
        .map_err(|error| KeyStoreError::InvalidKey {
            key: key.clone(),
            reason: "the vault holds no valid public key for it".to_owned(),
            source: Some(Box::new(error)),
        })?
        .with_alg(alg.as_str());

        let kid = jwk.thumbprint_sha256().map_err(|error| KeyStoreError::InvalidKey {
            key: key.clone(),
            reason: "its public key has no thumbprint".to_owned(),
            source: Some(Box::new(error)),
        })?;

        Ok(jwk.with_kid(kid))
    }

    async fn sign(&self, key: &KeyId, alg: JwsAlgorithm, data: &[u8]) -> Result<Vec<u8>, KeyStoreError> {
        let key_alg = Self::key_algorithm(key)?;
        if alg != key_alg {
            return Err(KeyStoreError::AlgorithmMismatch {
                key: key.clone(),
                key_alg,
                requested: alg,
            });
        }
        let location = self.existing_key_location(key)?;

        match key_alg {
            JwsAlgorithm::EdDSA => self
                .client
                .execute_procedure(Ed25519Sign {
                    private_key: location,
                    msg: data.to_vec(),
                })
                .map(|signature| signature.to_vec())
                .map_err(backend),
            _ => execute_procedure_ext(
                &self.client,
                Es256Procs::Sign(procs::es256::Sign {
                    private_key: location,
                    msg: data.to_vec(),
                }),
            )
            .map(Into::into)
            .map_err(backend),
        }
    }

    async fn exists(&self, key: &KeyId) -> Result<bool, KeyStoreError> {
        if Self::key_algorithm(key).is_err() {
            return Ok(false);
        }
        self.client.record_exists(&Self::key_location(key)).map_err(backend)
    }
}

fn backend(error: impl std::error::Error + Send + Sync + 'static) -> KeyStoreError {
    KeyStoreError::Backend(Box::new(error))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::persistence::STRONGHOLD;
    use tempfile::NamedTempFile;

    fn stronghold_manager() -> (StrongholdManager, tempfile::TempPath) {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();
        (StrongholdManager::create("sup3rSecr3t").unwrap(), path)
    }

    /// The members of the public JWKs that the wallet's `did:jwk`s encode, see [`StrongholdManager::public_jwk`].
    #[tokio::test]
    #[serial_test::serial]
    async fn public_keys_carry_alg_and_thumbprint_kid() {
        let (store, _path) = stronghold_manager();

        for (key, alg) in KEYS {
            let jwk = store.public_jwk(&KeyId::new(key)).await.unwrap();
            assert_eq!(jwk.alg(), Some(alg.as_str()));
            assert_eq!(jwk.kid(), Some(jwk.thumbprint_sha256().unwrap().as_str()));
        }
    }

    /// identity-kit's contract for key stores: `public_jwk`, `sign` in the JWS signature form, `AlgorithmMismatch`,
    /// `NotFound` and no `generate` or `delete`.
    #[tokio::test]
    #[serial_test::serial]
    async fn follows_the_key_store_contract() {
        let (store, _path) = stronghold_manager();

        identity_kit::keys::testsuite::run_with_keys(&store, &KEYS.map(|(key, alg)| (KeyId::new(key), alg))).await;
    }
}
