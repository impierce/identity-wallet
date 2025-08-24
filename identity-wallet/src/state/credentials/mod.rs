pub mod actions;
pub mod reducers;

use super::{core_utils::helpers::get_unverified_jwt_claims, FeatTrait};
use crate::{error::AppError, state::core_utils::DateUtils};

use derivative::Derivative;
use identity_credential::{sd_jwt_v2::Sha256Hasher, sd_jwt_vc::SdJwtVc};
use log::info;
use oid4vc::oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_configurations_supported::IssuerMetadataClaim,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

/// A credential displayable by the frontend.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS, Default)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/DisplayCredential.ts")]
pub struct DisplayCredential {
    #[derivative(PartialEq = "ignore")]
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
    // TODO: should this be moved to `metadata`?
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
    #[derivative(PartialEq = "ignore")]
    pub date_issued: String,
    #[ts(optional)]
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VerifiableCredentialRecord {
    pub verifiable_credential: serde_json::Value,
    pub display_credential: DisplayCredential,
}

#[test]
fn test() {
    // "Verifiable ID Card (jwt_vc)"
    // "Person Identification Data"
    let credential: SdJwtVc = "eyJ0eXAiOiJ2YytzZC1qd3QiLCJraWQiOiJkaWQ6andrOmV5SnJkSGtpT2lKRlF5SXNJbmdpT2lJM2VXMUhhWEJyVEdReGIzaFNSMWxEU1dGME9EUlBjWHAxVUdaTU1GbHZUQzF5V1VGdlMzRlFhbmhySWl3aWVTSTZJa1ZLVEVRNFJHSTRPRXhRTW5Oa01raERiRlpyV25Ka2VHd3dlV2x3YlVkVlprdEdPRFZKWmxWVk5sRWlMQ0pqY25ZaU9pSlFMVEkxTmlKOSMwIiwiYWxnIjoiRVMyNTYiLCJjdHkiOiJ2YyJ9.eyJpc3MiOiJkaWQ6andrOmV5SnJkSGtpT2lKRlF5SXNJbmdpT2lJM2VXMUhhWEJyVEdReGIzaFNSMWxEU1dGME9EUlBjWHAxVUdaTU1GbHZUQzF5V1VGdlMzRlFhbmhySWl3aWVTSTZJa1ZLVEVRNFJHSTRPRXhRTW5Oa01raERiRlpyV25Ka2VHd3dlV2x3YlVkVlprdEdPRFZKWmxWVk5sRWlMQ0pqY25ZaU9pSlFMVEkxTmlKOSIsImlhdCI6MTc1NTg2OTkwMiwibmJmIjoxNzU1ODY5OTAyLCJleHAiOjE3NzE3Njc1MDIsIkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy9ucy9jcmVkZW50aWFscy92MiJdLCJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiVmVyaWZpYWJsZUlkQ2FyZEp3dFZjIl0sImNyZWRlbnRpYWxTdWJqZWN0Ijp7Il9zZCI6WyJRanBseWtud1RUTG13WXkzTTZwS09wMnF1V3E4VkRDUmI5NDV3RVU3VElFIiwiVVpDUFJMRTZWOWZibEtmSDNkRS1XN1NrQi1MUU1vZ2NFcmd3Uk5RZHJXTSIsIlduOXlLeXNqZWRJd1BESkh4dUZWelNQSGdqOXNLTGIyZGxUSGNlRG1jd1UiLCJYakFwZjQ3WWNjQWltdHVzYmxkMFhzejMtc2hnWC1oZDRiN3NGclFnWHRRIiwibFNzQ2tLU2tHU1pXUjhMd2VFZlNZU1V6QUktczNndFVCdDJydi1kemtsNCIsInFPM2xFelV4eC10SU83ZlhHS05iMi1INEJXcXFUdFJsNHBpeVBmSlpYLUEiLCJxcm1WaDBuVUtaNmlpd2gzYUo3dlc3VmVTaks3SVR6WW1UWjJuclpVQ3ZvIiwieUxfczE4WUVzQ3AxYXFYdGZJdHJHX3I1a1p0MjRLaXRvSE9LRUJQaElGVSJdfSwiaXNzdWVyIjoiZGlkOmp3azpleUpyZEhraU9pSkZReUlzSW5naU9pSTNlVzFIYVhCclRHUXhiM2hTUjFsRFNXRjBPRFJQY1hwMVVHWk1NRmx2VEMxeVdVRnZTM0ZRYW5ocklpd2llU0k2SWtWS1RFUTRSR0k0T0V4UU1uTmtNa2hEYkZaclduSmtlR3d3ZVdsd2JVZFZaa3RHT0RWSlpsVlZObEVpTENKamNuWWlPaUpRTFRJMU5pSjkiLCJ2YWxpZEZyb20iOiIyMDI1LTA4LTIyVDEzOjM4OjIyLjI4OVoiLCJ2YWxpZFVudGlsIjoiMjAyNi0wMi0yMlQxMzozODoyMi4yODlaIiwiY25mIjp7ImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGVXpJMU5pSXNJbU55ZGlJNklsQXRNalUySWl3aWEybGtJam9pWVVwSU4zZEdlalJXVEVkM1FWZERNbkIxTW1KU1FYWkdYMU15TFdOVWRFMDVYMlZSVlZwUmNGZzBRU0lzSW10MGVTSTZJa1ZESWl3aWVDSTZJbFZNUm1WS1RVaDZORTVvT1Y5WVZURktZVXc0UmpSVlkxWldOVzVuWjBadVdXZGZNelowUldSQ1JuTWlMQ0o1SWpvaVEwWnlSMmczVUcweWVrSktXa2RQYkRGRGEyTmpiREJZZFcxdVNWZFRVV05IWkVJdGVVOWhNakE0Y3lKOSMwIn0sInN0YXR1cyI6eyJzdGF0dXNfbGlzdCI6eyJ1cmkiOiJodHRwczovL2l0Yi5pbGFicy5haS9yZmMtaXNzdWVyL3N0YXR1cy1saXN0LzIxMmM3OTRiLWM0NjYtNDk5NS1hODQyLTNlY2MxNjQyMGIwNyIsImlkeCI6MH19LCJfc2RfYWxnIjoic2hhLTI1NiJ9.YlBNjBFuM6-aj0cHKJzzSWjawz8pAcIYFjkhj4Dsp1zMh8n8DvOxeITqFioEungKCcJIxnTceTWs4qJQvFx0tg~WyJmZGVkNmI5MTQ5M2U1NGVmIiwiZ2l2ZW5fbmFtZSIsIkhhbm5hIl0~WyIxMWE3YThjY2YzNGRhYWEzIiwiZmFtaWx5X25hbWUiLCJNYXRrYWxhaW5lbiJd~WyJlYzZiOGI5ZGYwNDRlOGUzIiwiYmlydGhfZGF0ZSIsIjAxLjA3LjIwMDUiXQ~WyJjN2JlN2VmNzAyZGFhZjYwIiwiYWdlX292ZXJfMTgiLHRydWVd~WyIwMTVmNDJiM2FiNzc1NzI3IiwiaXNzdWFuY2VfZGF0ZSIsMTc1NTg2OTkwMjI4OV0~WyJiYmIyNDViNTgwODU2NzRlIiwiZXhwaXJ5X2RhdGUiLDE3ODc0MDU5MDIyODld~WyJiMWMxNWRmZDhjYjU3MmQzIiwiaXNzdWluZ19hdXRob3JpdHkiLCJVQWVnZWFuIFRlc3QgSXNzdWVyIl0~WyJlMjc3ZWMxYzM3NWQ0NGViIiwiaXNzdWluZ19jb3VudHJ5IiwiRmlubGFuZCJd~".parse().unwrap();

    println!("Parsed SD-JWT VC: {:#?}", credential);
}

impl VerifiableCredentialRecord {
    pub fn new(verifiable_credential: serde_json::Value, claims: Vec<IssuerMetadataClaim>) -> Result<Self, AppError> {
        let display_credential = {
            // Try to parse the Verifiable Credential as an SD-JWT credential.
            let (format, data, issuance_date) = if let Some(sd_jwt_vc) = verifiable_credential
                .as_str()
                .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwtVc>().ok())
            {
                info!("Verifiable Credential parsed as a SD-JWT VC");

                let typ = sd_jwt_vc.header().get("typ").and_then(|typ| typ.as_str());

                info!("typ: {typ:?}");

                let issuance_date = sd_jwt_vc.claims().iat.map(|iat| iat.to_rfc3339()).unwrap_or_default();

                let mut credential_subject = serde_json::json!(sd_jwt_vc
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?);

                if let Some("vc+sd-jwt") = typ {
                    credential_subject = serde_json::json!(credential_subject
                        .get("credentialSubject")
                        .cloned()
                        .ok_or_else(|| AppError::Error("Missing credentialSubject in SD JWT VC".to_string()))?
                        .as_object()
                        .cloned()
                        .ok_or_else(|| {
                            AppError::Error("credentialSubject is not a JSON object in SD JWT VC".to_string())
                        })?);
                };

                // Remove the SD-JWT specific fields that should not be displayed in the frontend.
                for key in ["iss", "nbf", "exp", "status", "iat", "sub", "_sd_alg", "cnf", "vct"] {
                    credential_subject.as_object_mut().unwrap().remove(key);
                }

                // TODO: preserve order of the claims (this needs to be fixed in `openid4vc` first).

                // FXIME!!!!
                // Rename the keys in the Credential according to the display hints provided by the Issuer.
                for claim in claims {
                    let _ = claim.path.rename_key_in_json(
                        &mut credential_subject,
                        claim.display[0]["name"]
                            .as_str()
                            .map(ToString::to_string)
                            .unwrap_or_default(),
                    );
                }

                let format = if let Some("dc+sd-jwt") = typ {
                    CredentialFormats::DcSdJwt(())
                } else {
                    CredentialFormats::VcSdJwt(())
                };

                // TODO: Remove this workaround that is basically a way of disguising the SD JWT VC as a VC so that
                // it can be displayed in the Frontend.
                let data = json!({
                    "type": ["VerifiableCredential"],
                    "issuer": sd_jwt_vc.claims().iss,
                    "credentialSubject": credential_subject

                });

                info!("data: {data:?}");

                (format, data, issuance_date)
            } else {
                let credential_display = get_unverified_jwt_claims(&verifiable_credential)?["vc"].clone();

                let issuance_date = credential_display["issuanceDate"]
                    .as_str()
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let format = CredentialFormats::JwtVcJson(());

                // TODO: Use the claims to rename the keys in the Credential according to the display hints provided by
                // the Issuer. Before we do this we need to make sure that UniCore supports Claims Description for
                // Issuer Metadata (see: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#claims-description-issuer-metadata).
                // For now we just display the raw credential as is.

                let data = credential_display;

                (format, data, issuance_date)
            };

            DisplayCredential {
                id: Uuid::new_v4().to_string(),
                format,
                data,
                metadata: CredentialMetadata {
                    is_favorite: false,
                    date_added: DateUtils::new_date_string(),
                    date_issued: issuance_date,
                    icon: None,
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

// TODO: remove this function and find a cleaner implementation for this functionality.
impl TryFrom<serde_json::Value> for VerifiableCredentialRecord {
    type Error = AppError;

    fn try_from(verifiable_credential: serde_json::Value) -> Result<Self, AppError> {
        let display_credential = {
            // Try to parse the Verifiable Credential as an SD-JWT credential.
            let (id, format, data, issuance_date) = if let Some(sd_jwt_vc) = verifiable_credential
                .as_str()
                .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwtVc>().ok())
            {
                info!("Verifiable Credential parsed as a SD-JWT VC");

                let typ = sd_jwt_vc.header().get("typ").and_then(|typ| typ.as_str());

                info!("typ: {typ:?}");

                let issuance_date = sd_jwt_vc.claims().iat.map(|iat| iat.to_rfc3339()).unwrap_or_default();

                let mut credential_subject = sd_jwt_vc
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?;

                if let Some("vc+sd-jwt") = typ {
                    credential_subject = credential_subject
                        .get("credentialSubject")
                        .cloned()
                        .ok_or_else(|| AppError::Error("Missing credentialSubject in SD JWT VC".to_string()))?
                        .as_object()
                        .cloned()
                        .ok_or_else(|| {
                            AppError::Error("credentialSubject is not a JSON object in SD JWT VC".to_string())
                        })?;
                };

                // Remove the SD-JWT specific fields that should not be displayed in the frontend.
                for key in ["iss", "nbf", "exp", "status", "iat", "sub", "_sd_alg", "cnf", "vct"] {
                    credential_subject.remove(key);
                }

                // TODO: We are using this hash as Credential ID so that we can prevent credential duplication in
                // demo situations. Now we can actually delete Credentials in UniMe we don't need to use the hash of the
                // credential as the ID anymore. We should simply generate a random UUID.
                let hash = sha256::digest(json!(credential_subject).to_string());

                let id = Uuid::from_slice(&hash.as_bytes()[..16])?.to_string();

                let format = CredentialFormats::DcSdJwt(());
                // TODO: Remove this workaround that is basically a way of disguising the SD JWT VC as a VC so that
                // it can be displayed in the Frontend.
                let data = json!({
                    "type": ["VerifiableCredential"],
                    "issuer": sd_jwt_vc.claims().iss,
                    "credentialSubject": credential_subject

                });

                info!("data: {data:?}");

                (id, format, data, issuance_date)
            } else {
                let credential_display = get_unverified_jwt_claims(&verifiable_credential)?["vc"].clone();

                let issuance_date = credential_display["issuanceDate"]
                    .as_str()
                    .map(ToString::to_string)
                    .unwrap_or_default();
                let id = Uuid::new_v4().to_string();
                let format = CredentialFormats::JwtVcJson(());
                let data = credential_display;

                (id, format, data, issuance_date)
            };

            DisplayCredential {
                id,
                format,
                data,
                metadata: CredentialMetadata {
                    is_favorite: false,
                    date_added: DateUtils::new_date_string(),
                    date_issued: issuance_date,
                    icon: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verifiable_credential_record_try_from_jwt_vc_json() {
        let jwt_vc_json = json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw");

        let verifiable_credential_record = VerifiableCredentialRecord::try_from(jwt_vc_json).unwrap();

        assert_eq!(
            verifiable_credential_record.display_credential.format,
            CredentialFormats::JwtVcJson(())
        );

        assert_eq!(
            verifiable_credential_record.display_credential.data,
            json!({
              "@context": [
                "https://www.w3.org/2018/credentials/v1",
                "https://www.w3.org/2018/credentials/examples/v1"
              ],
              "type": [
                "VerifiableCredential",
                "PersonalInformation"
              ],
              "issuanceDate": "2022-01-01T00:00:00Z",
              "issuer": "http://192.168.1.127:9090/",
              "credentialSubject": {
                "id": "did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY",
                "givenName": "Ferris",
                "familyName": "Crabman",
                "email": "ferris.crabman@crabmail.com",
                "birthdate": "1985-05-21"
              }
            })
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2022-01-01T00:00:00Z"
        );
    }

    #[test]
    fn test_verifiable_credential_record_try_from_sd_jwt_vc() {
        let sd_jwt_vc = json!("eyJjdHkiOiJjcmVkZW50aWFsLWNsYWltcy1zZXQranNvbiIsInR5cCI6InZjK3NkLWp3dCIsImFsZyI6IkVTMjU2Iiwia2lkIjoiZGlkOndlYjpkZW1vcGlkcHJvdmlkZXIuYWNjLmNyZWRlbmNvLmNvbTpkaWQ6YjQ5YmMzN2MtNmUyNC00NDJhLTk1OTctMDY5Y2RiYzk0NmJkI3pEdlRSU0xBSzNzOGNGeGZoTEQzdUxrWHBmbmMzUWg2czFGdGdSX3RWX2cifQ.eyJ2Y3QiOiJwaWRfdmMrc2Qtand0IiwiaWQiOiJ1cm46dXVpZDpkMDY5NmM2MC0zNGJkLTQyZWMtYjFkYi04NmYxYTY4ZTk0ODciLCJpc3MiOiJkaWQ6d2ViOmRlbW9waWRwcm92aWRlci5hY2MuY3JlZGVuY28uY29tOmRpZDpiNDliYzM3Yy02ZTI0LTQ0MmEtOTU5Ny0wNjljZGJjOTQ2YmQiLCJpYXQiOjE3NDExMTc3MzEsIm5iZiI6MTc0MTExNzczMSwiZXhwIjoxNzcyNjUzNzMxLCJfc2QiOlsidjB5SDh4bERYRWdyNjVWY01nUnA1YkpzWFNDbFlNbGsxSXBHSkRod0hpRSIsImtSaXJDM3c3cklnNkNGTnBhbkdMWkQ1WHpibnpZVEdHMnZqNXd2dnFfMU0iLCJrQWRpeDZ5ZTBpSU9hS2w4QUlRczlQVHduR1JicTVVcHBDejA5MGRQVTNvIiwiRHNSMHNUdkpBamF3bEFsZ083cDhudmFsWHRFVWIyX0xQcG1KX2R5SWRUOCIsIlN2Tl80U1pKQ0xic29WdXFJaG0yNHIzSk1OZU1jT19JUFNxUmZxbGxjMTAiLCJNaElvT2lBTkl2RHQtV29nQTdueExOZGlsUTJxb19sOVFld2pIb05WekdFIiwieGJYRWs4OW1BNkRabWJfSmU3b3NPdk1CSjkwSGpxZ0dra2N0SldkNW9NQSIsIjlCWFBSYzRHYVBwaUNnSUlqWTRWRnFVTVo0SnFTNXBNRDMya1E1RHFBbzgiLCJfQ3I2c3RlOHBLLWliaUJScWtxRkF1YVl2cmNRMG5fZEd0SXplc3AyWl9BIiwiMHlRRUI2anpQS0llU1cxYW1zeUZBZEM2bU5EUl8zUHdfZWduTDZUVDlHYyIsIjJ6NHdQMU1DdFdObDFIUjRJUWlHWnhKallfZkQzcDVWSkx4YlR1UXR1UXMiLCI3X01LdDFXTndxbWh3THRtNnZKeHRaVnNjc01JNlBmelM2UldibkRxZDZrIiwickJVeFZqZ2tpcWFmQVp2RHV4ZXZzV05QSzdNZ0ZiMHRBTXBMVENIQ0VYRSIsImFuSjJlZjlESzZ0Y0pBT2NYdFZ1RENjOFJTVHdtUmpCY1ZpeHluUzBQMlkiLCJNX25aeUNyNmtSeTVKaDc3dmlCNXMwWUNZa1pvRktyZElLdHNqX2d6RnBFIiwiRVNWb25ZR2N1SEdGZEFtOERVdXc0dXUyVHRMX3NGNHlRWnVCSTlXR0gzOCIsInpMSnFzbnAySzNJTkVMbXBSX20welE5eE5zVTRJNjJfTDlIRWpqVW1XMU0iLCJ4MUxiMlhPMmRBYkJKdFgtNWR3WjFwVW5DSGhpZFdRcmc3RVN1Z1ljYzlRIiwiVFdnM1Y1M3VmakRqZnBzN05JQzkxYXFuZnlqNFFaWGNMdk94cVE3ZThnQSIsIl9KMW11MnhHemg1d3ZFSXJhVWt4ckxXbzdtWlplZ0JScUZVN2ZxQmpPM2MiLCJoc3l4ZVJQeFdINXlRbXJ2dmFTRDhZTHlPSko2Rm1tbXltMElhQkpNWEh3IiwiUGxqY0hvTWFMYktJdEpDYk9PdFJCS0tmREVMTTlqeG9xTC1WUUgyeV84WSIsIk5pTEFBY3gzTXVUNTVqSTUyNnNYWUdlOUpMbnY3cC1nOC1SSEM4SW1YM1UiLCI5dURFWGI4SUNlWG05X2ZETlp3ZVBnc05EbEdEUnVwT1pDc2NBTGl5M3pRIiwiMERSM3FPeDVQYWx4c2lHcEZwdzFPdENYd0l6X2h3V3NBaVpsR3pEYUdnayIsImRvdGZCdnJJTmp2RUcxM1d4OE53c1I0VDJwdzU2VzljNkN0OVVaNVA5c28iLCJFd3lyRDNPaDhlcm55VmVWUk5rbTRITnRmdmhXenY0MERYV0VBc0hyb0FBIl19.aLXleOiiyzvWxTc1Wp0tSKRIm3IZ34wv_nJSeATUqgtwazK64AADus3TQW46PSfcn0i0dsvIneXr6ihSn-mKmw~WyJkOWtKZUl6NVBYSk1QdlNJbl9mblNRPT0iLCJmYW1pbHlfbmFtZSIsIlZpc3NlciJd~WyJ4WGJaZDRlc20xZk1pMzgzaTBQQWF3PT0iLCJnaXZlbl9uYW1lIiwiQ2hhcmxvdHRlIl0~WyIzUmx4T3I4RjVqcGlfNmdVRkljOUlRPT0iLCJiaXJ0aF9kYXRlIiwiMTk5MC0wNy0yMiJd~WyJkX1kybV9hRU44RkhUYTJVRVdLRTN3PT0iLCJmYW1pbHlfbmFtZV9iaXJ0aCIsIlZpc3NlciJd~WyJfenByV3haUGE0VzBENzFLeVp2aTZRPT0iLCJnaXZlbl9uYW1lX2JpcnRoIiwiQ2hhcmxvdHRlIl0~WyIzamx0TFkyZUswaW1QUWI5N2hkX1h3PT0iLCJiaXJ0aF9wbGFjZSIsIlV0cmVjaHQiXQ~WyJxTDlMWlJLVG5RX1JVNEFPQWlGSnlBPT0iLCJiaXJ0aF9jb3VudHJ5IiwiTmV0aGVybGFuZHMiXQ~WyIyV0RfMlZDTmhZenl2Wm5OckE2Tk5nPT0iLCJiaXJ0aF9zdGF0ZSIsIlV0cmVjaHQiXQ~WyJzajQ4TjZMNE9tdmtINVl5OWxXbVJnPT0iLCJiaXJ0aF9jaXR5IiwiVXRyZWNodCJd~WyJHOWNheFU4Q0l3NEtEUXBWaGd3VndBPT0iLCJyZXNpZGVudF9hZGRyZXNzIiwiT3VkZWdyYWNodCA0NTYiXQ~WyJEUlpucHNaVjRfRzF3dEhKTU4yNFlBPT0iLCJyZXNpZGVudF9jb3VudHJ5IiwiTmV0aGVybGFuZHMiXQ~WyJkOThzdUFiQV9OaHFVY2pxRWtFOFhRPT0iLCJyZXNpZGVudF9zdGF0ZSIsIlV0cmVjaHQiXQ~WyJYb09HU1E5UXNzQ2gtaEFCVXh1VWZRPT0iLCJyZXNpZGVudF9jaXR5IiwiVXRyZWNodCJd~WyJlSGNxRkhYQUgya0JMWkdObndrdWJBPT0iLCJyZXNpZGVudF9wb3N0YWxfY29kZSIsIjM1MTEgQVMiXQ~WyJ0dDJuakdpRm5yNm91WUg0TTJsN3dBPT0iLCJyZXNpZGVudF9zdHJlZXQiLCJPdWRlZ3JhY2h0Il0~WyI5WndPcmUxTUVhaGk3SlkxNTg4eGNnPT0iLCJyZXNpZGVudF9ob3VzZV9udW1iZXIiLCI0NTYiXQ~WyJLWFRpaVg0bC1EZjJBMk4zdTlnNnRnPT0iLCJnZW5kZXIiLCJGZW1hbGUiXQ~WyJzMzFjLXd4MHMwZ0xoMXhIV3N3NFJnPT0iLCJuYXRpb25hbGl0eSIsIkR1dGNoIl0~WyI3bDRKNDNGU1lTUi1Db3B0YkJVeS1RPT0iLCJkb2N1bWVudF9udW1iZXIiLCJERUY3ODkwMTIiXQ~WyJ4RnZWTENXcFdyMnBCUmdWd2dZY09BPT0iLCJhZG1pbmlzdHJhdGl2ZV9udW1iZXIiLCIxMjAwMjEzNDEyIl0~WyI0NWNoVDRXdzlmaVNrYlFFWndFX3h3PT0iLCJpc3N1aW5nX2NvdW50cnkiLCJOZXRoZXJsYW5kcyJd~WyItajU4aGtpZ2FIb3JUellVSTd2cnh3PT0iLCJpc3N1aW5nX2p1cmlzZGljdGlvbiIsIk5MLVVUIl0~WyJVNllKcDAyanpFY2EyM0FiS3pTMDRRPT0iLCJhZ2Vfb3Zlcl8xOCIsdHJ1ZV0~WyJOV0R5bU1feno3ckRXUi1VRWdOZi13PT0iLCJhZ2Vfb3Zlcl8yMSIsdHJ1ZV0~WyJnNXdnZTY2MlpSMjIxSjI1OGlaNlpnPT0iLCJhZ2VfaW5feWVhcnMiLDMxXQ~WyJuSEFCVGxsSW45OXhOYXZyNUVETFhnPT0iLCJhZ2VfYmlydGhfeWVhciIsMTk5MF0~WyJMMWVpdnByS2hSRmdGUjVaS052RDVRPT0iLCJzdWIiLCJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyYVdRaU9pSlRTbTl2UW5RNFpVdFhVamhZZWpkeE5FUXlWMFZrTW1WVFowNXhhVGgwVFRGbk5HazROR0pRVVZOaklpd2lhM1I1SWpvaVQwdFFJaXdpZUNJNkltTjBUVEZrT1RoWFRsUmxabk4zTFZCcWJYcERXV0ZQWmtGc1dsbFVMVWxLWlZGV1kySkVhV3RqV2pBaWZRIl0~");

        let verifiable_credential_record = VerifiableCredentialRecord::try_from(sd_jwt_vc).unwrap();

        assert_eq!(
            verifiable_credential_record.display_credential.format,
            CredentialFormats::DcSdJwt(())
        );

        assert_eq!(
            verifiable_credential_record.display_credential.data,
            json!({
              "type": [
                "VerifiableCredential"
              ],
              "issuer": "did:web:demopidprovider.acc.credenco.com:did:b49bc37c-6e24-442a-9597-069cdbc946bd",
              "credentialSubject": {
                "id": "urn:uuid:d0696c60-34bd-42ec-b1db-86f1a68e9487",
                "family_name": "Visser",
                "given_name": "Charlotte",
                "birth_date": "1990-07-22",
                "family_name_birth": "Visser",
                "given_name_birth": "Charlotte",
                "birth_place": "Utrecht",
                "birth_country": "Netherlands",
                "birth_state": "Utrecht",
                "birth_city": "Utrecht",
                "resident_address": "Oudegracht 456",
                "resident_country": "Netherlands",
                "resident_state": "Utrecht",
                "resident_city": "Utrecht",
                "resident_postal_code": "3511 AS",
                "resident_street": "Oudegracht",
                "resident_house_number": "456",
                "gender": "Female",
                "nationality": "Dutch",
                "document_number": "DEF789012",
                "administrative_number": "1200213412",
                "issuing_country": "Netherlands",
                "issuing_jurisdiction": "NL-UT",
                "age_over_18": true,
                "age_over_21": true,
                "age_in_years": 31,
                "age_birth_year": 1990
              }
            })
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2025-03-04T19:48:51Z"
        );
    }
}
