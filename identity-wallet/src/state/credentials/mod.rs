pub mod actions;
pub mod reducers;

use super::{core_utils::helpers::get_unverified_jwt_claims, FeatTrait};
use crate::{error::AppError, state::core_utils::DateUtils};
use derivative::Derivative;
use identity_credential::{sd_jwt_v2::Sha256Hasher, sd_jwt_vc::SdJwtVc};
use log::info;
use oid4vc::{
    oid4vc_core::claim_path_pointer::ClaimPathPointer,
    oid4vci::{
        credential_format_profiles::CredentialFormats,
        credential_issuer::credential_configurations_supported::ClaimDescription,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS)]
#[ts(export, export_to = "bindings/credentials/DisplayClaim.ts")]
pub struct DisplayClaim {
    #[ts(type = "Array<string>")]
    pub path: ClaimPathPointer,
    pub key: String,
    #[ts(type = "any")]
    pub value: serde_json::Value,
}

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
    // TODO: Remove this field?
    #[ts(type = "any")]
    pub data: serde_json::Value,
    // TODO: change this to `HashMap<Locale, Vec<DisplayClaim>>` to support multiple locales.
    #[serde(default)]
    pub display_claims: Vec<DisplayClaim>,
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

impl VerifiableCredentialRecord {
    pub fn try_new(
        verifiable_credential: serde_json::Value,
        claim_descriptions: Vec<ClaimDescription>,
    ) -> Result<Self, AppError> {
        let display_credential = {
            // Try to parse the Verifiable Credential as an SD-JWT credential.
            let (id, format, data, issuance_date, display_claims) = if let Some(sd_jwt_vc) = verifiable_credential
                .as_str()
                .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwtVc>().ok())
            {
                info!("Verifiable Credential parsed as a SD-JWT VC");

                let typ = sd_jwt_vc.header().get("typ").and_then(|typ| typ.as_str());

                let id = Uuid::new_v4().to_string();
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

                let display_claims: Vec<DisplayClaim> = claim_descriptions
                    .into_iter()
                    .map(|claim_description| {
                        // FIXME
                        let key = claim_description.display[0].name.clone();
                        let value = claim_description
                            .path
                            .get_values_from_json(&json!(credential_subject))
                            .first()
                            .expect("FIXME")
                            .clone()
                            .clone();

                        DisplayClaim {
                            path: claim_description.path,
                            key,
                            value,
                        }
                    })
                    .collect();

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

                (id, format, data, issuance_date, display_claims)
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

                // TODO: Use the claims to rename the keys in the Credential according to the display hints provided by
                // the Issuer. Before we do this we need to make sure that UniCore supports Claims Description for
                // Issuer Metadata (see: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#claims-description-issuer-metadata).
                // For now we just display the raw credential as is.

                let data = credential_display;

                // FIXME!!: empty vec!
                (id, format, data, issuance_date, vec![])
            };

            DisplayCredential {
                id,
                format,
                data,
                display_claims,
                metadata: CredentialMetadata {
                    is_favorite: false,
                    date_added: DateUtils::new_date_string(),
                    date_issued: issuance_date,
                    icon: None,
                },
                // The other fields will be filled in at a later stage.
                issuer_name: String::new(),
                connection_id: None,
                display_name: String::new(),
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
    use oid4vc::{
        oid4vc_core::claim_path_pointer::ClaimPathElement,
        oid4vci::credential_issuer::credential_configurations_supported::ClaimDescriptionDisplay,
    };

    #[test]
    fn test_verifiable_credential_record_try_from_jwt_vc_json() {
        let jwt_vc_json = json!("eyJ0eXAiOiJKV1QiLCJhbGciOiJFZERTQSIsImtpZCI6ImRpZDprZXk6ejZNa2toUDQzTENTWGFqM1NRQm92eTF1RTJuWHZTQm5SUFdaMndoUExxblo4UGdEI3o2TWtraFA0M0xDU1hhajNTUUJvdnkxdUUyblh2U0JuUlBXWjJ3aFBMcW5aOFBnRCJ9.eyJpc3MiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsInN1YiI6ImRpZDprZXk6ejZNa2cxWFhHVXFma2hBS1Uxa1ZkMVBtdzZVRWoxdnhpTGoxeGM5MU1CejVvd05ZIiwiZXhwIjo5OTk5OTk5OTk5LCJpYXQiOjAsInZjIjp7IkBjb250ZXh0IjpbImh0dHBzOi8vd3d3LnczLm9yZy8yMDE4L2NyZWRlbnRpYWxzL3YxIiwiaHR0cHM6Ly93d3cudzMub3JnLzIwMTgvY3JlZGVudGlhbHMvZXhhbXBsZXMvdjEiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCIsIlBlcnNvbmFsSW5mb3JtYXRpb24iXSwiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoiLCJpc3N1ZXIiOiJodHRwOi8vMTkyLjE2OC4xLjEyNzo5MDkwLyIsImNyZWRlbnRpYWxTdWJqZWN0Ijp7ImlkIjoiZGlkOmtleTp6Nk1rZzFYWEdVcWZraEFLVTFrVmQxUG13NlVFajF2eGlMajF4YzkxTUJ6NW93TlkiLCJnaXZlbk5hbWUiOiJGZXJyaXMiLCJmYW1pbHlOYW1lIjoiQ3JhYm1hbiIsImVtYWlsIjoiZmVycmlzLmNyYWJtYW5AY3JhYm1haWwuY29tIiwiYmlydGhkYXRlIjoiMTk4NS0wNS0yMSJ9fX0.Yl841U5BwWgctX5vF5Zi8SYCEQpxFqEs8_J8KrX9D_mOwL-IRmP64BeQZvnKeAdcOoYGn6CyciV51_amdPNQBw");

        let verifiable_credential_record = VerifiableCredentialRecord::try_new(jwt_vc_json, vec![]).unwrap();

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
        let sd_jwt_vc = json!("eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyYVdRaU9pSkdRVFl4Y3pFMmFIVk5NVFZuVm14a09XaHNNbll6TTB4cWN6QjVRVUYxVGxwZmJFZGFkbEZGWHpFNElpd2lhM1I1SWpvaVQwdFFJaXdpZUNJNklrTlpSakI2U2trMVdtZE9OVkZpZG1obFRteFVVMmhKVTJoc2J6WmpZbVJzYXpWM1VXVTRNakJGWWxraWZRIzAiLCJ0eXAiOiJkYytzZC1qd3QiLCJhbGciOiJFZERTQSJ9.eyJ0eXBlIjpbIlZlcmlmaWFibGVDcmVkZW50aWFsIiwiTmF0dXJhbFBlcnNvbkNyZWRlbnRpYWwiXSwiaXNzdWFuY2VEYXRlIjoiMjAyNS0wOC0yOVQxOTozOTowNi4xMjFaIiwibmFtZSI6Ik15IFByb2ZpbGUiLCJjcmVkZW50aWFsU3ViamVjdCI6eyJnaXZlbl9uYW1lIjoiRmVycmlzIiwiZmFtaWx5X25hbWUiOiJSdXN0YWNlYW4iLCJiaXJ0aF9kYXRlIjoiMjAyMy0wNC0wMVQwMDowMDowMC4wMDBaIiwiYmlydGhfcGxhY2UiOiJBdGxhbnRpYyBPY2VhbiIsIm5hdGlvbmFsaXR5IjpbIk5MIl19LCJpc3N1ZXIiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlpFUlRRU0lzSW1OeWRpSTZJa1ZrTWpVMU1Ua2lMQ0pyYVdRaU9pSkdRVFl4Y3pFMmFIVk5NVFZuVm14a09XaHNNbll6TTB4cWN6QjVRVUYxVGxwZmJFZGFkbEZGWHpFNElpd2lhM1I1SWpvaVQwdFFJaXdpZUNJNklrTlpSakI2U2trMVdtZE9OVkZpZG1obFRteFVVMmhKVTJoc2J6WmpZbVJzYXpWM1VXVTRNakJGWWxraWZRIiwiaXNzIjoiZGlkOmp3azpleUpoYkdjaU9pSkZaRVJUUVNJc0ltTnlkaUk2SWtWa01qVTFNVGtpTENKcmFXUWlPaUpHUVRZeGN6RTJhSFZOTVRWblZteGtPV2hzTW5Zek0weHFjekI1UVVGMVRscGZiRWRhZGxGRlh6RTRJaXdpYTNSNUlqb2lUMHRRSWl3aWVDSTZJa05aUmpCNlNrazFXbWRPTlZGaWRtaGxUbXhVVTJoSlUyaHNielpqWW1Sc2F6VjNVV1U0TWpCRllsa2lmUSIsImlhdCI6MTc1NjQ5NjM0NiwidmN0IjoiaHR0cHM6Ly93d3cuaWV0Zi5vcmcvYXJjaGl2ZS9pZC9kcmFmdC10ZXJidS1vYXV0aC1zZC1qd3QtdmMtMDAuaHRtbCIsIl9zZF9hbGciOiJzaGEtMjU2IiwiY25mIjp7ImtpZCI6ImRpZDpqd2s6ZXlKaGJHY2lPaUpGWkVSVFFTSXNJbU55ZGlJNklrVmtNalUxTVRraUxDSnJhV1FpT2lKR1FUWXhjekUyYUhWTk1UVm5WbXhrT1doc01uWXpNMHhxY3pCNVFVRjFUbHBmYkVkYWRsRkZYekU0SWl3aWEzUjVJam9pVDB0UUlpd2llQ0k2SWtOWlJqQjZTa2sxV21kT05WRmlkbWhsVG14VVUyaEpVMmhzYnpaalltUnNhelYzVVdVNE1qQkZZbGtpZlEjMCJ9fQ.7go3pFdLUK4DpKUWydPrtgDjShwz9UTdHGCjJUG2o8Lzs4DFtd8pzTVghaVyVnWITPwPyffemG4P5uCKMmdUDQ~");

        let verifiable_credential_record = VerifiableCredentialRecord::try_new(
            sd_jwt_vc,
            vec![ClaimDescription {
                path: ClaimPathPointer::try_new(vec![
                    ClaimPathElement::String("credentialSubject".to_string()),
                    ClaimPathElement::String("given_name".to_string()),
                ])
                .unwrap(),
                mandatory: true,
                display: vec![ClaimDescriptionDisplay {
                    name: "Given Name".to_string(),
                    locale: None,
                }],
            }],
        )
        .unwrap();

        assert_eq!(
            verifiable_credential_record.display_credential.format,
            CredentialFormats::DcSdJwt(())
        );

        assert_eq!(
            verifiable_credential_record.display_credential.display_claims,
            vec![DisplayClaim {
                path: ClaimPathPointer::try_new(vec![
                    ClaimPathElement::String("credentialSubject".to_string()),
                    ClaimPathElement::String("given_name".to_string())
                ])
                .unwrap(),
                key: "Given Name".to_string(),
                value: json!("Ferris"),
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2025-08-29T19:39:06Z"
        );
    }
}
