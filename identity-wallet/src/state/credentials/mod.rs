pub mod actions;
pub mod reducers;

use super::FeatTrait;

use derivative::Derivative;
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// A credential displayable by the frontend.
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

/// Contains metadata about a credential.
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

/// Info concerning the display of a credential.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/display-credential/CredentialDisplay.ts")]
pub struct CredentialDisplay {
    pub icon: Option<String>,
    pub color: Option<String>,
    pub name: Option<String>,
}
