mod common;

use crate::common::{get_jwt_claims, memory_storage::MemoryStorage};
use did_key::{generate, Ed25519KeyPair};
use oid4vc_core::Subject;
use oid4vc_manager::{
    managers::credential_issuer::CredentialIssuerManager, methods::key_method::KeySubject,
    servers::credential_issuer::Server,
};
use oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_offer::{CredentialOffer, CredentialOfferQuery, CredentialsObject, Grants},
    credential_response::{BatchCredentialResponse, CredentialResponse, CredentialResponseType},
    token_request::{PreAuthorizedCode, TokenRequest},
    Wallet,
};
use std::{net::TcpListener, sync::Arc};

#[tokio::main]
async fn main() {
    // Setup the credential issuer.
    let mut credential_issuer = Server::<_, CredentialFormats>::setup(
        CredentialIssuerManager::new(
            Some(TcpListener::bind("0.0.0.0:8000").unwrap()),
            MemoryStorage,
            [Arc::new(KeySubject::from_keypair(
                generate::<Ed25519KeyPair>(Some(
                    "this-is-a-very-UNSAFE-issuer-secret-key"
                        .as_bytes()
                        .try_into()
                        .unwrap(),
                )),
                None,
            ))],
        )
        .unwrap(),
        None,
    )
    .unwrap();
    dbg!(&credential_issuer
        .credential_issuer_manager
        .credential_issuer_url()
        .unwrap()
        .as_str());
    credential_issuer.start_server().await.unwrap();
}
