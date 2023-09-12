use oid4vci::credential_format_profiles::{CredentialFormats, WithCredential};
use serde::{Deserialize, Serialize};
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
                DisplayCredential {
                    id: Uuid::new_v4().to_string(),
                    issuer_name: None,
                    format: (&verifiable_credential).try_into().unwrap(),
                    data: credential_display,
                    metadata: CredentialMetadata::default(),
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
    pub metadata: CredentialMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialMetadata.ts")]
pub struct CredentialMetadata {
    pub is_favorite: bool,
    pub display: CredentialDisplay,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialDisplay.ts")]
pub struct CredentialDisplay {
    pub icon: Option<String>,
    pub color: Option<String>,
    pub name: Option<String>,
}
