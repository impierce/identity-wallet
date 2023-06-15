use crate::did::persistence::save_secret_key;
use did_key::{Config, DIDCore, Document, Ed25519KeyPair, Generate};
use tracing::info;

/// Generates a DID document with method did:key from an existing public key (unsigned).
pub async fn generate_new_did(public_key: Vec<u8>) -> anyhow::Result<Document> {
    let keypair = Ed25519KeyPair::from_public_key(&public_key);
    let did_document = keypair.get_did_document(Config::default());
    info!("{:#?}", did_document);
    Ok(did_document)
}

/// CAUTION: UNSAFE DEV MODE! - Uses a static secret key, saves it to disk (unencrypted!) and returns its DID.
pub async fn generate_dev_did() -> anyhow::Result<Document> {
    let unsafe_dev_secret_key: [u8; 32] = "this-is-a-very-UNSAFE-secret-key".as_bytes().try_into().unwrap();
    save_secret_key(unsafe_dev_secret_key).await?;
    let keypair = Ed25519KeyPair::from_secret_key(&unsafe_dev_secret_key);
    Ok(keypair.get_did_document(Config::default()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UNSAFE_DEV_STORAGE;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_generate_dev_did() {
        let path = NamedTempFile::new().unwrap().into_temp_path();
        *UNSAFE_DEV_STORAGE.lock().unwrap() = path.as_os_str().into();

        let did_document_json = generate_dev_did().await.unwrap();
        assert_eq!(
            did_document_json.id,
            "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"
        );
    }
}
