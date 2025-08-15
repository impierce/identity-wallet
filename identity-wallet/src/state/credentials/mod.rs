pub mod actions;
pub mod reducers;

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

                let issuance_date = sd_jwt_vc.claims().iat.map(|iat| iat.to_rfc3339()).unwrap_or_default();
                let mut credential_subject = sd_jwt_vc
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?;

                // Remove the SD-JWT specific fields that should not be displayed in the frontend.
                for key in ["iss", "nbf", "exp", "status", "iat", "sub", "_sd_alg", "cnf", "vct"] {
                    credential_subject.remove(key);
                }

                // TODO: We are using this hash as Credential ID so that we can prevent credential duplication in
                // demo situations. Now we can actually delete Credentials in UniMe we don't need to use the hash of the
                // credential as the ID anymore. We should simply generate a random UUID.
                let hash = sha256::digest(json!(credential_subject).to_string());

                let id = Uuid::from_slice(&hash.as_bytes()[..16])?.to_string();

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

                    let mut credential_subject_value = credential_display["credentialSubject"].clone();

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
                let id = Uuid::from_slice(&hash.as_bytes()[..16])?.to_string();
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
            CredentialFormats::VcSdJwt(())
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
