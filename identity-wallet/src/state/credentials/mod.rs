pub mod actions;
pub mod create_public_link;
pub mod reducers;
use super::{core_utils::helpers::get_unverified_jwt_claims, FeatTrait};
use crate::{error::AppError, state::core_utils::DateUtils};
use derivative::Derivative;
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_iota::{
    core::{FromJson as _, Object},
    credential::CredentialV2,
};
use oauth_tsl::status_list::StatusType;
use oid4vc::{
    oid4vc_core::claim_path_pointer::ClaimPathPointer,
    oid4vci::{
        credential_format_profiles::CredentialFormats,
        credential_issuer::credential_configurations_supported::ClaimDescription,
    },
};
use sd_jwt::{SdJwt, Sha256Hasher};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use url::Url;
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
    #[ts(type = "{ format: string }")]
    pub format: CredentialFormats,
    pub issuer_name: String,
    // TODO: Remove this field once we fully implemented `display_claims` for all credential formats.
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
    #[ts(optional)]
    pub credential_status: Option<CredentialStatus>,
    pub public_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/CredentialStatus.ts")]
pub struct CredentialStatus {
    #[ts(type = "'VALID' | 'INVALID' | 'SUSPENDED' | 'UNDEFINED'")]
    pub status: StatusType,
    #[ts(skip)]
    pub idx: u64,
    #[ts(skip)]
    pub uri: Url,
    #[derivative(PartialEq = "ignore")]
    pub last_checked: String,
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
    #[derivative(PartialEq = "ignore")]
    pub expiration_date: Option<String>,
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
        format: CredentialFormats,
        verifiable_credential: serde_json::Value,
        claim_descriptions: Vec<ClaimDescription>,
    ) -> Result<Self, AppError> {
        let display_credential = {
            let (id, data, issuance_date, expiration_date, display_claims) = match format {
                CredentialFormats::DcSdJwt(()) => {
                    let sd_jwt_vc = verifiable_credential
                        .as_str()
                        .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwtVc>().ok())
                        .ok_or(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: provided verifiable credential is not a valid SD-JWT VC"
                                .to_string(),
                        ))?;

                    let id = Uuid::new_v4().to_string();
                    let issuance_date = sd_jwt_vc.claims().iat.map(|iat| iat.to_rfc3339()).unwrap_or_default();
                    let expiration_date = sd_jwt_vc.claims().exp.map(|exp| exp.to_rfc3339());

                    if sd_jwt_vc.headers().get("typ").and_then(|typ| typ.as_str()) != Some("dc+sd-jwt") {
                        return Err(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: SD-JWT 'typ' header is not 'dc+sd-jwt'"
                                .to_string(),
                        ));
                    }

                    let credential_subject = serde_json::json!(sd_jwt_vc
                        .clone()
                        .into_disclosed_object(&Sha256Hasher::new())
                        .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?);

                    let display_claims: Vec<DisplayClaim> =
                        get_display_claims(claim_descriptions, &json!(credential_subject));

                    // TODO: Remove this workaround that is basically a way of disguising the SD JWT VC as a VC so that
                    // it can be displayed in the Frontend.
                    // TODO: moreover, this workaround is incomplete, since a few fields at the root level are still missing.
                    // Most importantly, we're missing the credentialStatus property.
                    let data = json!({
                        "type": ["VerifiableCredential"],
                        "issuer": sd_jwt_vc.claims().iss,
                        "credentialSubject": credential_subject
                    });

                    (id, data, issuance_date, expiration_date, display_claims)
                }
                CredentialFormats::VcSdJwt(()) => {
                    let vcdm2_sd_jwt = verifiable_credential
                        .as_str()
                        .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwt>().ok())
                        .ok_or(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: provided verifiable credential is not a valid SD-JWT".to_string(),
                        ))?;

                    let id = Uuid::new_v4().to_string();
                    let issuance_date = vcdm2_sd_jwt
                        .claims()
                        .get("validFrom")
                        .and_then(|valid_from| valid_from.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let expiration_date = vcdm2_sd_jwt
                        .claims()
                        .get("validUntil")
                        .and_then(|valid_until| valid_until.as_str().map(ToString::to_string)); // TODO: import this from UniCore

                    if vcdm2_sd_jwt.headers().get("typ").and_then(|typ| typ.as_str()) != Some("vc+sd-jwt") {
                        return Err(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: SD-JWT 'typ' header is not 'vc+sd-jwt'"
                                .to_string(),
                        ));
                    }

                    let disclosed_claims = vcdm2_sd_jwt
                        .clone()
                        .into_disclosed_object(&Sha256Hasher::new())
                        .map_err(|err| {
                            AppError::Error(format!("Failed to convert SD JWT VC to Disclosed Object: {err}"))
                        })?;
                    let credential =
                        CredentialV2::<Object>::from_json_value(serde_json::Value::Object(disclosed_claims)).map_err(
                            |err| AppError::Error(format!("Failed to convert Disclosed Object to Credential: {err}")),
                        )?;

                    let display_claims: Vec<DisplayClaim> = get_display_claims(claim_descriptions, &json!(credential));

                    let data = json!(credential);

                    (id, data, issuance_date, expiration_date, display_claims)
                }
                CredentialFormats::JwtVcJson(()) => {
                    let credential_display = get_unverified_jwt_claims(&verifiable_credential)?
                        .get("vc")
                        .cloned()
                        .ok_or(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: 'vc' claim is missing in the JWT VC"
                                .to_string(),
                        ))?;

                    // TODO: do not use a hash to generate the credential ID. Currently we still do this so that our tests in `unime/src-tauri/tests` don't break.
                    let hash = { sha256::digest(json!(credential_display).to_string()) };

                    let id = Uuid::from_slice(&hash.as_bytes()[..16])?.to_string();
                    let issuance_date = credential_display["issuanceDate"] // TODO: direct indexing, unsafe
                        .as_str()
                        .map(ToString::to_string)
                        .ok_or(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: 'issuanceDate' is missing".to_string(),
                        ))?;
                    let expiration_date = credential_display
                        .get("expirationDate") // TODO: this is Data model specific, ensure catching all cases
                        .and_then(|valid_until| valid_until.as_str().map(ToString::to_string)); // TODO: import this from UniCore

                    // TODO: Use the claims to rename the keys in the Credential according to the display hints provided by
                    // the Issuer. Before we do this we need to make sure that UniCore supports Claims Description for
                    // Issuer Metadata (see: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#claims-description-issuer-metadata).
                    // For now we just display the raw credential as is.
                    let display_claims = vec![];

                    let data = credential_display;

                    (id, data, issuance_date, expiration_date, display_claims)
                }
                _ => {
                    return Err(AppError::Error(
                    "Failed to create a VerifiableCredentialRecord: provided verifiable credential format is not supported"
                        .to_string(),
                ));
                }
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
                    expiration_date,
                    icon: None,
                },
                // The other fields will be filled in at a later stage.
                issuer_name: String::new(),
                connection_id: None,
                display_name: String::new(),
                // The credential status is None here but it will be set right after this function.
                // This initialization is separated since it requires async fetching.
                credential_status: None,
                public_link: None,
            }
        };

        Ok(Self {
            verifiable_credential,
            display_credential,
        })
    }
}

fn get_display_claims(claim_descriptions: Vec<ClaimDescription>, data: &serde_json::Value) -> Vec<DisplayClaim> {
    claim_descriptions
        .into_iter()
        .map(|claim_description| {
            let key = claim_description
                .display
                // TODO: Support multiple locales here. For now we just take the first one if it exists.
                .first()
                .map(|display| display.name.clone())
                // TODO: Come up with a proper fallback strategy here.
                .unwrap_or_default();
            let value = claim_description
                .path
                .get_values_from_json(data)
                .first()
                .cloned()
                // TODO: Come up with a proper fallback strategy here.
                .unwrap_or_default();

            DisplayClaim {
                path: claim_description.path,
                key,
                value,
            }
        })
        .collect()
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

        let verifiable_credential_record =
            VerifiableCredentialRecord::try_new(CredentialFormats::JwtVcJson(()), jwt_vc_json, vec![]).unwrap();

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
        let sd_jwt_vc = json!("eyJ0eXAiOiJkYytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJ2Y3QiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvdmN0L1UwUXRTbGRVLzAiLCJfc2QiOlsiNzZ0bGNVNi1kMlNaQUhHV3ZrVk5aV2hweS11QUFzczBuemdCdmx0X19QRSIsIlZsSVdHMVJNai1vckpSeTZIWUJqaXRLaDVUOGxLTUU5UkVlbmFUN2htRzgiLCJta19kazVuc0pXQ2hZYWpocW4yT2N2cXNYazlRdTdNcHdXeEpnTlBQQVp3Il0sImlzcyI6ImRpZDp3ZWI6bG9jYWxob3N0JTNBMzAzMyIsIm5iZiI6MTc3MDE0OTU2NSwiaWF0IjoxNzcwMTQ5NTY1LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMSIsImlkeCI6ODEyNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.8ZY3p-eeEtlFQjfEYBcRGjxHeq2DpqIHNgD4WKPLcF8qmK8bWOJxYKaw1cfrptx6WsNs5cqhZgtlztBV9HckMw~WyJaam1obE9QQzhpZWREOHJxTXItckxvVzFvRXFDSUR4SHFoQmJkRlpKIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyIyUkhteHh0cXpHNHRCeU1DdG9LWXhTRXdEaE92MTd0Z1FaR2hNVEUwIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~WyJQSlBPbjZaSG95Z0FlUHltWU5iRVYtVDJ0N3lwU2E0eE5uOVlDQ1ZBIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~");

        let verifiable_credential_record = VerifiableCredentialRecord::try_new(
            CredentialFormats::DcSdJwt(()),
            sd_jwt_vc,
            vec![ClaimDescription {
                path: ClaimPathPointer::try_new(vec![ClaimPathElement::String("first_name".to_string())]).unwrap(),
                mandatory: true,
                display: vec![ClaimDescriptionDisplay {
                    name: "First Name".to_string(),
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
                path: ClaimPathPointer::try_new(vec![ClaimPathElement::String("first_name".to_string())]).unwrap(),
                key: "First Name".to_string(),
                value: json!("Ferris"),
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:12:45Z"
        );
    }

    #[test]
    fn test_verifiable_credential_record_try_from_vcdm2_sd_jwt() {
        let vcdm2_sd_jwt = json!("eyJ0eXAiOiJ2YytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvbnMvY3JlZGVudGlhbHMvdjIiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJfc2QiOlsiLU04ZmdVWlVuSkllWUpCcU45d2NjZVlwMzBMS21pZlZrR1Z1OFNacWFoQSIsIlZ5dW1yWUxPblJXaUxJMDZzTk1XY1J2MkswWGFoYWZnWjM0ekZMcDlDREEiLCJ2cnFyWW1yelRCTmtkSGlXLWllMnpURG1BamhjQkFURU5sZVNkaUI5VDN3Il19LCJpc3N1ZXIiOnsiaWQiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvIiwibmFtZSI6IlVuaUNvcmUifSwidmFsaWRGcm9tIjoiMjAyNi0wMi0wM1QyMDoxMToxOFoiLCJjcmVkZW50aWFsU3RhdHVzIjp7ImlkIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsInR5cGUiOiJzdGF0dXNsaXN0K2p3dCIsImlkeCI6MTExNCwidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCJ9LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsImlkeCI6MTExNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.ZxWYelfw39UScERquLEshscRMOBt0lyDrOaD9WzZgJ81j42wTVOYvqL1Ok_C3UcGgs-6-szu4o2V8uRksQc9jw~WyJZN1pKZGlqMnZPSDBWSnJmaEswY29zQVlPRGtibjlmcHNPMFNqTFduIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~WyJoU1hxZEZWYzlYRnBoLWFWaUtQZUoxVVNBU3Z2c3RCOVBuaVdNSGFFIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyJjZFZ4cXJucWE2WF93SUhyOGN0QnlnZXdFU0dTSHNZNDJsSVg5MzZOIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~");

        let verifiable_credential_record = VerifiableCredentialRecord::try_new(
            CredentialFormats::VcSdJwt(()),
            vcdm2_sd_jwt,
            vec![ClaimDescription {
                path: ClaimPathPointer::try_new(vec![
                    ClaimPathElement::String("credentialSubject".to_string()),
                    ClaimPathElement::String("first_name".to_string()),
                ])
                .unwrap(),
                mandatory: true,
                display: vec![ClaimDescriptionDisplay {
                    name: "First Name".to_string(),
                    locale: None,
                }],
            }],
        )
        .unwrap();

        assert_eq!(
            verifiable_credential_record.display_credential.format,
            CredentialFormats::VcSdJwt(())
        );

        assert_eq!(
            verifiable_credential_record.display_credential.display_claims,
            vec![DisplayClaim {
                path: ClaimPathPointer::try_new(vec![
                    ClaimPathElement::String("credentialSubject".to_string()),
                    ClaimPathElement::String("first_name".to_string()),
                ])
                .unwrap(),
                key: "First Name".to_string(),
                value: json!("Ferris"),
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:11:18Z"
        );
    }
}
