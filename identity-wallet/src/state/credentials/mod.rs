pub mod actions;
pub mod credential_query_matching;
pub mod issuer_signature;
pub mod reducers;
pub mod selective_disclosure;
pub mod self_issued_migration;
use super::FeatTrait;
use crate::{error::AppError, state::core_utils::DateUtils};
use derivative::Derivative;
use identity_kit::{
    sd_jwt_vc::{SdJwt, SdJwtVc},
    status_list::StatusType,
    vc::{CredentialV2, JwtClaims},
};
use log::warn;
use oid4vc_core::{
    claim_path_pointer::{check_claims_description_paths, ClaimPathElement, ClaimPathPointer, ClaimPathSemantics},
    utils::jwt::get_unverified_jwt_claims,
};
use oid4vci::{
    credential_format_profiles::CredentialFormats,
    credential_issuer::credential_configurations_supported::ClaimDescription,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ts_rs::TS;
use url::Url;
use uuid::Uuid;

/// A top-level claim of a credential, as the issuer's claims metadata describes it (OpenID4VCI 1.0, Appendix B.2).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS)]
#[ts(export, export_to = "bindings/credentials/DisplayClaim.ts")]
pub struct DisplayClaim {
    #[ts(type = "Array<string>")]
    pub path: ClaimPathPointer,
    pub key: String,
    #[ts(type = "any")]
    pub value: serde_json::Value,
    /// How the issuer wants the claim shown, e.g. `table`, `country` or `date`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub format: Option<String>,
    /// The issuer's descriptions of the claims inside this one, in the issuer's order.
    #[serde(default)]
    pub nested: Vec<NestedDisplayClaim>,
}

/// A claim inside a [`DisplayClaim`], as the issuer's claims metadata describes it.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, TS)]
#[ts(export, export_to = "bindings/credentials/NestedDisplayClaim.ts")]
pub struct NestedDisplayClaim {
    /// The path from the enclosing claim. `null` selects every item of an array (OpenID4VCI 1.0, Appendix C.1).
    #[ts(type = "Array<string | number | null>")]
    pub path: Vec<ClaimPathElement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(optional)]
    pub format: Option<String>,
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
    #[serde(default)]
    pub issuer_logo_uri: Option<String>,
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
    #[ts(optional)]
    pub public_link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Derivative, TS)]
#[derivative(PartialEq)]
#[ts(export, export_to = "bindings/credentials/CredentialStatus.ts")]
pub struct CredentialStatus {
    #[ts(type = "'VALID' | 'INVALID' | 'SUSPENDED' | 'UNDEFINED'")]
    pub status: CredentialStatusType,
    #[ts(skip)]
    pub idx: u64,
    #[ts(skip)]
    pub uri: Url,
    #[derivative(PartialEq = "ignore")]
    pub last_checked: String,
}

/// The status of a credential as the wallet stores and shows it, read from the credential's Token Status List.
///
/// It serializes as its name, as persisted state and the frontend expect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CredentialStatusType {
    Valid,
    Invalid,
    Suspended,
    /// An application-specific status, or a status that has not been fetched yet.
    Undefined,
}

impl TryFrom<StatusType> for CredentialStatusType {
    type Error = AppError;

    fn try_from(status: StatusType) -> Result<Self, AppError> {
        match status.value() {
            0 => Ok(Self::Valid),
            1 => Ok(Self::Invalid),
            2 => Ok(Self::Suspended),
            // Application-specific values.
            3 | 11..=15 => Ok(Self::Undefined),
            // Every other value is reserved for future registration.
            _ => Err(AppError::GetCredentialStatusError),
        }
    }
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
    /// The moment from which the credential is valid (`validFrom`, VCDM 1.1 `issuanceDate` or `nbf`), if it states one.
    #[ts(optional)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[derivative(PartialEq = "ignore")]
    pub valid_from: Option<String>,
    // TODO: use this expiration date in the frontend to display
    #[ts(optional)]
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
                    let sd_jwt = parse_sd_jwt(&verifiable_credential, "dc+sd-jwt")?;
                    let sd_jwt_vc = SdJwtVc::from_sd_jwt(sd_jwt).map_err(|err| {
                        AppError::Error(format!(
                            "Failed to create a VerifiableCredentialRecord: provided verifiable credential is not a valid SD-JWT VC: {err}"
                        ))
                    })?;

                    let id = Uuid::new_v4().to_string();
                    let issuance_date = sd_jwt_vc.claims().iat.and_then(rfc3339).unwrap_or_default();
                    let expiration_date = sd_jwt_vc.claims().exp.and_then(rfc3339);

                    let credential_subject = json!(
                        sd_jwt_vc
                            .sd_jwt()
                            .process()
                            .map_err(|_| AppError::Error(
                                "Failed to convert SD JWT VC to Disclosed Object".to_string()
                            ))?
                            .payload
                    );

                    // The claims of an SD-JWT VC sit at the top level of its payload.
                    let display_claims: Vec<DisplayClaim> =
                        get_display_claims(claim_descriptions, &json!(credential_subject), &[]);

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
                    let vcdm2_sd_jwt = parse_sd_jwt(&verifiable_credential, "vc+sd-jwt")?;
                    let claims = vcdm2_sd_jwt.payload().map_err(|err| {
                        AppError::Error(format!(
                            "Failed to create a VerifiableCredentialRecord: provided verifiable credential is not a valid SD-JWT: {err}"
                        ))
                    })?;

                    let id = Uuid::new_v4().to_string();
                    let issuance_date = claims
                        .get("validFrom")
                        .and_then(|valid_from| valid_from.as_str())
                        .unwrap_or_default()
                        .to_string();
                    let expiration_date = claims
                        .get("validUntil")
                        .and_then(|valid_until| valid_until.as_str().map(ToString::to_string)); // TODO: import this as fn to_unescaped_string from UniCore, once we implement UniCore in UniMe.

                    let disclosed_claims = vcdm2_sd_jwt.process().map_err(|err| {
                        AppError::Error(format!("Failed to convert SD JWT VC to Disclosed Object: {err}"))
                    })?;
                    let credential =
                        serde_json::from_value::<CredentialV2>(serde_json::Value::Object(disclosed_claims.payload))
                            .map_err(|err| {
                                AppError::Error(format!("Failed to convert Disclosed Object to Credential: {err}"))
                            })?;

                    // The claims of a VCDM 2.0 credential sit in its `credentialSubject`.
                    let display_claims: Vec<DisplayClaim> =
                        get_display_claims(claim_descriptions, &json!(credential), &["credentialSubject"]);

                    let data = json!(credential);

                    (id, data, issuance_date, expiration_date, display_claims)
                }
                CredentialFormats::JwtVcJson(()) => {
                    let claims = get_unverified_jwt_claims(&verifiable_credential)
                        .map_err(|e| AppError::Error(e.to_string()))?;
                    let credential_display = claims.get("vc").cloned().ok_or(AppError::Error(
                        "Failed to create a VerifiableCredentialRecord: 'vc' claim is missing in the JWT VC"
                            .to_string(),
                    ))?;

                    // TODO: do not use a hash to generate the credential ID. Currently we still do this so that our tests in `unime/src-tauri/tests` don't break.
                    let hash = { sha256::digest(json!(credential_display).to_string()) };

                    let id = Uuid::from_slice(&hash.as_bytes()[..16])?.to_string();
                    let issuance_date = credential_display
                        .get("issuanceDate")
                        .or_else(|| credential_display.get("validFrom"))
                        .and_then(|value| value.as_str().map(ToString::to_string))
                        .or_else(|| {
                            claims.get("nbf").or_else(|| claims.get("iat")).and_then(|v| {
                                if let Some(secs) = v.as_i64() {
                                    chrono::DateTime::from_timestamp(secs, 0).map(|dt| dt.to_rfc3339())
                                } else {
                                    v.as_str().map(|s| s.to_string())
                                }
                            })
                        })
                        .ok_or(AppError::Error(
                            "Failed to create a VerifiableCredentialRecord: 'issuanceDate' or 'validFrom' is missing"
                                .to_string(),
                        ))?;
                    let expiration_date = credential_display
                        .get("expirationDate")
                        .or_else(|| credential_display.get("validUntil"))
                        .and_then(|valid_until| valid_until.as_str().map(ToString::to_string))
                        .or_else(|| {
                            claims.get("exp").and_then(|v| {
                                if let Some(secs) = v.as_i64() {
                                    chrono::DateTime::from_timestamp(secs, 0).map(|dt| dt.to_rfc3339())
                                } else {
                                    v.as_str().map(|s| s.to_string())
                                }
                            })
                        });

                    // TODO: Use the claims to rename the keys in the Credential according to the display hints provided by
                    // the Issuer. Before we do this we need to make sure that UniCore supports Claims Description for
                    // Issuer Metadata (see: https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0-15.html#claims-description-issuer-metadata).
                    // For now we just display the raw credential as is.
                    let display_claims = vec![];

                    let data = credential_display;

                    (id, data, issuance_date, expiration_date, display_claims)
                }
                _ => {
                    return Err(AppError::UnsupportedError(
                    "Failed to create a VerifiableCredentialRecord: provided verifiable credential format is not supported"
                        .to_string(),
                ));
                }
            };

            let valid_from = valid_from(&format, &verifiable_credential);

            DisplayCredential {
                id,
                format,
                data,
                display_claims,
                metadata: CredentialMetadata {
                    is_favorite: false,
                    date_added: DateUtils::new_date_string(),
                    date_issued: issuance_date,
                    valid_from,
                    expiration_date,
                    icon: None,
                },
                // The other fields will be filled in at a later stage.
                issuer_name: String::new(),
                issuer_logo_uri: None,
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

/// The latest of the credential's `validFrom` (or VCDM 1.1 `issuanceDate`) and its JWT `nbf`. The issuance time (`iat`)
/// says when the credential was signed, not from when it is valid, so it is not used.
pub fn valid_from(format: &CredentialFormats, verifiable_credential: &serde_json::Value) -> Option<String> {
    let timestamp = |value: &serde_json::Value| match value {
        serde_json::Value::Number(secs) => {
            chrono::DateTime::from_timestamp(secs.as_i64()?, 0).map(|date| date.to_rfc3339())
        }
        serde_json::Value::String(date) => Some(date.clone()),
        _ => None,
    };

    let candidates: Vec<String> = match format {
        CredentialFormats::DcSdJwt(()) => {
            let sd_jwt_vc = SdJwtVc::parse(verifiable_credential.as_str()?).ok()?;
            sd_jwt_vc.claims().nbf.and_then(rfc3339).into_iter().collect()
        }
        CredentialFormats::VcSdJwt(()) => {
            let claims = SdJwt::parse(verifiable_credential.as_str()?).ok()?.payload().ok()?;
            ["validFrom", "nbf"]
                .iter()
                .filter_map(|key| claims.get(*key).and_then(timestamp))
                .collect()
        }
        CredentialFormats::JwtVcJson(()) => {
            let claims = get_unverified_jwt_claims(verifiable_credential).ok()?;
            let vc = claims.get("vc");
            [
                vc.and_then(|vc| vc.get("validFrom")),
                vc.and_then(|vc| vc.get("issuanceDate")),
                claims.get("nbf"),
            ]
            .into_iter()
            .flatten()
            .filter_map(timestamp)
            .collect()
        }
        _ => vec![],
    };

    candidates
        .into_iter()
        .filter_map(|date| {
            chrono::DateTime::parse_from_rfc3339(&date)
                .ok()
                .map(|parsed| (parsed, date))
        })
        .max_by_key(|(parsed, _)| *parsed)
        .map(|(_, date)| date)
}

/// The JWT NumericDate `numeric_date` in RFC 3339, in UTC with a `Z`.
fn rfc3339(numeric_date: i64) -> Option<String> {
    JwtClaims::timestamp(numeric_date).map(|date| date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true))
}

/// The SD-JWT `verifiable_credential`, which must carry the `typ` header `typ`.
fn parse_sd_jwt(verifiable_credential: &serde_json::Value, typ: &str) -> Result<SdJwt, AppError> {
    let sd_jwt = verifiable_credential
        .as_str()
        .and_then(|verifiable_credential| SdJwt::parse(verifiable_credential).ok())
        .ok_or(AppError::Error(
            "Failed to create a VerifiableCredentialRecord: provided verifiable credential is not a valid SD-JWT"
                .to_string(),
        ))?;
    let header_typ = sd_jwt.jwt().decode().ok().and_then(|jws| jws.header().typ.clone());
    if header_typ.as_deref() != Some(typ) {
        return Err(AppError::UnsupportedError(format!(
            "Failed to create a VerifiableCredentialRecord: SD-JWT 'typ' header is not '{typ}'"
        )));
    }
    Ok(sd_jwt)
}

/// The top-level claims of a credential with what the issuer's claims metadata says about them (OpenID4VCI 1.0,
/// Appendix B). Every claims description belongs to the top-level claim its path starts with, the first element below
/// `root`:
/// - a description of the top-level claim itself gives its label and format;
/// - a description of a claim inside it becomes a [`NestedDisplayClaim`], so that an Object, a List or a Table keeps its
///   structure and labels, also when the issuer describes only the leaves.
///
/// The claims keep the order of their first description, which Appendix B.3 makes the display order. A claim the
/// credential does not hold is left out.
fn get_display_claims(
    claim_descriptions: Vec<ClaimDescription>,
    data: &serde_json::Value,
    root: &[&str],
) -> Vec<DisplayClaim> {
    // Appendix B.3: "When a repeated or contradictory claim description is provided, the processing MUST be
    // aborted." Without display claims, the credential shows its raw claims. The paths hold claim names, not values.
    let paths = claim_descriptions
        .iter()
        .map(|claim_description| &claim_description.path);
    if let Err(error) = check_claims_description_paths(paths, ClaimPathSemantics::Json) {
        warn!("Ignoring the issuer's claims metadata: {error}");
        return vec![];
    }

    let mut display_claims: Vec<DisplayClaim> = vec![];
    for claim_description in claim_descriptions {
        let elements = claim_description.path.as_ref();
        let under_root = elements.len() > root.len()
            && root
                .iter()
                .zip(elements)
                .all(|(key, element)| matches!(element, ClaimPathElement::String(name) if name == key));
        let top_level_length = if under_root { root.len() + 1 } else { 1 };
        let (top_level_path, nested_path) = elements.split_at(top_level_length);

        let index = match display_claims
            .iter()
            .position(|claim| claim.path.as_ref() == top_level_path)
        {
            Some(index) => index,
            None => {
                let Ok(path) = ClaimPathPointer::try_new(top_level_path.to_vec()) else {
                    continue;
                };
                // A claim the credential does not have selects nothing (OpenID4VP 1.0, Section 7.1.1): no value.
                let value = path
                    .get_values_from_json(data)
                    .ok()
                    .and_then(|values| values.into_iter().next())
                    .unwrap_or_default();
                display_claims.push(DisplayClaim {
                    key: last_key(top_level_path),
                    path,
                    value,
                    format: None,
                    nested: vec![],
                });
                display_claims.len() - 1
            }
        };

        let label = claim_label(&claim_description);
        let format = claim_format(&claim_description);
        let display_claim = &mut display_claims[index];
        if nested_path.is_empty() {
            if let Some(label) = label {
                display_claim.key = label;
            }
            display_claim.format = format;
        } else {
            display_claim.nested.push(NestedDisplayClaim {
                path: nested_path.to_vec(),
                label,
                format,
            });
        }
    }

    display_claims.retain(|claim| !claim.value.is_null());
    display_claims
}

/// The last key of a path, the claim's name when the issuer gives no label.
fn last_key(path: &[ClaimPathElement]) -> String {
    path.iter()
        .rev()
        .find_map(|element| match element {
            ClaimPathElement::String(key) if !key.trim().is_empty() => Some(key.clone()),
            _ => None,
        })
        .unwrap_or_default()
}

/// The issuer's label for a claim.
fn claim_label(claim_description: &ClaimDescription) -> Option<String> {
    claim_description
        .display
        // TODO: Support multiple locales here. For now we just take the first one if it exists.
        .first()
        .and_then(|display| display.name.as_deref())
        .map(|name| name.trim().to_string())
        .filter(|name| !name.is_empty())
}

/// The issuer's `format` for a claim, a parameter of the claim's display objects next to `name` and `locale`
/// (impierce/ssi-agent#534). A format does not depend on the language, so the first display object that has one
/// gives it. The frontend ignores a format it does not know.
fn claim_format(claim_description: &ClaimDescription) -> Option<String> {
    claim_description.display.iter().find_map(|display| {
        display
            .additional_parameters
            .get("format")
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|format| !format.is_empty())
            .map(ToString::to_string)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use oid4vci::credential_issuer::credential_configurations_supported::ClaimDescriptionDisplay;

    #[test]
    fn test_get_display_claims_uses_claim_path_as_key_when_display_name_is_missing() {
        let path = ClaimPathPointer::try_new(vec![
            ClaimPathElement::String("credentialSubject".to_string()),
            ClaimPathElement::String("email".to_string()),
        ])
        .unwrap();

        let display_claims = get_display_claims(
            vec![ClaimDescription {
                path: path.clone(),
                mandatory: true,
                display: vec![],
            }],
            &json!({
                "credentialSubject": {
                    "email": "ferris@example.com"
                }
            }),
            &["credentialSubject"],
        );

        assert_eq!(
            display_claims,
            vec![DisplayClaim {
                path,
                key: "email".to_string(),
                value: json!("ferris@example.com"),
                format: None,
                nested: vec![],
            }]
        );
    }

    /// A claims description with a label, or none.
    fn description(path: serde_json::Value, name: Option<&str>) -> ClaimDescription {
        match name {
            Some(name) => from_metadata(json!({ "path": path, "display": [{ "name": name }] })),
            None => from_metadata(json!({ "path": path })),
        }
    }

    /// A claims description from its JSON form in the issuer metadata.
    fn from_metadata(description: serde_json::Value) -> ClaimDescription {
        serde_json::from_value(description).unwrap()
    }

    fn nested(path: serde_json::Value, label: Option<&str>) -> NestedDisplayClaim {
        NestedDisplayClaim {
            path: serde_json::from_value(path).unwrap(),
            label: label.map(ToString::to_string),
            format: None,
        }
    }

    fn training_record() -> serde_json::Value {
        json!({
            "credentialSubject": {
                "Trainee": { "Full name": "Ada Lovelace", "Nationality": "GB" },
                "Websites": ["https://example.com/ada"],
                "Courses": [
                    { "Code": "C101", "Country": "GB" },
                    { "Code": "C102", "Country": "IT" }
                ]
            }
        })
    }

    #[test]
    fn test_get_display_claims_nests_the_descriptions_inside_a_claim_under_it() {
        let display_claims = get_display_claims(
            vec![
                description(json!(["credentialSubject", "Trainee"]), Some("Trainee")),
                description(json!(["credentialSubject", "Trainee", "Full name"]), Some("Full name")),
                description(
                    json!(["credentialSubject", "Trainee", "Nationality"]),
                    Some("Nationality"),
                ),
                description(json!(["credentialSubject", "Websites"]), Some("Websites")),
                description(json!(["credentialSubject", "Websites", null]), None),
                description(json!(["credentialSubject", "Courses"]), Some("Courses")),
                description(json!(["credentialSubject", "Courses", null, "Code"]), Some("Code")),
                description(
                    json!(["credentialSubject", "Courses", null, "Country"]),
                    Some("Country"),
                ),
            ],
            &training_record(),
            &["credentialSubject"],
        );

        let summary: Vec<_> = display_claims
            .iter()
            .map(|claim| (claim.key.as_str(), claim.nested.clone()))
            .collect();
        assert_eq!(
            summary,
            vec![
                (
                    "Trainee",
                    vec![
                        nested(json!(["Full name"]), Some("Full name")),
                        nested(json!(["Nationality"]), Some("Nationality")),
                    ]
                ),
                ("Websites", vec![nested(json!([null]), None)]),
                (
                    "Courses",
                    vec![
                        nested(json!([null, "Code"]), Some("Code")),
                        nested(json!([null, "Country"]), Some("Country")),
                    ]
                ),
            ]
        );
        assert_eq!(
            display_claims[0].value,
            training_record()["credentialSubject"]["Trainee"]
        );
        assert_eq!(
            display_claims[2].path,
            serde_json::from_value::<ClaimPathPointer>(json!(["credentialSubject", "Courses"])).unwrap()
        );
    }

    #[test]
    fn test_get_display_claims_groups_leaves_under_an_object_without_a_description() {
        // Issuers that describe only the leaves of an Object, as UniCore does before impierce/ssi-agent#534.
        let display_claims = get_display_claims(
            vec![
                description(json!(["credentialSubject", "Trainee", "Full name"]), Some("Full name")),
                description(json!(["credentialSubject", "Websites"]), None),
                description(
                    json!(["credentialSubject", "Trainee", "Nationality"]),
                    Some("Nationality"),
                ),
            ],
            &training_record(),
            &["credentialSubject"],
        );

        let summary: Vec<_> = display_claims
            .iter()
            .map(|claim| (claim.key.as_str(), claim.nested.len()))
            .collect();
        assert_eq!(summary, vec![("Trainee", 2), ("Websites", 0)]);
    }

    #[test]
    fn test_get_display_claims_leaves_out_claims_the_credential_does_not_hold() {
        let display_claims = get_display_claims(
            vec![
                description(json!(["credentialSubject", "Trainee"]), Some("Trainee")),
                description(json!(["credentialSubject", "Certificates"]), Some("Certificates")),
            ],
            &training_record(),
            &["credentialSubject"],
        );

        assert_eq!(display_claims.len(), 1);
        assert_eq!(display_claims[0].key, "Trainee");
    }

    #[test]
    fn test_get_display_claims_groups_sd_jwt_vc_claims_from_the_top_level() {
        let display_claims = get_display_claims(
            vec![
                description(json!(["address"]), Some("Place of residence")),
                description(json!(["address", "locality"]), Some("City")),
                description(json!(["given_name"]), Some("Given name")),
            ],
            &json!({ "address": { "locality": "Berlin" }, "given_name": "Erika" }),
            &[],
        );

        let summary: Vec<_> = display_claims
            .iter()
            .map(|claim| (claim.key.as_str(), claim.nested.len()))
            .collect();
        assert_eq!(summary, vec![("Place of residence", 1), ("Given name", 0)]);
    }

    #[test]
    fn test_get_display_claims_ignores_claims_metadata_that_breaks_appendix_b3() {
        let broken = [
            // The same claim twice.
            vec![
                json!(["credentialSubject", "Courses"]),
                json!(["credentialSubject", "Courses"]),
            ],
            // All items of an array, and a single one.
            vec![
                json!(["credentialSubject", "Courses", null, "Code"]),
                json!(["credentialSubject", "Courses", 0, "Code"]),
            ],
            // An array, and an object.
            vec![
                json!(["credentialSubject", "Courses", null, "Code"]),
                json!(["credentialSubject", "Courses", "Code"]),
            ],
        ];

        for paths in broken {
            let descriptions = paths.into_iter().map(|path| description(path, None)).collect();
            assert_eq!(
                get_display_claims(descriptions, &training_record(), &["credentialSubject"]),
                vec![]
            );
        }
    }

    #[test]
    fn test_get_display_claims_takes_the_format_from_the_display_objects() {
        let display_claims = get_display_claims(
            vec![
                from_metadata(json!({
                    "path": ["credentialSubject", "Trainee", "Nationality"],
                    "display": [{ "name": "Nationality", "format": "country" }]
                })),
                from_metadata(
                    json!({ "path": ["credentialSubject", "Websites"], "display": [{ "name": "Websites" }] }),
                ),
                // A display object without `name` (Appendix B.2: OPTIONAL).
                from_metadata(
                    json!({ "path": ["credentialSubject", "Websites", null], "display": [{ "format": "uri" }] }),
                ),
                // The first display object that has a format gives it, whatever its language.
                from_metadata(json!({
                    "path": ["credentialSubject", "Courses"],
                    "display": [
                        { "name": "Kurse", "locale": "de" },
                        { "name": "Courses", "locale": "en", "format": "table" }
                    ]
                })),
            ],
            &training_record(),
            &["credentialSubject"],
        );

        let summary: Vec<_> = display_claims
            .iter()
            .map(|claim| (claim.key.as_str(), claim.format.as_deref(), claim.nested.clone()))
            .collect();
        let with_format = |mut nested: NestedDisplayClaim, format: &str| {
            nested.format = Some(format.to_string());
            nested
        };
        assert_eq!(
            summary,
            vec![
                (
                    "Trainee",
                    None,
                    vec![with_format(
                        nested(json!(["Nationality"]), Some("Nationality")),
                        "country"
                    )]
                ),
                ("Websites", None, vec![with_format(nested(json!([null]), None), "uri")]),
                ("Kurse", Some("table"), vec![]),
            ]
        );
    }

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
        assert_eq!(
            verifiable_credential_record
                .display_credential
                .metadata
                .valid_from
                .as_deref(),
            Some("2022-01-01T00:00:00Z")
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
                    name: Some("First Name".to_string()),
                    locale: None,
                    additional_parameters: Default::default(),
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
                format: None,
                nested: vec![],
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:12:45Z"
        );
        assert_eq!(
            verifiable_credential_record
                .display_credential
                .metadata
                .valid_from
                .as_deref(),
            Some("2026-02-03T20:12:45Z")
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
                    name: Some("First Name".to_string()),
                    locale: None,
                    additional_parameters: Default::default(),
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
                format: None,
                nested: vec![],
            }]
        );
        assert_eq!(
            verifiable_credential_record.display_credential.metadata.date_issued,
            "2026-02-03T20:11:18Z"
        );
        assert_eq!(
            verifiable_credential_record
                .display_credential
                .metadata
                .valid_from
                .as_deref(),
            Some("2026-02-03T20:11:18Z")
        );
    }

    #[test]
    fn test_valid_from_uses_the_latest_of_issuance_date_and_nbf_but_not_iat() {
        let jwt_vc_json = json!("eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJpYXQiOjAsIm5iZiI6NDEwMjQ0NDgwMCwidmMiOnsiaXNzdWFuY2VEYXRlIjoiMjAyMi0wMS0wMVQwMDowMDowMFoifX0.sig");

        assert_eq!(
            valid_from(&CredentialFormats::JwtVcJson(()), &jwt_vc_json).as_deref(),
            Some("2100-01-01T00:00:00+00:00")
        );
    }

    #[test]
    fn test_valid_from_uses_the_issuance_date_when_valid_from_is_invalid() {
        let jwt_vc_json = json!("eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ2YyI6eyJ2YWxpZEZyb20iOiJub3QgYSBkYXRlIiwiaXNzdWFuY2VEYXRlIjoiMjEwMC0wMS0wMVQwMDowMDowMFoifX0.sig");

        assert_eq!(
            valid_from(&CredentialFormats::JwtVcJson(()), &jwt_vc_json).as_deref(),
            Some("2100-01-01T00:00:00Z")
        );
    }
}
