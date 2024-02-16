use derivative::Derivative;
use oid4vc::oid4vci::credential_format_profiles::{CredentialFormats, WithCredential};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
                    },
                    display_name: None,
                    display_icon: None,
                    display_color: None,
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/display-credential/DisplayCredential.ts")]
pub struct DisplayCredential {
    pub id: String,
    pub issuer_name: Option<String>,
    #[ts(type = "string")]
    pub format: CredentialFormats,
    #[ts(type = "object")]
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: CredentialMetadata,

    pub display_icon: Option<String>,
    pub display_color: Option<String>,
    pub display_name: Option<String>,
}

impl DisplayCredential {
    pub fn get_achievement_name_from_data(&self) -> Option<String> {
        let cred_subject = self.data.get("credentialSubject")?;
        let achievement = cred_subject.get("achievement")?;
        let name = achievement.get("name")?;

        Some(name.to_string())
    }

    pub fn get_type_name_from_data(&self) -> Option<String> {
        let data_type = self.data.get("type")?;

        match data_type {
            Value::Array(array) => {
                let last = array.last()?;
                Some(last.to_string())
            }
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, TS, Default, Derivative)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/display-credential/CredentialMetadata.ts")]
pub struct CredentialMetadata {
    pub is_favorite: bool,
    #[derivative(PartialEq = "ignore")]
    pub date_added: String,
    #[derivative(PartialEq = "ignore")]
    pub date_issued: String,
}
