use std::{collections::HashMap, sync::Arc};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use identity_iota::core::Timestamp;
use log::{info, warn};
use oid4vc::oid4vci::credential_format_profiles::CredentialFormats;
use serde_json::{json, Map, Value};
use uuid::Uuid;

use crate::{
    error::AppError,
    state::credentials::{reducers::self_issue_credential::sign_self_issued_sd_jwt, VerifiableCredentialRecord},
    stronghold::StrongholdManager,
};

/// The `vct` that UniMe up to 0.18 put in every self-issued `dc+sd-jwt`.
const LEGACY_SELF_ISSUED_VCT: &str = "https://www.ietf.org/archive/id/draft-terbu-oauth-sd-jwt-vc-00.html";
const SELF_ISSUED_TYPES: [&str; 2] = ["NaturalPersonCredential", "ResidenceCredential"];

/// A Profile or Address credential that an older UniMe self-issued as `dc+sd-jwt`.
#[derive(Debug, PartialEq)]
pub struct LegacySelfIssuedCredential {
    pub did_method: String,
    pub issuer_did: String,
    pub key_type: String,
    pub credential_type: String,
    pub credential_subject: Map<String, Value>,
    pub valid_from: Timestamp,
}

/// Recognizes a legacy self-issued credential by its format, placeholder `vct`, type and an issuer that is one of the
/// holder's own DIDs. Older versions concealed no claims, so everything needed to re-sign it is in the issuer-signed
/// JWT payload.
pub fn legacy_self_issued_credential(
    record: &VerifiableCredentialRecord,
    dids: &HashMap<String, String>,
) -> Option<LegacySelfIssuedCredential> {
    if !matches!(record.display_credential.format, CredentialFormats::DcSdJwt(())) {
        return None;
    }

    let issuer_signed_jwt = record.verifiable_credential.as_str()?.split('~').next()?;
    let mut segments = issuer_signed_jwt.split('.');
    let header = decode_segment(segments.next()?)?;
    let payload = decode_segment(segments.next()?)?;

    if payload.get("vct").and_then(Value::as_str) != Some(LEGACY_SELF_ISSUED_VCT) {
        return None;
    }

    let issuer_did = payload.get("iss").and_then(Value::as_str)?;
    let did_method = dids
        .iter()
        .find_map(|(did_method, did)| (did == issuer_did).then(|| did_method.clone()))?;

    let credential_type = payload
        .get("type")
        .and_then(Value::as_array)?
        .iter()
        .filter_map(Value::as_str)
        .find(|credential_type| SELF_ISSUED_TYPES.contains(credential_type))?;

    let mut credential_subject = payload.get("credentialSubject").and_then(Value::as_object)?.clone();
    credential_subject.remove("id");

    let valid_from = payload
        .get("iat")
        .and_then(Value::as_i64)
        .and_then(|iat| Timestamp::from_unix(iat).ok())
        .unwrap_or_else(Timestamp::now_utc);

    Some(LegacySelfIssuedCredential {
        did_method,
        issuer_did: issuer_did.to_string(),
        key_type: header.get("alg").and_then(Value::as_str)?.to_string(),
        credential_type: credential_type.to_string(),
        credential_subject,
        valid_from,
    })
}

fn decode_segment(segment: &str) -> Option<Map<String, Value>> {
    let bytes = URL_SAFE_NO_PAD.decode(segment).ok()?;
    serde_json::from_slice(&bytes).ok()
}

/// Re-signs the legacy self-issued credentials in the Stronghold as `vc+sd-jwt`, keeping their id, name and metadata.
/// A credential that cannot be migrated is left as it is, so that unlocking never fails because of it.
pub async fn migrate_self_issued_credentials(
    stronghold_manager: &StrongholdManager,
    subject: Arc<dyn oid4vc::oid4vc_core::Subject>,
    dids: &HashMap<String, String>,
) {
    let records = match stronghold_manager.values() {
        Ok(records) => records.unwrap_or_default(),
        Err(e) => {
            warn!("Failed to read credentials to migrate self-issued credentials: {e}");
            return;
        }
    };

    for record in records {
        let Some(legacy) = legacy_self_issued_credential(&record, dids) else {
            continue;
        };
        let id = record.display_credential.id.clone();

        match migrate_record(record, legacy, subject.clone(), stronghold_manager).await {
            Ok(()) => info!("Migrated self-issued credential {id} from dc+sd-jwt to vc+sd-jwt"),
            Err(e) => warn!("Failed to migrate self-issued credential {id}, keeping it as dc+sd-jwt: {e}"),
        }
    }
}

async fn migrate_record(
    mut record: VerifiableCredentialRecord,
    legacy: LegacySelfIssuedCredential,
    subject: Arc<dyn oid4vc::oid4vc_core::Subject>,
    stronghold_manager: &StrongholdManager,
) -> Result<(), AppError> {
    let key: Uuid = record
        .display_credential
        .id
        .parse()
        .map_err(|_| AppError::Error("Credential id is not a UUID".to_string()))?;

    let self_issued = sign_self_issued_sd_jwt(
        subject,
        &legacy.did_method,
        &legacy.issuer_did,
        &legacy.key_type,
        &legacy.credential_type,
        legacy.credential_subject,
        legacy.valid_from,
    )
    .await?;

    let mut display_data = Map::from_iter([("name".to_string(), json!(record.display_credential.data.get("name")))]);
    display_data.extend(self_issued.data);

    record.verifiable_credential = json!(self_issued.sd_jwt);
    record.display_credential.format = CredentialFormats::VcSdJwt(());
    record.display_credential.data = Value::Object(display_data);

    stronghold_manager
        .insert(key, json!(record).to_string().into_bytes())
        .map_err(AppError::StrongholdInsertionError)
}
