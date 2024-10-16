use crate::{persistence::STRONGHOLD, state::credentials::VerifiableCredentialRecord};

use iota_stronghold::{
    procedures::{GenerateKey, KeyType, StrongholdProcedure},
    Client, KeyProvider, Location, SnapshotPath, Stronghold,
};
use log::info;
use stronghold_ext::{
    execute_procedure_ext,
    procs::{self, es256::Es256Procs},
};
use uuid::Uuid;

// This file is where we implement the stronghold library for our app, which is used to store sensitive data.
// We have to follow the hard-coded values used in `identity.rs` to make our Stronghold compatible.
static STRONGHOLD_VAULT_PATH: &str = "iota_identity_vault";
static STRONGHOLD_CLIENT_PATH: &[u8] = b"iota_identity_client";

/// This struct is the main point of communication between our appstate and the stronghold library.
#[derive(Debug)]
pub struct StrongholdManager {
    stronghold: Stronghold,
    client: Client,
    key_provider: KeyProvider,
    snapshot_path: SnapshotPath,
}

impl StrongholdManager {
    pub fn create(password: &str) -> anyhow::Result<Self> {
        let stronghold = Stronghold::default();
        let client_path = STRONGHOLD
            .lock()
            .unwrap()
            .to_str()
            .ok_or(anyhow::anyhow!("failed to get stronghold path"))?
            .to_owned();

        let snapshot_path = SnapshotPath::from_path(client_path.clone());
        let key_provider =
            KeyProvider::with_passphrase_hashed_blake2b(password.as_bytes().to_vec()).expect("failed to load key");

        let client: Client = stronghold
            .create_client(STRONGHOLD_CLIENT_PATH)
            .expect("cannot create client");

        // Generate ed25519 key
        {
            let ed25519_output_location = Location::generic(
                STRONGHOLD_VAULT_PATH.as_bytes().to_vec(),
                "ed25519-0".to_string().as_bytes().to_vec(),
            );

            info!("ed25519_output_location: {:?}", ed25519_output_location);

            client
                .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
                    ty: KeyType::Ed25519,
                    output: ed25519_output_location,
                }))
                .expect("failed to generate new private key");

            info!("Successfully generated new private key with type Ed25519");
        }

        // Generate ES256 Key
        {
            let es256_output_location = Location::generic(
                STRONGHOLD_VAULT_PATH.as_bytes().to_vec(),
                "es256-0".to_string().as_bytes().to_vec(),
            );

            info!("es256_output_location: {:?}", es256_output_location);

            execute_procedure_ext(
                &client,
                Es256Procs::GenerateKey(procs::es256::GenerateKey {
                    output: es256_output_location,
                }),
            )
            .expect("failed to generate new private key");

            info!("Successfully generated new private key with type Es256");
        }

        let stronghold_manager = Self {
            stronghold,
            client,
            key_provider,
            snapshot_path: snapshot_path.clone(),
        };

        stronghold_manager.commit()?;
        Ok(stronghold_manager)
    }

    pub fn load(password: &str) -> anyhow::Result<Self> {
        let stronghold = Stronghold::default();
        let client_path = STRONGHOLD
            .lock()
            .unwrap()
            .to_str()
            .ok_or(anyhow::anyhow!("failed to get stronghold path"))?
            .to_owned();
        let snapshot_path = SnapshotPath::from_path(client_path.clone());
        let key_provider =
            KeyProvider::with_passphrase_hashed_blake2b(password.as_bytes().to_vec()).expect("failed to load key");

        info!("Loading snapshot");

        let client = stronghold.load_client_from_snapshot(STRONGHOLD_CLIENT_PATH, &key_provider, &snapshot_path)?;

        Ok(Self {
            stronghold,
            client,
            key_provider,
            snapshot_path,
        })
    }

    pub fn commit(&self) -> anyhow::Result<()> {
        // Set the work factor to 10 to speed up the commit.
        // TODO: security: weaker encryption?
        engine::snapshot::try_set_encrypt_work_factor(10)?;

        self.stronghold
            .write_client(STRONGHOLD_CLIENT_PATH)
            .expect("store client state into snapshot state failed");

        self.stronghold
            .commit_with_keyprovider(&self.snapshot_path, &self.key_provider)
            .expect("stronghold could not commit");

        Ok(())
    }

    pub fn get(&self, key: Uuid) -> anyhow::Result<Option<Vec<u8>>> {
        let key = key.to_string().as_bytes().to_vec();
        let value = self.client.store().get(&key)?;

        Ok(value)
    }

    pub fn insert(&self, key: Uuid, value: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .store()
            .insert(key.to_string().as_bytes().to_vec(), value, None)?;

        self.commit()
    }

    // TODO: fix this function's return type.
    pub fn values(&self) -> anyhow::Result<Option<Vec<VerifiableCredentialRecord>>> {
        let client = self.client.clone();

        let mut keys = self.client.store().keys()?;
        keys.sort();
        keys.iter()
            .map(|key| {
                client
                    .store()
                    .get(key)
                    .map(|value| value.map(|value| serde_json::from_slice(&value)))
            })
            .collect::<Result<Option<serde_json::Result<_>>, _>>()?
            .transpose()
            .map_err(|e| anyhow::anyhow!(e))
    }

    pub fn remove(&self, key: Uuid) -> anyhow::Result<Option<Vec<u8>>> {
        let value = self.client.store().delete(key.to_string().as_bytes())?;
        self.commit()?;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    #[serial_test::serial]
    fn test_stronghold_manager() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *STRONGHOLD.lock().unwrap() = path.as_os_str().into();

        let stronghold_manager = StrongholdManager::create("sup3rSecr3t").unwrap();

        let key = Uuid::new_v4();
        let value = "test".as_bytes().to_vec();

        stronghold_manager.insert(key, value).unwrap();

        let value = stronghold_manager.get(key).unwrap().unwrap();
        assert_eq!(value, "test".as_bytes().to_vec());

        stronghold_manager.remove(key).unwrap();

        let value = stronghold_manager.get(key).unwrap();
        assert!(value.is_none());
    }
}
