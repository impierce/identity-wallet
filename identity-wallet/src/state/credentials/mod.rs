use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use serde::{Deserialize, Serialize};
use derivative::Derivative;
use ts_rs::TS;
use super::FeatTrait;

pub mod actions;
pub mod reducers;

/// A displayable credential usable for the frontend.
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
}

#[typetag::serde(name = "display_credential")]
impl FeatTrait for DisplayCredential {}

/// Metadata for a credential.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, TS, Default, Derivative)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/display-credential/CredentialMetadata.ts")]
pub struct CredentialMetadata {
    pub is_favorite: bool,
    #[derivative(PartialEq = "ignore")]
    pub date_added: String,
    #[derivative(PartialEq = "ignore")]
    pub date_issued: String,
    pub display: CredentialDisplay,
}

/// Display information for a credential.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialDisplay.ts")]
pub struct CredentialDisplay {
    pub icon: Option<String>,
    pub color: Option<String>,
    pub name: Option<String>,
}
