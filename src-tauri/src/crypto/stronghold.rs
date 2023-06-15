use iota_stronghold::{Client, Stronghold, Location, procedures::{StrongholdProcedure, GenerateKey, KeyType, ProcedureOutput, PublicKey}, SnapshotPath, KeyProvider};
use tracing::info;

use crate::STRONGHOLD;

pub async fn hash_password(password: &str) -> anyhow::Result<Vec<u8>> {
    // TODO: use the following config or the default config?
    let config = argon2::Config {
        lanes: 2,
        mem_cost: 50_000,
        time_cost: 30,
        thread_mode: argon2::ThreadMode::from_threads(2),
        variant: argon2::Variant::Argon2id,
        ..Default::default()
    };

    let config = argon2::Config::default();

    // TODO: use a random salt for each instance of the app
    let password_hash = argon2::hash_raw(password.as_ref(), b"SALT_IDEALLY_SHOULD_BE_RANDOM", &config)
        .expect("failed to hash password");
    info!("password hashed successfully");

    Ok(password_hash)
}

pub async fn create_new_stronghold(password_hash: Vec<u8>) -> anyhow::Result<()> {
    let stronghold = Stronghold::default();
    
    // do we need to differ between: client_path and snapshot_path?
    let path = STRONGHOLD.lock().unwrap().clone().to_str().unwrap().to_owned();

    let client: Client = stronghold.create_client(path.clone()).expect("cannot create client");

    // TODO: should we use a counter or a generic location?
    let output_location = Location::generic("my-vault".as_bytes().to_vec(), "my-key".as_bytes().to_vec());
    let output_location = Location::counter(path.clone(), 0u8);

    assert!(client
        .execute_procedure(StrongholdProcedure::GenerateKey(GenerateKey {
            ty: KeyType::Ed25519,
            output: output_location.clone(),
        }))
        .is_ok());

    let output: ProcedureOutput = client
        .execute_procedure(StrongholdProcedure::PublicKey(PublicKey {
            ty: KeyType::Ed25519,
            private_key: output_location,
        }))
        .unwrap();
    let public_key: [u8; 32] = output.try_into().unwrap();

    info!("public_key (base64): {:?}", base64::encode(public_key));

    stronghold.write_client(path.clone()).expect("store client state into snapshot state failed");

    info!(
        "snapshot created successully? {}",
        stronghold
            .commit_with_keyprovider(&SnapshotPath::from_path(path), &KeyProvider::try_from(password_hash).unwrap())
            .is_ok()
    );

    Ok(())
}
