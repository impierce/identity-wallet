use iota_stronghold::{
    procedures::{GenerateKey, KeyType, PublicKey, StrongholdProcedure},
    Client, KeyProvider, Location, SnapshotPath, Stronghold,
};
use log::{debug, info};
use oid4vci::credential_format_profiles::{CredentialFormats, WithCredential};
use uuid::Uuid;

use crate::STRONGHOLD;

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
            snapshot_path,
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

    pub fn insert(&self, key: Uuid, value: Vec<u8>) -> anyhow::Result<()> {
        self.client
            .store()
            .insert(key.to_string().as_bytes().to_vec(), value, None)
            .unwrap();

        self.commit()
    }

    // TODO: fix this function's return type.
    pub fn get_all(&self) -> anyhow::Result<Option<Vec<(Uuid, CredentialFormats<WithCredential>)>>> {
        let mut credentials: Vec<(Uuid, CredentialFormats<WithCredential>)> = self
            .client
            .store()
            .keys()?
            .iter()
            .map(|key| {
                (
                    Uuid::parse_str(std::str::from_utf8(key).unwrap()).unwrap(),
                    serde_json::from_str(std::str::from_utf8(&self.client.store().get(key).unwrap().unwrap()).unwrap())
                        .unwrap(),
                )
            })
            .collect();
        credentials.sort_by(|a, b| a.0.cmp(&b.0));

        Ok(Some(credentials))
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
