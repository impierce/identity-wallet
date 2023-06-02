use crate::did::persistence::load_existing_keypair;
use did_key::{generate, Ed25519KeyPair, KeyMaterial};
use openid4vc::{key_method::KeySubject, Provider};

// TODO: This is a temporary solution to get a `Provider` instance by obtaining a keypair from the unsafe storage.
//       This should be replaced with a DID method agnostic solution.
pub async fn provider() -> anyhow::Result<Provider<KeySubject>> {
    let private = load_existing_keypair().await?.private_key_bytes();

    // Use private key to create a `KeySubject`.
    let key_subject = KeySubject::from_keypair(generate::<Ed25519KeyPair>(Some(private.as_slice())));

    // Return the `Provider` instance.
    Ok(Provider::new(key_subject).await?)
}
