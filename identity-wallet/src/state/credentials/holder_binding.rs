//! The holder binding of a credential: the DID or key the issuer bound it to, which only the wallet holding that key
//! can present it with.

use crate::{
    error::AppError,
    state::{SUPPORTED_DID_METHODS, SUPPORTED_SIGNING_ALGORITHMS},
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use log::warn;
use oid4vc_core::{utils::jwt::get_unverified_jwt_claims, Subject};
use oid4vci::credential_format_profiles::CredentialFormats;
use serde_json::Value;

/// What a credential is bound to.
#[derive(Debug, Clone, PartialEq)]
pub enum HolderBinding {
    /// A DID, from the `sub` claim or `credentialSubject.id`, or the DID of the DID URL in `cnf.kid`.
    Did(String),
    /// A public key in `cnf.jwk`.
    Jwk(Value),
}

/// When the wallet checks a credential's holder binding, which decides what the user is told when it fails.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingCheck {
    /// Before storing a credential that was just issued.
    Receive,
    /// Before presenting a stored credential.
    Present,
}

/// The holder binding of `credential`, received in `format`, or `None` for a credential not bound to any holder.
///
/// - `jwt_vc_json`: the `sub` claim, else `vc.credentialSubject.id`, when it is a DID.
/// - `dc+sd-jwt` and `vc+sd-jwt`: the `cnf` claim, a `kid` DID URL or a `jwk`.
pub fn holder_binding(format: &CredentialFormats, credential: &Value) -> Result<Option<HolderBinding>, AppError> {
    let token = credential.as_str().ok_or(AppError::InvalidCredentialFormatError)?;
    let jwt = token.split('~').next().unwrap_or(token);
    let claims = get_unverified_jwt_claims(&Value::String(jwt.to_string()))
        .map_err(|err| AppError::InvalidCredentialError(format!("its claims cannot be read: {err:#}")))?;

    let did = |value: Option<&Value>| {
        value
            .and_then(Value::as_str)
            .filter(|id| id.starts_with("did:"))
            .map(|id| HolderBinding::Did(did_of(id).to_string()))
    };

    Ok(match format {
        CredentialFormats::JwtVcJson(()) => did(claims.get("sub")).or_else(|| {
            did(claims
                .get("vc")
                .and_then(|vc| vc.get("credentialSubject"))
                .and_then(|subject| subject.get("id")))
        }),
        CredentialFormats::DcSdJwt(()) | CredentialFormats::VcSdJwt(()) => {
            let cnf = claims.get("cnf");
            did(cnf.and_then(|cnf| cnf.get("kid")))
                .or_else(|| cnf.and_then(|cnf| cnf.get("jwk")).cloned().map(HolderBinding::Jwk))
        }
        _ => None,
    })
}

/// Fails when `credential` is bound to a DID or key that the wallet does not hold.
pub async fn ensure_held_binding(
    format: &CredentialFormats,
    credential: &Value,
    subject: &dyn Subject,
    check: BindingCheck,
) -> Result<(), AppError> {
    let Some(binding) = holder_binding(format, credential)? else {
        return Ok(());
    };

    let held_dids = held_dids(subject).await;
    let held = match &binding {
        HolderBinding::Did(did) => held_dids.contains(did),
        HolderBinding::Jwk(jwk) => held_dids
            .iter()
            .filter_map(|did| did_jwk_public_key(did))
            .any(|held| same_public_key(&held, jwk)),
    };

    if held {
        return Ok(());
    }

    // The DIDs and keys are not logged: they identify the holder (ADR 0003).
    warn!("The credential is bound to a holder this wallet does not hold ({check:?})");
    Err(AppError::HolderBindingError(match check {
        BindingCheck::Receive => "The credential is bound to a DID or key that this wallet does not hold".to_string(),
        BindingCheck::Present => {
            "The credential is bound to a key this wallet no longer holds, for example after a reset".to_string()
        }
    }))
}

/// Every DID of the wallet: one per supported DID method and signing algorithm.
async fn held_dids(subject: &dyn Subject) -> Vec<String> {
    let mut dids = vec![];
    for method in SUPPORTED_DID_METHODS {
        for algorithm in SUPPORTED_SIGNING_ALGORITHMS {
            if let Ok(did) = subject.identifier(method, *algorithm).await {
                dids.push(did_of(&did).to_string());
            }
        }
    }
    dids
}

/// The DID of a DID or DID URL.
fn did_of(did_url: &str) -> &str {
    did_url.split(['#', '?']).next().unwrap_or(did_url)
}

/// The public key a `did:jwk` encodes.
fn did_jwk_public_key(did: &str) -> Option<Value> {
    let encoded = did.strip_prefix("did:jwk:")?;
    serde_json::from_slice(&URL_SAFE_NO_PAD.decode(encoded).ok()?).ok()
}

/// Whether two JWKs are the same public key.
fn same_public_key(a: &Value, b: &Value) -> bool {
    ["kty", "crv", "x", "y"]
        .iter()
        .all(|member| a.get(member) == b.get(member))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn jwt(claims: Value) -> Value {
        let encode = |value: Value| URL_SAFE_NO_PAD.encode(value.to_string());
        json!(format!("{}.{}.c2ln", encode(json!({"alg": "EdDSA"})), encode(claims)))
    }

    #[test]
    fn jwt_vc_json_is_bound_to_its_sub_or_credential_subject_id() {
        let format = CredentialFormats::JwtVcJson(());

        assert_eq!(
            holder_binding(&format, &jwt(json!({"sub": "did:key:z6MkHolder"}))).unwrap(),
            Some(HolderBinding::Did("did:key:z6MkHolder".to_string()))
        );
        assert_eq!(
            holder_binding(
                &format,
                &jwt(json!({"vc": {"credentialSubject": {"id": "did:key:z6MkHolder#key-1"}}}))
            )
            .unwrap(),
            Some(HolderBinding::Did("did:key:z6MkHolder".to_string()))
        );
        assert_eq!(
            holder_binding(&format, &jwt(json!({"vc": {"credentialSubject": {"name": "Ferris"}}}))).unwrap(),
            None
        );
    }

    #[test]
    fn sd_jwt_is_bound_to_its_cnf_claim() {
        let format = CredentialFormats::DcSdJwt(());
        let sd_jwt = |claims| json!(format!("{}~", jwt(claims).as_str().unwrap()));

        assert_eq!(
            holder_binding(&format, &sd_jwt(json!({"cnf": {"kid": "did:jwk:eyJ9#0"}}))).unwrap(),
            Some(HolderBinding::Did("did:jwk:eyJ9".to_string()))
        );
        assert_eq!(
            holder_binding(&format, &sd_jwt(json!({"cnf": {"jwk": {"kty": "OKP", "x": "abc"}}}))).unwrap(),
            Some(HolderBinding::Jwk(json!({"kty": "OKP", "x": "abc"})))
        );
        assert_eq!(
            holder_binding(&format, &sd_jwt(json!({"iss": "did:key:z6MkIssuer"}))).unwrap(),
            None
        );
    }

    #[test]
    fn a_did_jwk_is_the_same_public_key_as_its_jwk() {
        let jwk = json!({"kty": "OKP", "crv": "Ed25519", "x": "11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"});
        let did = format!("did:jwk:{}", URL_SAFE_NO_PAD.encode(jwk.to_string()));

        let held = did_jwk_public_key(&did).unwrap();

        let mut with_extra_members = jwk.clone();
        with_extra_members["use"] = json!("sig");
        assert!(same_public_key(&held, &with_extra_members));
        assert!(!same_public_key(
            &held,
            &json!({"kty": "OKP", "crv": "Ed25519", "x": "other"})
        ));
    }
}
