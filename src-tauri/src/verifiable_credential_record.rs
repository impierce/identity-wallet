use chrono::Local;
use oid4vci::credential_format_profiles::{CredentialFormats, WithCredential};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

use crate::get_unverified_jwt_claims;

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
                    format: (&verifiable_credential).try_into().unwrap(),
                    data: credential_display,
                    metadata: CredentialMetadata {
                        is_favorite: false,
                        date_added: Local::now().format("%Y-%m-%d").to_string(), // should be same format as date_issued, and all other dates.
                        date_issued: issuance_date.to_string(),
                        display: CredentialDisplay::default()
                    }
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS)]
#[ts(export, export_to = "bindings/display-credential/DisplayCredential.ts")]
pub struct DisplayCredential {
    pub id: String,
    pub issuer_name: Option<String>,
    #[ts(type = "string")]
    pub format: CredentialFormats<()>,
    #[ts(type = "object")]
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: CredentialMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialMetadata.ts")]
pub struct CredentialMetadata {
    pub is_favorite: bool,
    pub date_added: String,
    pub date_issued: String,
    pub display: CredentialDisplay,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialDisplay.ts")]
pub struct CredentialDisplay {
    pub icon: Option<String>,
    pub color: Option<String>,
    pub name: Option<String>,
}
