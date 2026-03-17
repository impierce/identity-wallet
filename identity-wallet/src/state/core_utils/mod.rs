pub mod helpers;
pub mod history_event;

#[cfg(target_os = "android")]
use log::info;
#[cfg(target_os = "android")]
use rustls::{client::danger::ServerCertVerifier, ClientConfig};
#[cfg(target_os = "android")]
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerified},
    pki_types::{CertificateDer, ServerName, UnixTime},
    DigitallySignedStruct, Error, SignatureScheme,
};
#[cfg(target_os = "android")]
use rustls_platform_verifier::BuilderVerifierExt;

use crate::command::Runtime;
use crate::stronghold::StrongholdManager;
use crate::subject::Subject;
pub use helpers::DateUtils;

use oid4vc::{
    oid4vc_core::authorization_request::{AuthorizationRequest, Object},
    oid4vc_manager::ProviderManager,
    oid4vci::{credential_offer::CredentialOfferParameters, Wallet},
    oid4vp::oid4vp::OID4VP,
    siopv2::siopv2::SIOPv2,
};

use serde::{Deserialize, Serialize};
use std::sync::Arc;

// TODO: Warning! This is very unsafe, this is only used for Android builds to temporarily bypass certificate
// verification! The problem with `rustls_platform_verifier` on Android is that the Android Trust Store seems to
// require OCSP stapling, which not all servers support. `api.mainnet.iota.cafe` is one of those servers.
// This should be replaced with a proper certificate verifier that is secure and works on Android.
// More info: https://github.com/rustls/rustls-platform-verifier/pull/179
#[cfg(target_os = "android")]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UnsafeCertVerifier;

#[cfg(target_os = "android")]
impl ServerCertVerifier for UnsafeCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::aws_lc_rs::default_provider()
            .signature_verification_algorithms
            .supported_schemes()
    }

    fn requires_raw_public_keys(&self) -> bool {
        false
    }

    fn root_hint_subjects(&self) -> Option<&[rustls::DistinguishedName]> {
        None
    }
}

#[cfg(target_os = "android")]
pub async fn tls_config() -> anyhow::Result<rustls::ClientConfig> {
    info!("Creating TLS config for Android");
    let arc_crypto_provider = std::sync::Arc::new(rustls::crypto::ring::default_provider());

    info!("Using crypto provider: {:?}", arc_crypto_provider);
    let mut config = ClientConfig::builder_with_provider(arc_crypto_provider)
        .with_safe_default_protocol_versions()?
        .with_platform_verifier()?
        .with_no_client_auth();

    // TODO: implement a secure custom certificate verifier
    config
        .dangerous()
        .set_certificate_verifier(Arc::new(UnsafeCertVerifier));

    info!("TLS config created for Android");

    Ok(config)
}

/// CoreUtils is a struct that contains all the utils that only the rustside needs to perform its tasks.
#[derive(Clone, Default, Debug)]
pub struct CoreUtils {
    pub app_handle: Option<tauri::AppHandle<Runtime>>,
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,

    // TODO: These 'active_' fields should either be part of `oid4vc-manager`, or the `IdentityManager` struct.
    pub active_connection_request: Option<ConnectionRequest>,
    pub active_credential_configuration_ids: Option<Vec<String>>,
    pub active_credential_offer: Option<CredentialOfferParameters>,
    pub active_code_verifier: Option<Vec<u8>>,
    pub active_wallet_state: Option<String>,
}

/// Managers contains both the stronghold manager and the identity manager needed to perform operations on connections & credentials.
#[derive(Default, Debug)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

/// IdentityManager contains the subject, provider_manager and wallet needed to perform operations within the oid4vc library.
#[derive(Debug)]
pub struct IdentityManager {
    pub subject: Arc<Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    OID4VP(Box<AuthorizationRequest<Object<OID4VP>>>),
}
