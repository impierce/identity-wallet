use derivative::Derivative;
use log::info;
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

                let display_name = get_achievement_name_from_data(&credential_display)
                    .or(get_type_name_from_data(&credential_display))
                    .unwrap_or("".to_string());

                let credential_type = get_credential_type(&credential_display);

                DisplayCredential {
                    id: Uuid::from_slice(&hash.as_bytes()[..16]).unwrap().to_string(),
                    issuer_name: "".to_string(),
                    format: verifiable_credential.format().unwrap(),
                    data: credential_display,
                    metadata: CredentialMetadata {
                        is_favorite: false,
                        date_added: chrono::Utc::now().to_rfc3339(),
                        date_issued: issuance_date.to_string(),
                    },
                    display_name,
                    credential_type,
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

fn get_credential_type(credential_display: &serde_json::Value) -> CredentialType {
    let cred_type = credential_display.get("type");

    if let Some(cred_type) = cred_type {
        let cred_type_str = cred_type.to_string();

        info!("Cred type str: {:?}", cred_type_str);

        if cred_type_str.contains("OpenBadgeCredential") {
            return CredentialType::Badge;
        }
    }

    CredentialType::Credential
}

fn get_achievement_name_from_data(credential_display: &serde_json::Value) -> Option<String> {
    let cred_subject = credential_display.get("credentialSubject")?;
    let achievement = cred_subject.get("achievement")?;
    let name = achievement.get("name")?;

    // Don't use the to_string for the name variable as it add's "" around the string
    name.as_str().map(|name| name.to_string())
}

fn get_type_name_from_data(credential_display: &serde_json::Value) -> Option<String> {
    match credential_display.get("type")? {
        // Don't use the to_string for the name variable as it add's "" around the string
        Value::Array(array) => array.last()?.as_str().map(|name| name.to_string()),
        _ => None,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS)]
#[ts(export, export_to = "bindings/display-credential/CredentialType.ts")]
pub enum CredentialType {
    Badge,
    Credential,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/display-credential/DisplayCredential.ts")]
pub struct DisplayCredential {
    /// Also display_icon
    pub id: String,
    pub issuer_name: String,
    #[ts(type = "string")]
    pub format: CredentialFormats,
    #[ts(type = "any")]
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: CredentialMetadata,

    pub display_name: String,
    pub credential_type: CredentialType,
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
