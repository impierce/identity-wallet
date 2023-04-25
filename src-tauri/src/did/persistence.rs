use std::path::PathBuf;

use did_key::{AsymmetricKey, Ed25519KeyPair, Generate};
use ed25519_dalek::{PublicKey, SecretKey};
use tauri::Manager;
use tokio::{
    fs::{read, File},
    io::AsyncWriteExt,
};
use tracing::info;

/// CAUTION: UNSAFE! - A keypair can be loaded from storage when it needs to be used (for example for signatures).
pub async fn load_existing_keypair() -> anyhow::Result<AsymmetricKey<PublicKey, SecretKey>> {
    // let unsafe_key_file_path = app_handle.path().data_dir()?.join("com.tauri.dev").join("unsafe.bin");
    let unsafe_key_file_path = PathBuf::from("/Users/daniel/Library/Application Support/com.tauri.dev/unsafe.bin");
    let bytes = read(&unsafe_key_file_path).await?;
    info!("existing unsafe private key loaded from app_data_dir: {:?}", bytes);
    let keypair = Ed25519KeyPair::from_secret_key(&bytes);
    Ok(keypair)
}

/// CAUTION: UNSAFE! - A secret key can be saved to storage for later use.
pub async fn save_secret_key(secret_key_bytes: [u8; 32]) -> anyhow::Result<()> {
    // let unsafe_key_file_path = app_handle.path().data_dir()?.join("com.tauri.dev").join("unsafe.bin");
    let unsafe_key_file_path = PathBuf::from("/Users/daniel/Library/Application Support/com.tauri.dev/unsafe.bin");
    let mut file = File::create(&unsafe_key_file_path).await?;
    file.write_all(&secret_key_bytes).await?;
    info!(
        "new unsafe private key saved to app_data_dir: {}",
        unsafe_key_file_path.display()
    );
    Ok(())
}
