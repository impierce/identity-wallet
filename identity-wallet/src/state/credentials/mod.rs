pub mod credential_offers_selected;
pub mod credentials_selected;
pub mod update_credential_metadata;
pub mod verifiable_credential_record;

pub use verifiable_credential_record::VerifiableCredentialRecord;

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
    /// This id is also used as the image asset id
    pub id: String,
    pub issuer_name: String,
    #[ts(type = "string")]
    pub format: CredentialFormats,
    #[ts(type = "any")]
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: CredentialMetadata,

    pub display_name: String,
    pub display_color: Option<String>,
    pub display_icon: Option<String>,
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
}
