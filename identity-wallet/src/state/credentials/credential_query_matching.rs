use crate::error::AppError;
use crate::state::{connections::Connection, credentials::VerifiableCredentialRecord};
use chrono::{DateTime, Utc};
use identity_credential::sd_jwt_vc::SdJwtVc;
use log::debug;
use oauth_tsl::status_list::StatusType;
use oid4vc::oid4vc_core::utils::jwt::get_unverified_jwt_claims;
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
    /// Whether the verifier rejects a response without this credential. Only queries that sit exclusively in optional
    /// credential sets are not required.
    pub required: bool,
    /// Ids of the stored credentials that satisfy this query.
    pub options: Vec<String>,
    /// The options that were last shared with this verifier for the same credential types.
    pub last_shared: Vec<String>,
}

/// Returns, for every credential query in the DCQL query, the ids of the stored credentials that satisfy it.
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
            required: is_required(dcql_query, credential_query),
            options: verifiable_credentials
                .iter()
                .filter(|record| satisfies_credential_query(credential_query, record))
                .map(|record| record.display_credential.id.clone())
                .collect(),
            last_shared: vec![],
        })
        .collect()
}

/// Fails when a required credential query has no shareable option, or when no query has one at all.
pub fn ensure_shareable(
    requested_credentials: &[RequestedCredential],
    verifiable_credentials: &[VerifiableCredentialRecord],
) -> Result<(), AppError> {
    let has_shareable_option = |requested: &RequestedCredential| {
        verifiable_credentials
            .iter()
            .any(|record| requested.options.contains(&record.display_credential.id) && is_shareable(record))
    };

    let missing: Vec<String> = requested_credentials
        .iter()
        .filter(|requested| requested.required && !has_shareable_option(requested))
        .map(RequestedCredential::type_label)
        .collect();
    if !missing.is_empty() {
        return Err(AppError::MissingRequiredCredentialError(missing.join(", ")));
    }
    if !requested_credentials.iter().any(has_shareable_option) {
        return Err(AppError::NoMatchingCredentialError);
    }
    Ok(())
}

/// Revoked, suspended and not yet valid credentials cannot be shared. Expired ones can, after a warning.
fn is_shareable(record: &VerifiableCredentialRecord) -> bool {
    let display_credential = &record.display_credential;
    let revoked_or_suspended = display_credential
        .credential_status
        .as_ref()
        .is_some_and(|status| matches!(status.status, StatusType::INVALID | StatusType::SUSPENDED));
    let not_yet_valid = DateTime::parse_from_rfc3339(&display_credential.metadata.date_issued)
        .is_ok_and(|valid_from| valid_from > Utc::now());
    !revoked_or_suspended && !not_yet_valid
}

/// Fills in `last_shared` from the credentials previously shared with this connection.
pub fn with_last_shared(
    mut requested_credentials: Vec<RequestedCredential>,
    connection: Option<&Connection>,
) -> Vec<RequestedCredential> {
    if let Some(connection) = connection {
        for requested in &mut requested_credentials {
            if let Some(last_shared) = connection.last_shared_credentials.get(&requested.types_key()) {
                requested.last_shared = last_shared
                    .iter()
                    .filter(|id| requested.options.contains(id))
                    .cloned()
                    .collect();
            }
        }
    }
    requested_credentials
}

impl RequestedCredential {
    pub fn types_key(&self) -> String {
        types_key(&self.types)
    }

    /// A readable name for the requested type, e.g. "Open Badge Credential" for `OpenBadgeCredential`.
    pub fn type_label(&self) -> String {
        let credential_type = self
            .types
            .iter()
            .find(|t| t.as_str() != "VerifiableCredential")
            .or(self.types.first())
            .unwrap_or(&self.query_id);
        let name = credential_type
            .rsplit(['/', '#', ':'])
            .next()
            .unwrap_or(credential_type);
        let mut label = String::new();
        let mut previous_lowercase = false;
        for c in name.chars() {
            if c.is_uppercase() && previous_lowercase {
                label.push(' ');
            }
            previous_lowercase = c.is_lowercase();
            label.push(if c == '_' || c == '-' { ' ' } else { c });
        }
        label
    }
}

pub fn credential_query_types_key(credential_query: &CredentialQuery) -> String {
    types_key(&requested_types(credential_query))
}

fn types_key(types: &[String]) -> String {
    let mut types = types.to_vec();
    types.sort();
    types.join(" ")
}

fn is_required(dcql_query: &DcqlQuery, credential_query: &CredentialQuery) -> bool {
    match &dcql_query.credential_sets {
        None => true,
        Some(credential_sets) => credential_sets.iter().any(|credential_set| {
            credential_set.required.unwrap_or(true)
                && credential_set
                    .options
                    .iter()
                    .all(|option| option.contains(&credential_query.id))
        }),
    }
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

fn credential_data(
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
        (Format::DcSdJwt | Format::VcSdJwt | Format::JwtVcJson, _) => None,
        _ => {
            debug!("Unhandled credential format: {format:?}");
            get_unverified_jwt_claims(verifiable_credential).ok()
        }
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
