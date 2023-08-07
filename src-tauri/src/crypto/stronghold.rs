use iota_stronghold::{
    procedures::{GenerateKey, KeyType, ProcedureOutput, PublicKey, StrongholdProcedure},
    Client, KeyProvider, Location, SnapshotPath, Stronghold,
};
use log::{debug, info};
use oid4vci::credential_format_profiles::{CredentialFormats, WithCredential};

use crate::STRONGHOLD;

pub async fn hash_password(password: &str) -> anyhow::Result<Vec<u8>> {
    let config = argon2::Config::default();

    let password_hash = argon2::hash_raw(password.as_ref(), b"D4F88D86F2C60DF8AB3EC3821083EF89", &config)
        .expect("failed to hash password");
    debug!("password hashed successfully");

    Ok(password_hash)
}

pub async fn create_new_stronghold(password: &str) -> anyhow::Result<()> {
    let stronghold = Stronghold::default();

    let path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();

    let client: Client = stronghold.create_client(&path).expect("cannot create client");

    let output_location = Location::counter(path.clone(), 0u8);

    client
        .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
            ty: KeyType::Ed25519,
            output: output_location.clone(),
        }))
        .expect("failed to generate new private key");

    let output: ProcedureOutput = client
        .execute_procedure(StrongholdProcedure::PublicKey(PublicKey {
            ty: KeyType::Ed25519,
            private_key: output_location,
        }))
        .expect("failed to get public key");

    let public_key: [u8; 32] = output.try_into().unwrap();

    debug!("public_key (base64): {:?}", base64::encode(public_key));

    stronghold
        .write_client(&path)
        .expect("store client state into snapshot state failed");

    stronghold
        .commit_with_keyprovider(
            &SnapshotPath::from_path(path),
            &KeyProvider::try_from(hash_password(password).await.unwrap()).unwrap(),
        )
        .expect("stronghold could not commit");

    Ok(())
}

pub async fn get_public_key(password: &str) -> anyhow::Result<Vec<u8>> {
    let stronghold = Stronghold::default();
    let path = STRONGHOLD.lock().unwrap().clone().to_str().unwrap().to_owned();
    let snapshot_path = SnapshotPath::from_path(path.clone());

    let key = hash_password(password).await?;
    let keyprovider = KeyProvider::try_from(key).expect("failed to load key");

    info!("Loading snapshot");

    let client = stronghold
        .load_client_from_snapshot(path.clone(), &keyprovider, &snapshot_path)
        .expect("Could not load client from Snapshot");

    debug!("Creating public key");
    let procedure_result = client
        .execute_procedure(StrongholdProcedure::PublicKey(PublicKey {
            ty: KeyType::Ed25519,
            private_key: Location::counter(path.clone(), 0u8),
        }))
        .unwrap();

    let output: Vec<u8> = procedure_result.into();
    info!(r#"Public key is "{}" (base64)"#, base64::encode(&output));

    Ok(output)
}

pub async fn insert_into_stronghold(key: Vec<u8>, value: Vec<u8>, password: &str) -> anyhow::Result<()> {
    let stronghold = Stronghold::default();

    let path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();

    let client: Client = stronghold.create_client(&path).expect("cannot create client");

    client.store().insert(key, value, None).unwrap();

    stronghold
        .write_client(&path)
        .expect("store client state into snapshot state failed");

    stronghold
        .commit_with_keyprovider(
            &SnapshotPath::from_path(path),
            &KeyProvider::try_from(hash_password(password).await.unwrap()).unwrap(),
        )
        .expect("stronghold could not commit");

    Ok(())
}

pub async fn get_all_from_stronghold(password: &str) -> anyhow::Result<Option<Vec<CredentialFormats<WithCredential>>>> {
    let stronghold = Stronghold::default();

    let path = STRONGHOLD.lock().unwrap().to_str().unwrap().to_owned();
    let snapshot_path = SnapshotPath::from_path(path.clone());

    let keyprovider = KeyProvider::try_from(hash_password(password).await?).expect("failed to load key");

    info!("Loading snapshot");

    let client = stronghold
        .load_client_from_snapshot(path.clone(), &keyprovider, &snapshot_path)
        .expect("Could not load client from Snapshot");

    let credentials: Vec<CredentialFormats<WithCredential>> = client
        .store()
        .keys()?
        .iter()
        .map(|key| {
            serde_json::from_str(std::str::from_utf8(&client.store().get(key).unwrap().unwrap()).unwrap()).unwrap()
        })
        .collect();

    Ok(Some(credentials))
}
