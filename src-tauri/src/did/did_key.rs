use crate::did::persistence::save_secret_key;
use did_key::{Config, DIDCore, Document, Ed25519KeyPair, Generate};
use rand::RngCore;

/// CAUTION: UNSAFE! - Generates a new random secret key, saves it to disk (unencrypted!) and returns its DID.
pub async fn generate_new_did() -> anyhow::Result<Document> {
    let mut unsafe_secret_key = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut unsafe_secret_key);
    save_secret_key(unsafe_secret_key).await?;
    let keypair = Ed25519KeyPair::from_secret_key(&unsafe_secret_key);
    Ok(keypair.get_did_document(Config::default()))
}

/// CAUTION: UNSAFE! - Generates always the same static secret key, saves it disk (unencrypted!) and returns its DID.
pub async fn generate_dev_did() -> anyhow::Result<Document> {
    let unsafe_dev_secret_key: [u8; 32] = "this-is-a-very-UNSAFE-secret-key".as_bytes().try_into().unwrap();
    save_secret_key(unsafe_dev_secret_key).await?;
    let keypair = Ed25519KeyPair::from_secret_key(&unsafe_dev_secret_key);
    Ok(keypair.get_did_document(Config::default()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_dev_did() {
        let did_document_json = generate_dev_did().await.unwrap();
        assert_eq!(
            did_document_json.id,
            "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY"
        );
    }
}
