use crate::state::{connections::Connection, credentials::VerifiableCredentialRecord};
use identity_credential::sd_jwt_vc::SdJwtVc;
use log::debug;
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
