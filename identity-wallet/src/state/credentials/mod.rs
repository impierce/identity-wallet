pub mod actions;
pub mod reducers;

use std::collections::HashMap;

use super::{core_utils::helpers::get_unverified_jwt_claims, FeatTrait};
use crate::{
    error::AppError,
    persistence::{hash, persist_asset},
    state::{
        core_utils::{
            helpers::{validate_credential_types, validate_jwt_vc_json, validate_vc_sd_jwt},
            DateUtils, IdentityManager,
        },
        credentials::reducers::refresh_credential_status::fetch_credential_status,
        profile_settings::Locale,
    },
};
use base64::display;
use derivative::Derivative;
use did_manager::Resolver;
use identity_credential::sd_jwt_vc::SdJwtVc;
use identity_iota::{
    core::{FromJson as _, Object},
    credential::{self, CredentialV2, Jwt},
};
use log::{info, warn};
use oauth_tsl::{status_list::StatusType, tokens::referenced_token::StatusClaim};
use oid4vc::{
    oid4vc_core::claim_path_pointer::ClaimPathPointer,
    oid4vci::{
        credential_format_profiles::CredentialFormats,
        credential_issuer::credential_configurations_supported::{
            ClaimDescription, CredentialConfigurationsSupportedObject,
        },
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
    pub is_selective_disclosable: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS, Default)]
#[ts(export, export_to = "bindings/credentials/CredentialType.ts")]
pub enum CredentialType {
    #[default]
    Plain,
    OpenBadgeCredential,
    PID,
}

/// A credential displayable by the frontend.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS, Default)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/DisplayCredential.ts")]
pub struct DisplayCredential {
    #[derivative(PartialEq = "ignore")]
    pub id: Uuid,
    pub credential_type: CredentialType,
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
    pub is_self_issued: bool,
    #[derivative(PartialEq = "ignore")]
    pub date_added: String,
    #[derivative(PartialEq = "ignore")]
    pub date_issued: String,
    #[ts(optional)]
    pub icon: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct VerifiableCredentialRecord {
    pub id: Uuid,
    pub verifiable_credential: serde_json::Value,
    pub display_credential: DisplayCredential,
}

impl VerifiableCredentialRecord {
    pub async fn try_new(
        identity_manager: &IdentityManager,
        resolver: &Resolver,
        verifiable_credential: serde_json::Value,
        credential_configuration: &CredentialConfigurationsSupportedObject,
        issuer_name: &String,
        connection_id: Option<String>,
    ) -> Result<Self, AppError> {
        let id = Uuid::new_v4();

        // Collect the display information for the credential from the credential configuration metadata if it exists.
        let (display, claims) = credential_configuration
            .credential_metadata
            .as_ref()
            .map(|metadata| {
                (
                    metadata.display.clone().unwrap_or_default(),
                    metadata.claims.clone().unwrap_or_default(),
                )
            })
            .unwrap_or_default();

        let (data, credential_status_value, issuance_date) = match credential_configuration.credential_format {
            CredentialFormats::JwtVcJson(_) => {
                // Convert the received credential (as a string) into a Jwt instance for validation.
                let jwt_vc_json_credential = Jwt::new(
                    verifiable_credential
                        .as_str()
                        .ok_or(AppError::Error("Invalid JWT string.".to_string()))?
                        .to_string(),
                );

                let data = json!(
                    validate_jwt_vc_json(&resolver, jwt_vc_json_credential)
                        .await?
                        .credential
                );

                let credential_status_value = data.get("credentialStatus").cloned();

                let issuance_date = data["issuanceDate"]
                    .as_str()
                    .map(ToString::to_string)
                    .unwrap_or_default();

                (data, credential_status_value, issuance_date)
            }
            CredentialFormats::DcSdJwt(_) => {
                let dc_sd_jwt_credential = verifiable_credential
                    .as_str()
                    .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwtVc>().ok())
                    .unwrap();

                info!("Verifiable Credential parsed as a SD-JWT VC");

                let issuance_date = dc_sd_jwt_credential
                    .claims()
                    .iat
                    .map(|iat| iat.to_rfc3339())
                    .unwrap_or_default();

                let data = serde_json::json!(dc_sd_jwt_credential
                    .clone()
                    .into_disclosed_object(&Sha256Hasher::new())
                    .map_err(|_| AppError::Error("Failed to convert SD JWT VC to Disclosed Object".to_string()))?);

                (data, None, issuance_date)
            }
            CredentialFormats::VcSdJwt(_) => {
                let vc_sd_jwt_credential = verifiable_credential
                    .as_str()
                    .and_then(|verifiable_credential| verifiable_credential.parse::<SdJwt>().ok())
                    .unwrap();

                if vc_sd_jwt_credential.headers().get("typ").and_then(|typ| typ.as_str()) != Some("vc+sd-jwt") {
                    return Err(AppError::Error(
                        "Failed to create a VerifiableCredentialRecord: SD-JWT 'typ' header is not 'vc+sd-jwt'"
                            .to_string(),
                    ));
                }

                let data = json!(validate_vc_sd_jwt(resolver, vc_sd_jwt_credential.clone()).await?);

                let issuance_date = vc_sd_jwt_credential
                    .claims()
                    .get("validFrom")
                    .and_then(|valid_from| valid_from.as_str())
                    .unwrap_or_default()
                    .to_string();

                (data, None, issuance_date)
            }
            _ => {
                return Err(AppError::Error(
                    "Failed to create a VerifiableCredentialRecord: verifiable credential format either an invalid SD-JWT or JWT VC JSON, or an unsupported format"
                        .to_string(),
                ));
            }
        };

        let credential_status_value = verifiable_credential.get("status").cloned().or(credential_status_value);

        let credential_status = if let Some(credential_status_value) = &credential_status_value {
            if let Some(credential_status_claim) =
                serde_json::from_value::<StatusClaim>(credential_status_value.clone()).ok()
            {
                get_credential_status(credential_status_claim, identity_manager)
                    .await
                    .inspect(|_| info!("Successfully fetched credential status for credential with id: `{id}`"))
            } else {
                warn!("The credential status claim/property is not in the OAuth Token Status List format: {credential_status_value}");
                None
            }
        } else {
            warn!("The credential does not contain a status claim/property");

            None
        };

        // Validate the credential against its corresponding credential JSON Schema.
        validate_credential_types(&data)?;

        let display_claims: Vec<DisplayClaim> = get_display_claims(claims, &data);

        let display_credential = DisplayCredential {
            id: id.clone(),
            credential_type: CredentialType::Plain,
            data,
            display_claims,
            metadata: CredentialMetadata {
                is_self_issued: false,
                is_favorite: false,
                date_added: DateUtils::new_date_string(),
                date_issued: issuance_date,
                icon: None,
            },
            issuer_name: issuer_name.clone(),
            connection_id: connection_id.clone(),
            display_name: String::new(),
            credential_status,
        };

        // Persist the Credential logo if it exists.
        display
            .first()
            .and_then(|display| display.logo.clone())
            .map(|logo| logo.uri.clone())
            .and_then(|uri| persist_asset(&hash(uri.as_str()), id.to_string().as_str()).ok());

        Ok(Self {
            id,
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
                is_selective_disclosable: false,
            }
        })
        .collect()
}

/// Helper function to fetch the credential status of a newly received credential and set the fields the `credential_status` field of the DisplayCredential.
/// Currently supports only the OAuth Token Status List mechanism.
/// The function looks for the credential status info in 2 places:
/// 1. In the JWT root for the key `status` as specified in the IETF OAuth Token Status List specification.
/// 2. In the `credentialStatus` property of the credential, as specified in the W3C Verifiable Credential Data Model specification (1.1 and 2.0).
///     * How to fill in the `credentialStatus` property is not specified in the W3C VC Data Model specifications for the OAuth Token Status List mechanism.
///       We decided the most logical way is to assume this should be exactly the same as the `status` claim in the JWT root.
///       There is a discussion ongoing in the DIIP profile community about this, see: https://github.com/FIDEScommunity/DIIP/issues/60
///
/// An error is returned when:
/// 1. The credential does not contain a status claim in the JWT root or a credentialStatus property in the VC.
/// 2. The status claim/property does not use the OAuth Token Status List mechanism.
async fn get_credential_status(
    credential_status_claim: StatusClaim,
    identity_manager: &IdentityManager,
) -> Option<CredentialStatus> {
    // Here we initialize the credential status with UNDEFINED status and an empty last_checked field, these fields will be filled after fetching the status.
    let mut credential_status_data = CredentialStatus {
        status: StatusType::UNDEFINED,
        idx: credential_status_claim.referenced_status_list.idx,
        uri: credential_status_claim.referenced_status_list.uri,
        last_checked: String::new(),
    };

    let status = match fetch_credential_status(&credential_status_data, identity_manager).await {
        Ok(status) => status,
        Err(_) => {
            warn!("Failed to fetch credential status");
            return None;
        }
    };
    credential_status_data.status = status;
    credential_status_data.last_checked = DateUtils::new_date_string();

    Some(credential_status_data)
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

        // assert_eq!(
        //     verifiable_credential_record.display_credential.format,
        //     CredentialFormats::JwtVcJson(())
        // );

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

        // assert_eq!(
        //     verifiable_credential_record.display_credential.format,
        //     CredentialFormats::DcSdJwt(())
        // );

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
                is_selective_disclosable: false,
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:12:45Z"
        );
    }

    #[test]
    fn test_verifiable_credential_record_try_from_vcdm2_sd_jwt() {
        let sd_jwt_vc = json!("eyJ0eXAiOiJ2YytzZC1qd3QiLCJraWQiOiJkaWQ6d2ViOmxvY2FsaG9zdCUzQTMwMzMjQXM1cjRVeG9fWjREV19XLW9BUVNURTRLT1hhMnVWeUJRYUNna3VTUkhwWSIsImFsZyI6IkVTMjU2In0.eyJAY29udGV4dCI6WyJodHRwczovL3d3dy53My5vcmcvbnMvY3JlZGVudGlhbHMvdjIiXSwidHlwZSI6WyJWZXJpZmlhYmxlQ3JlZGVudGlhbCJdLCJjcmVkZW50aWFsU3ViamVjdCI6eyJfc2QiOlsiLU04ZmdVWlVuSkllWUpCcU45d2NjZVlwMzBMS21pZlZrR1Z1OFNacWFoQSIsIlZ5dW1yWUxPblJXaUxJMDZzTk1XY1J2MkswWGFoYWZnWjM0ekZMcDlDREEiLCJ2cnFyWW1yelRCTmtkSGlXLWllMnpURG1BamhjQkFURU5sZVNkaUI5VDN3Il19LCJpc3N1ZXIiOnsiaWQiOiJodHRwOi8vbG9jYWxob3N0OjMwMzMvIiwibmFtZSI6IlVuaUNvcmUifSwidmFsaWRGcm9tIjoiMjAyNi0wMi0wM1QyMDoxMToxOFoiLCJjcmVkZW50aWFsU3RhdHVzIjp7ImlkIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsInR5cGUiOiJzdGF0dXNsaXN0K2p3dCIsImlkeCI6MTExNCwidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCJ9LCJzdGF0dXMiOnsic3RhdHVzX2xpc3QiOnsidXJpIjoiaHR0cDovL2xvY2FsaG9zdDozMDMzL2lldGYtb2F1dGgtdG9rZW4tc3RhdHVzLWxpc3QvMCIsImlkeCI6MTExNH19LCJfc2RfYWxnIjoic2hhLTI1NiIsImNuZiI6eyJraWQiOiJkaWQ6andrOmV5SmhiR2NpT2lKRlV6STFOaUlzSW1OeWRpSTZJbEF0TWpVMklpd2lhMmxrSWpvaVdHMUpPRmc1ZVhCU05WVTJlRE5ZTm5ab1NuUk9hM1ZwVkVWTk4zRklkbk5wVjBoa04zSkdSbTFxVVNJc0ltdDBlU0k2SWtWRElpd2llQ0k2SW1aME4yUnZja1JLZDFwTE5GaG5hVGRsVDFJNGNXRkZMWE4yTUhoQ01qaFJOWFZLYTNJdFEzZEJNalFpTENKNUlqb2lSWFI0T1RWM1IxcFZZekE1VjJzMVJXTnZNR1JHZVhreGVXVkVlbUl6TFdsS1VuRTNOMDF4T1U5YVl5SjkjMCJ9fQ.ZxWYelfw39UScERquLEshscRMOBt0lyDrOaD9WzZgJ81j42wTVOYvqL1Ok_C3UcGgs-6-szu4o2V8uRksQc9jw~WyJZN1pKZGlqMnZPSDBWSnJmaEswY29zQVlPRGtibjlmcHNPMFNqTFduIiwiZG9iIiwiMTk4Mi0wMS0wMSJd~WyJoU1hxZEZWYzlYRnBoLWFWaUtQZUoxVVNBU3Z2c3RCOVBuaVdNSGFFIiwiZmlyc3RfbmFtZSIsIkZlcnJpcyJd~WyJjZFZ4cXJucWE2WF93SUhyOGN0QnlnZXdFU0dTSHNZNDJsSVg5MzZOIiwibGFzdF9uYW1lIiwiQ3JhYm1hbiJd~");

        let verifiable_credential_record = VerifiableCredentialRecord::try_new(
            sd_jwt_vc,
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
            verifiable_credential_record.display_credential.display_claims,
            vec![DisplayClaim {
                path: ClaimPathPointer::try_new(vec![
                    ClaimPathElement::String("credentialSubject".to_string()),
                    ClaimPathElement::String("first_name".to_string()),
                ])
                .unwrap(),
                key: "First Name".to_string(),
                value: json!("Ferris"),
                is_selective_disclosable: false,
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:11:18Z"
        );
    }
}
