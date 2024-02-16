use crate::state::credentials::{CredentialDisplay, CredentialMetadata, DisplayCredential};
use crate::stronghold::StrongholdManager;
use oid4vc::{oid4vc_core::Subject, 
    oid4vc_manager::ProviderManager, 
    oid4vci::{Wallet, credential_format_profiles::{CredentialFormats, WithCredential}}};
use oid4vc::oid4vc_core::authorization_request::{AuthorizationRequest, Object}; ///
use oid4vc::oid4vp::oid4vp::OID4VP;
use oid4vc::siopv2::siopv2::SIOPv2;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

/// Utils used only in the backend.
#[derive(Default)]
pub struct BackEndUtils {
    pub managers: Arc<tauri::async_runtime::Mutex<Managers>>,
    pub active_connection_request: Option<ConnectionRequest>
}

#[derive(Serialize, Deserialize)]
pub enum ConnectionRequest {
    SIOPv2(Box<AuthorizationRequest<Object<SIOPv2>>>),
    OID4VP(Box<AuthorizationRequest<Object<OID4VP>>>),
}

pub struct IdentityManager {
    pub subject: Arc<dyn Subject>,
    pub provider_manager: ProviderManager,
    pub wallet: Wallet,
}

#[derive(Default)]
pub struct Managers {
    pub stronghold_manager: Option<Arc<StrongholdManager>>,
    pub identity_manager: Option<IdentityManager>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VerifiableCredentialRecord {
    pub verifiable_credential: CredentialFormats<WithCredential>,
    pub display_credential: DisplayCredential,
}

impl From<CredentialFormats<WithCredential>> for VerifiableCredentialRecord {
    fn from(verifiable_credential: CredentialFormats<WithCredential>) -> Self {
        let display_credential = match &verifiable_credential {
            CredentialFormats::JwtVcJson(credential) => {
                let credential_display = get_unverified_jwt_claims(&credential.credential)["vc"].clone();

                // Derive the hash from the credential display.
                let hash = {
                    let type_key = "type";
                    let type_value = credential_display[type_key].clone();

                    let credential_subject_key = "credentialSubject";
                    let mut credential_subject_value = credential_display[credential_subject_key].clone();

                    // TODO: Remove this hard-coded logic.
                    // Remove the `Passport Number` and `Staff Number` from the credential subject if they exists.
                    credential_subject_value["Passport Number"].take();
                    credential_subject_value["Staff Number"].take();
                    credential_subject_value["achievement"]["id"].take();

                    sha256::digest(
                        json!(
                            {
                                "type": type_value,
                                "credentialSubject": credential_subject_value,
                            }
                        )
                        .to_string(),
                    )
                };

                let issuance_date = credential_display["issuanceDate"].clone();

                DisplayCredential {
                    id: Uuid::from_slice(&hash.as_bytes()[..16]).unwrap().to_string(),
                    issuer_name: None,
                    format: verifiable_credential.format().unwrap(),
                    data: credential_display,
                    metadata: CredentialMetadata {
                        is_favorite: false,
                        date_added: chrono::Utc::now().to_rfc3339(),
                        date_issued: issuance_date.to_string(),
                        display: CredentialDisplay::default(),
                    },
                }
            }
            _ => unimplemented!(),
        };

        Self {
            verifiable_credential,
            display_credential,
        }
    }
}

// Get the claims from a JWT without performing validation.
pub fn get_unverified_jwt_claims(jwt: &serde_json::Value) -> serde_json::Value {
    let key = DecodingKey::from_secret(&[]);
    let mut validation = Validation::new(Algorithm::EdDSA);
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;
    validation.required_spec_claims.clear();
    decode(jwt.as_str().unwrap(), &key, &validation).unwrap().claims
}
