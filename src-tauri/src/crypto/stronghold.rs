use crate::{verifiable_credential_record::VerifiableCredentialRecord, STRONGHOLD};
use iota_stronghold::{
    procedures::{Ed25519Sign, GenerateKey, KeyType, PublicKey, StrongholdProcedure},
    Client, KeyProvider, Location, SnapshotPath, Stronghold,
};
use log::{debug, info};
use oid4vc_core::authentication::sign::ExternalSign;
use uuid::Uuid;

pub fn hash_password(password: &str) -> anyhow::Result<Vec<u8>> {
    let config = argon2::Config::default();

    let password_hash = argon2::hash_raw(password.as_ref(), b"D4F88D86F2C60DF8AB3EC3821083EF89", &config)?;
    debug!("password hashed successfully");

    Ok(password_hash)
}

#[derive(Debug)]
pub struct StrongholdManager {
    stronghold: Stronghold,
    client: Client,
    client_path: String,
    key_provider: KeyProvider,
    snapshot_path: SnapshotPath,
}

impl StrongholdManager {
    pub fn create(password: &str) -> anyhow::Result<Self> {
        let stronghold = Stronghold::default();
        let client_path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();

        let snapshot_path = SnapshotPath::from_path(format!("{client_path}.snapshot"));
        let key_provider = KeyProvider::try_from(hash_password(password)?).expect("failed to load key");

        let client: Client = stronghold.create_client(&client_path).expect("cannot create client");
        let output_location = Location::counter(client_path.clone(), 0u8);

        client
            .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
                ty: KeyType::Ed25519,
                output: output_location,
            }))
            .expect("failed to generate new private key");

        let stronghold_manager = Self {
            stronghold,
            client,
            client_path,
            key_provider,
            snapshot_path: snapshot_path.clone(),
        };

        let public_key = stronghold_manager.get_public_key()?;
        debug!("public_key (base64): {:?}", base64::encode(public_key));

        stronghold_manager.commit()?;
        Ok(stronghold_manager)
    }

    pub fn load(password: &str) -> anyhow::Result<Self> {
        let stronghold = Stronghold::default();
        let client_path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();
        let snapshot_path = SnapshotPath::from_path(format!("{client_path}.snapshot"));
        let key_provider = KeyProvider::try_from(hash_password(password)?).expect("failed to load key");

        info!("Loading snapshot");

        let client = stronghold.load_client_from_snapshot(&client_path, &key_provider, &snapshot_path)?;

        Ok(Self {
            stronghold,
            client,
            client_path,
            key_provider,
            snapshot_path,
        })
    }

    pub fn commit(&self) -> anyhow::Result<()> {
        self.stronghold
            .write_client(self.client_path.as_bytes())
            .expect("store client state into snapshot state failed");

        self.stronghold
            .commit_with_keyprovider(&self.snapshot_path, &self.key_provider)
            .expect("stronghold could not commit");

        Ok(())
    }

    pub fn get(&self, key: Uuid) -> anyhow::Result<Option<Vec<u8>>> {
        let key = key.to_string().as_bytes().to_vec();
        let value = self.client.store().get(&key).unwrap();

        Ok(value)
    }

    pub fn insert(&self, key: Uuid, value: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .store()
            .insert(key.to_string().as_bytes().to_vec(), value, None)
            .unwrap();

        self.commit()
    }

    // TODO: fix this function's return type.
    pub fn values(&self) -> anyhow::Result<Option<Vec<VerifiableCredentialRecord>>> {
        let client = self.client.clone();

        let mut keys = self.client.store().keys()?;
        keys.sort();
        let verifiable_credential_record: Vec<VerifiableCredentialRecord> = keys
            .iter()
            .map(|key| {
                serde_json::from_str(std::str::from_utf8(&client.store().get(key).unwrap().unwrap()).unwrap()).unwrap()
            })
            .collect();

        Ok(Some(verifiable_credential_record))
    }

    pub fn remove(&self, key: Uuid) -> anyhow::Result<()> {
        self.client.store().delete(key.to_string().as_bytes()).unwrap();

        self.commit()
    }

    pub fn get_public_key(&self) -> anyhow::Result<Vec<u8>> {
        debug!("Creating public key");
        let procedure_result = self
            .client
            .execute_procedure(StrongholdProcedure::PublicKey(PublicKey {
                ty: KeyType::Ed25519,
                private_key: Location::counter(self.client_path.as_bytes(), 0u8),
            }))
            .unwrap();

        let output: Vec<u8> = procedure_result.into();
        info!(r#"Public key is "{}" (base64)"#, base64::encode(&output));

        Ok(output)
    }
}

impl ExternalSign for StrongholdManager {
    fn sign(&self, message: &str) -> anyhow::Result<Vec<u8>> {
        let client_path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();
        let procedure_result = self
            .client
            .execute_procedure(StrongholdProcedure::Ed25519Sign(Ed25519Sign {
                private_key: Location::counter(client_path.clone(), 0u8),
                msg: message.as_bytes().to_vec(),
            }))?;

        let output: Vec<u8> = procedure_result.into();
        info!(r#"Signature is "{}" (base64)"#, base64::encode(&output));

        Ok(output)
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
