pub mod actions;
pub mod reducers;

use std::str::FromStr;

use super::{core_utils::helpers::get_unverified_jwt_claims, FeatTrait};
use crate::{error::AppError, state::core_utils::DateUtils};

use derivative::Derivative;
use identity_credential::{sd_jwt_v2::Sha256Hasher, sd_jwt_vc::SdJwtVc};
use log::info;
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

/// A credential displayable by the frontend.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS, Default)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/DisplayCredential.ts")]
pub struct DisplayCredential {
    pub id: String,
    #[ts(type = "string")]
    pub format: CredentialFormats,
    pub issuer_name: String,
    #[ts(type = "any")]
    pub data: serde_json::Value,
    #[serde(default)]
    pub metadata: CredentialMetadata,
    #[ts(optional)]
    pub connection_id: Option<String>,
    pub display_name: String,
}

#[typetag::serde(name = "display_credential")]
impl FeatTrait for DisplayCredential {}

/// Contains metadata about a credential.
/// PartialEq(ignore) used on the date_added field implemented because this would make testing with static json files impossible.
/// The date_added field is defined the moment the test is run and the json files are predefined.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, TS, Default, Derivative)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/CredentialMetadata.ts")]
pub struct CredentialMetadata {
    pub is_favorite: bool,
    #[derivative(PartialEq = "ignore")]
    pub date_added: String,
    pub date_issued: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VerifiableCredentialRecord {
    pub verifiable_credential: serde_json::Value,
    pub display_credential: DisplayCredential,
}

// TODO: remove this function and find a cleaner implementation for this functionality.
impl TryFrom<serde_json::Value> for VerifiableCredentialRecord {
    type Error = AppError;

    fn try_from(verifiable_credential: serde_json::Value) -> Result<Self, AppError> {
        let display_credential = {
            // FIX THIS
            let (id, format, data, issuance_date) = if let Ok(sd_jwt_vc) = SdJwtVc::from_str(
                verifiable_credential
                    .as_str()
                    .ok_or(AppError::Error("Credential is not a valid JWT".to_string()))?,
            ) {
                info!("sd_jwt_vc: {:#?}", sd_jwt_vc);

                let issuance_date = sd_jwt_vc.claims().iat.map(|iat| iat.to_rfc3339()).unwrap_or_default();
                let credential_subject = sd_jwt_vc
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?;

                // TODO: We are using this hash as Credential ID so that we can prevent credential duplication in
                // demo situations. Now we can actually delete Credentials in UniMe we don't need to use the hash of the
                // credential as the ID anymore. We should simply generate a random UUID.
                let hash = sha256::digest(json!(credential_subject).to_string());

                let id = Uuid::from_slice(&hash.as_bytes()[..16]).unwrap().to_string();
                let format = CredentialFormats::VcSdJwt(());
                // TODO: Remove this workaround that is basically a way of disguising the SD JWT VC as a VC so that
                // it can be displayed in the Frontend.
                let data = json!({
                    "type": ["VerifiableCredential"],
                    "issuer": sd_jwt_vc.claims().iss,
                    "credentialSubject": credential_subject

                });

                (id, format, data, issuance_date)
            } else {
                let credential_display = get_unverified_jwt_claims(&verifiable_credential)?["vc"].clone();

                // TODO: We are using this hash as Credential ID so that we can prevent credential duplication in
                // demo situations. Now we can actually delete Credentials in UniMe we don't need to use the hash of the
                // credential as the ID anymore. We should simply generate a random UUID.
                // Derive the hash from the credential display.
                let hash = {
                    let type_value = credential_display["type"].clone();

                    let credential_subject_key = "credentialSubject";
                    let mut credential_subject_value = credential_display[credential_subject_key].clone();

                    // TODO(ngdil): Remove this hard-coded logic.
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

                let issuance_date = credential_display["issuanceDate"]
                    .as_str()
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let id = Uuid::from_slice(&hash.as_bytes()[..16]).unwrap().to_string();
                let format = CredentialFormats::JwtVcJson(());
                let data = credential_display;

                (id, format, data, issuance_date)
            };

            info!(
                "Credential Display: {:#?}",
                serde_json::to_string_pretty(&data).unwrap()
            );

            DisplayCredential {
                id,
                format,
                data,
                metadata: CredentialMetadata {
                    is_favorite: false,
                    date_added: DateUtils::new_date_string(),
                    date_issued: issuance_date,
                },
                // The other fields will be filled in at a later stage.
                ..Default::default()
            }
        };

        Ok(Self {
            verifiable_credential,
            display_credential,
        })
    }
}
