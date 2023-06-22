use did_key::{Ed25519KeyPair, Generate};
use log::warn;
use tokio::{
    fs::{read, File},
    io::AsyncWriteExt,
};

use crate::UNSAFE_DEV_STORAGE;

/// CAUTION: UNSAFE! - A keypair can be loaded from storage when it needs to be used (for example for signatures).
pub async fn load_existing_keypair() -> anyhow::Result<Ed25519KeyPair> {
    let unsafe_storage = UNSAFE_DEV_STORAGE.lock().unwrap().clone();
    let bytes = read(unsafe_storage).await?;
    warn!("existing unsafe private key loaded from disk");
    let keypair = Ed25519KeyPair::from_secret_key(&bytes);
    Ok(keypair)
}

/// CAUTION: UNSAFE! - A secret key can be saved to storage for later use.
pub async fn save_secret_key(secret_key_bytes: [u8; 32]) -> anyhow::Result<()> {
    let unsafe_storage = UNSAFE_DEV_STORAGE.lock().unwrap().clone();
    let mut file = File::create(unsafe_storage).await?;
    file.write_all(&secret_key_bytes).await?;
    warn!("new unsafe private key saved to disk");
    Ok(())
}
