use crate::error::AppError;
use crate::state::{
    connections::Connection,
    credentials::{
        selective_disclosure::{claim_sets, concealable_claims, supports_selective_disclosure},
        VerifiableCredentialRecord,
    },
};
use chrono::{DateTime, Utc};
use identity_credential::sd_jwt_vc::SdJwtVc;
use log::debug;
use oauth_tsl::status_list::StatusType;
use oid4vc::oid4vc_core::{claim_path_pointer::ClaimPathPointer, utils::jwt::get_unverified_jwt_claims};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use oid4vc::oid4vp::{
    dcql::dcql_query::{CredentialQuery, DcqlQuery, Format, MetaTypes},
    dcql_evaluation::evaluate_credential_query,
    token::vp_token_validator::DecodedPresentations,
};
use sd_jwt::{SdJwt, Sha256Hasher};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

/// A credential query from an OID4VP request together with the stored credentials that satisfy it.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/RequestedCredential.ts")]
pub struct RequestedCredential {
    pub query_id: String,
    /// Whether the verifier accepts more than one credential for this query.
    pub multiple: bool,
    /// The credential types the verifier asks for (`vct_values`, the most specific `type_values` or the `doctype_value`).
    pub types: Vec<String>,
    /// The stored credentials that satisfy this query.
    pub options: Vec<CredentialOption>,
    /// The options that were last shared with this verifier for the same credential types.
    pub last_shared: Vec<String>,
    /// The claims the verifier asks for, in the order of the query.
    #[serde(default)]
    #[ts(type = "Array<Array<string | number | null>>")]
    pub claims: Vec<ClaimPathPointer>,
    /// The combinations of `claims` (by index) the verifier accepts, most preferred first. Empty when all claims are
    /// requested.
    #[serde(default)]
    pub claim_sets: Vec<Vec<usize>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/CredentialOption.ts")]
pub struct CredentialOption {
    pub id: String,
    pub state: OptionState,
    /// Whether only the requested claims of this credential are shared, which the holder can then choose from.
    #[serde(default)]
    pub selective_disclosure: bool,
    /// The requested claims as found in this credential, in the order of `RequestedCredential::claims`.
    #[serde(default)]
    pub claims: Vec<OptionClaim>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "bindings/user_prompt/OptionClaim.ts")]
pub struct OptionClaim {
    /// The value at the claim's path, `null` when the credential does not have it and an array when the path
    /// selects several values.
    #[ts(type = "any")]
    pub value: Value,
    /// Whether the claim can be left out of the presentation.
    pub concealable: bool,
}

/// Whether a stored credential can be shared: valid ones are preselected, expired ones can be chosen after a warning,
/// revoked, suspended and not yet valid ones cannot be chosen.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, export_to = "bindings/user_prompt/OptionState.ts")]
pub enum OptionState {
    Valid,
    Expired,
    Unavailable,
}

/// Returns, for every credential query in the DCQL query, the stored credentials that satisfy it.
pub fn match_credential_queries(
    dcql_query: &DcqlQuery,
    verifiable_credentials: &[VerifiableCredentialRecord],
) -> Vec<RequestedCredential> {
    dcql_query
        .credentials
        .iter()
        .map(|credential_query| RequestedCredential {
            query_id: credential_query.id.to_string(),
            multiple: credential_query.multiple.unwrap_or(false),
            types: requested_types(credential_query),
            options: verifiable_credentials
                .iter()
                .filter(|record| satisfies_credential_query(credential_query, record))
                .map(|record| credential_option(credential_query, record))
                .collect(),
            last_shared: vec![],
            claims: credential_query
                .claims
                .iter()
                .flatten()
                .map(|claim| claim.path.clone())
                .collect(),
            claim_sets: claim_sets(credential_query),
        })
        .collect()
}

/// Fails when a credential query has no option that can be shared, since the verifier requires all of them.
pub fn ensure_shareable(requested_credentials: &[RequestedCredential]) -> Result<(), AppError> {
    let missing: Vec<String> = requested_credentials
        .iter()
        .filter(|requested| {
            requested
                .options
                .iter()
                .all(|option| option.state == OptionState::Unavailable)
        })
        .map(|requested| requested.types.join(", "))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(AppError::MissingRequiredCredentialError(missing.join("; ")))
    }
}

fn credential_option(credential_query: &CredentialQuery, record: &VerifiableCredentialRecord) -> CredentialOption {
    let claim_paths: Vec<&ClaimPathPointer> = credential_query
        .claims
        .iter()
        .flatten()
        .map(|claim| &claim.path)
        .collect();
    let selective_disclosure = supports_selective_disclosure(&credential_query.format) && !claim_paths.is_empty();
    let concealable = match record.verifiable_credential.as_str() {
        Some(sd_jwt) if selective_disclosure => concealable_claims(sd_jwt, &claim_paths),
        _ => vec![false; claim_paths.len()],
    };
    let credential_json = credential_data(credential_query, record).unwrap_or_default();
    let claims = claim_paths
        .iter()
        .zip(concealable)
        .map(|(path, concealable)| {
            let mut values = path.get_values_from_json(&credential_json);
            let value = match values.len() {
                0 => Value::Null,
                1 => values.remove(0),
                _ => Value::Array(values),
            };
            OptionClaim { value, concealable }
        })
        .collect();

    CredentialOption {
        id: record.display_credential.id.clone(),
        state: option_state(record),
        selective_disclosure,
        claims,
    }
}

fn option_state(record: &VerifiableCredentialRecord) -> OptionState {
    let display_credential = &record.display_credential;
    let now = Utc::now();
    let revoked_or_suspended = display_credential
        .credential_status
        .as_ref()
        .is_some_and(|status| matches!(status.status, StatusType::INVALID | StatusType::SUSPENDED));
    let not_yet_valid = display_credential
        .metadata
        .valid_from
        .as_deref()
        .and_then(|valid_from| DateTime::parse_from_rfc3339(valid_from).ok())
        .is_some_and(|valid_from| valid_from > now);
    let expired = display_credential
        .metadata
        .expiration_date
        .as_deref()
        .and_then(|valid_until| DateTime::parse_from_rfc3339(valid_until).ok())
        .is_some_and(|valid_until| valid_until <= now);

    if revoked_or_suspended {
        OptionState::Unavailable
    } else if expired {
        OptionState::Expired
    } else if not_yet_valid {
        OptionState::Unavailable
    } else {
        OptionState::Valid
    }
}

/// Fills in `last_shared` from the credentials previously shared with this connection.
pub fn with_last_shared(
    mut requested_credentials: Vec<RequestedCredential>,
    connection: Option<&Connection>,
) -> Vec<RequestedCredential> {
    if let Some(connection) = connection {
        for requested in &mut requested_credentials {
            if let Some(last_shared) = connection.last_shared_credentials.get(&types_key(&requested.types)) {
                requested.last_shared = last_shared
                    .iter()
                    .filter(|id| requested.options.iter().any(|option| &option.id == *id))
                    .cloned()
                    .collect();
            }
        }
    }
    requested_credentials
}

/// The key under which the credentials shared for a credential query are remembered per connection.
pub fn credential_query_types_key(credential_query: &CredentialQuery) -> String {
    types_key(&requested_types(credential_query))
}

fn types_key(types: &[String]) -> String {
    let mut types = types.to_vec();
    types.sort();
    types.join(" ")
}

pub fn satisfies_credential_query(
    credential_query: &CredentialQuery,
    verifiable_credential_record: &VerifiableCredentialRecord,
) -> bool {
    let Some(credential_data) = credential_data(credential_query, verifiable_credential_record) else {
        return false;
    };
    let Some(credential_object) = credential_data.as_object() else {
        return false;
    };
    // `evaluate_credential_query` skips the `meta` check when the query requests no claims.
    if credential_query.claims.as_deref().unwrap_or_default().is_empty()
        && !matches_meta(credential_query, &credential_data)
    {
        return false;
    }
    match DecodedPresentations::try_new(vec![credential_object.clone()]) {
        Ok(decoded_presentations) => evaluate_credential_query(credential_query, &decoded_presentations),
        Err(e) => {
            debug!(
                "Failed to decode credential into DecodedPresentations; id: {:?}, format: {:?}, error: {:?}",
                verifiable_credential_record.display_credential.id,
                verifiable_credential_record.display_credential.format,
                e
            );
            false
        }
    }
}

pub fn credential_data(
    credential_query: &CredentialQuery,
    verifiable_credential_record: &VerifiableCredentialRecord,
) -> Option<Value> {
    let format = &verifiable_credential_record.display_credential.format;
    let verifiable_credential = &verifiable_credential_record.verifiable_credential;

    match (&credential_query.format, format) {
        (Format::DcSdJwt, CredentialFormats::DcSdJwt(())) => Some(serde_json::json!(verifiable_credential
            .as_str()?
            .parse::<SdJwtVc>()
            .ok()?
            .into_disclosed_object(&Sha256Hasher::new())
            .ok()?)),
        (Format::VcSdJwt, CredentialFormats::VcSdJwt(())) => Some(serde_json::json!(verifiable_credential
            .as_str()?
            .parse::<SdJwt>()
            .ok()?
            .into_disclosed_object(&Sha256Hasher::new())
            .ok()?)),
        // JWT_VC_JSON must be accessed from the vc values.
        (Format::JwtVcJson, CredentialFormats::JwtVcJson(())) => {
            let full_jwt_payload = get_unverified_jwt_claims(verifiable_credential).unwrap_or_default();
            Some(full_jwt_payload.get("vc").cloned().unwrap_or_else(|| {
                debug!("JWT-VC-JSON is missing `vc` claims or is not a valid JSON value: {full_jwt_payload:?}");
                serde_json::json!({})
            }))
        }
        // Other formats cannot be presented (yet), so they never match.
        _ => None,
    }
}

fn matches_meta(credential_query: &CredentialQuery, credential_data: &Value) -> bool {
    match &credential_query.meta {
        MetaTypes::SdJwtMeta { vct_values } => credential_data
            .get("vct")
            .and_then(Value::as_str)
            .is_some_and(|vct| vct_values.iter().any(|value| value == vct)),
        MetaTypes::W3CFormatMeta { type_values } => {
            let types: Vec<&str> = credential_data
                .get("type")
                .and_then(Value::as_array)
                .map(|types| types.iter().filter_map(Value::as_str).collect())
                .unwrap_or_default();
            type_values
                .iter()
                .any(|option| option.iter().all(|required| types.contains(&required.as_str())))
        }
        MetaTypes::MsoMdocMeta { .. } => true,
    }
}

fn requested_types(credential_query: &CredentialQuery) -> Vec<String> {
    match &credential_query.meta {
        MetaTypes::SdJwtMeta { vct_values } => vct_values.clone(),
        MetaTypes::W3CFormatMeta { type_values } => {
            type_values.iter().filter_map(|types| types.last().cloned()).collect()
        }
        MetaTypes::MsoMdocMeta { doctype_value } => vec![doctype_value.clone()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::credentials::{CredentialMetadata, DisplayCredential};
    use serde_json::json;

    fn record(date_issued: &str, valid_from: Option<&str>) -> VerifiableCredentialRecord {
        VerifiableCredentialRecord {
            verifiable_credential: serde_json::Value::Null,
            display_credential: DisplayCredential {
                metadata: CredentialMetadata {
                    date_issued: date_issued.to_string(),
                    valid_from: valid_from.map(ToString::to_string),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }

    #[test]
    fn option_state_uses_valid_from_for_not_yet_valid() {
        assert_eq!(
            option_state(&record("2020-01-01T00:00:00Z", Some("2100-01-01T00:00:00Z"))),
            OptionState::Unavailable
        );
        assert_eq!(
            option_state(&record("2020-01-01T00:00:00Z", Some("2020-01-01T00:00:00Z"))),
            OptionState::Valid
        );
    }

    #[test]
    fn option_state_checks_expiry_before_valid_from() {
        let mut expired_and_not_yet_valid = record("2020-01-01T00:00:00Z", Some("2100-01-01T00:00:00Z"));
        expired_and_not_yet_valid.display_credential.metadata.expiration_date =
            Some("2021-01-01T00:00:00Z".to_string());
        assert_eq!(option_state(&expired_and_not_yet_valid), OptionState::Expired);
    }

    #[test]
    fn option_state_ignores_the_issuance_date() {
        assert_eq!(option_state(&record("2100-01-01T00:00:00Z", None)), OptionState::Valid);
        assert_eq!(
            option_state(&record("2100-01-01T00:00:00Z", Some("not a date"))),
            OptionState::Valid
        );
    }

    fn query(meta: Value) -> CredentialQuery {
        serde_json::from_value(json!({ "id": "badge", "format": "vc+sd-jwt", "meta": meta })).unwrap()
    }

    #[test]
    fn matches_meta_requires_all_types_of_one_option() {
        let badge = query(json!({ "type_values": [["VerifiableCredential", "OpenBadgeCredential"]] }));

        assert!(matches_meta(
            &badge,
            &json!({ "type": ["VerifiableCredential", "OpenBadgeCredential"] })
        ));
        assert!(!matches_meta(&badge, &json!({ "type": ["VerifiableCredential"] })));
        assert!(!matches_meta(&badge, &json!({})));
    }

    #[test]
    fn matches_meta_compares_vct() {
        let pid = query(json!({ "vct_values": ["urn:eudi:pid:1"] }));

        assert!(matches_meta(&pid, &json!({ "vct": "urn:eudi:pid:1" })));
        assert!(!matches_meta(&pid, &json!({ "vct": "urn:example:email" })));
    }
}
