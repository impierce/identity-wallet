//! Identities verified with X.509 certificate chains, see `docs/adr/0004-identities-verified-with-did-or-x509.md`.

use crate::identity::Identity;
use crate::logging::url_host;
use anyhow::{anyhow, bail, Context as _};
use identity_kit::{
    jose::DefaultVerifier,
    resolver::{JwtValidationOptions, JwtValidator, KeyReference, X509Certificate, X509Resolver},
};
use log::warn;
use oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;
use serde_json::{Map, Value};
use url::Url;

/// A resolver that validates `x5c` chains against `trust_anchors`, CA certificates in PEM.
fn resolver(trust_anchors: &[String]) -> anyhow::Result<X509Resolver> {
    X509Resolver::from_pem(&trust_anchors.join("\n")).context("Invalid trust anchor")
}

/// Validates `token`, a compact JWS whose header carries an `x5c` chain, against `trust_anchors`, and returns its leaf
/// certificate and claims. An `iss` claim must be named by the leaf (identity-kit's issuer binding for `x5c`).
async fn validate(
    token: &str,
    trust_anchors: &[String],
    options: &JwtValidationOptions,
) -> anyhow::Result<(X509Certificate, identity_kit::jose::JwtClaims)> {
    let validator = JwtValidator::new(resolver(trust_anchors)?, DefaultVerifier);
    let jwt = validator.validate(token, options).await?;
    let KeyReference::X509Chain(chain) = &jwt.key_reference else {
        bail!("The token is not signed under an `x5c` certificate chain");
    };
    let leaf = X509Certificate::from_der(chain.first().context("Empty `x5c` chain")?)?;
    Ok((leaf, jwt.claims))
}

/// The X.509 identity of the credential issuer `credential_issuer`, verified with its `signed_metadata` (OID4VCI 1.0
/// §11.2.3): a JWT signed under an `x5c` chain that validates against `trust_anchors`, whose `sub` is the credential
/// issuer and whose leaf certificate has the credential issuer's host as a DNS name.
pub async fn credential_issuer_identity(
    signed_metadata: &str,
    credential_issuer: &Url,
    trust_anchors: &[String],
) -> anyhow::Result<Identity> {
    verify_signed_metadata(signed_metadata, credential_issuer, trust_anchors)
        .await
        .map(|(identity, _)| identity)
}

/// The credential issuer metadata of `credential_issuer` from `metadata`, its JSON, with the values of its
/// `signed_metadata` taking precedence over the unsigned ones (OID4VCI 1.0 §11.2.3), when the signed metadata verifies
/// the credential issuer under `trust_anchors` as [`credential_issuer_identity`] does. A signed metadata parameter
/// replaces the unsigned one of the same name; the JWT's own claims (`iss`, `sub`, `iat` and the like) are no metadata.
/// The values are merged before the metadata is parsed, since an issuer that enforces signed metadata leaves its
/// parameters out of the unsigned metadata.
///
/// Without signed metadata or trust anchors, or when the signed metadata does not verify, `metadata` is parsed as it
/// is: its values are then not vouched for by a certificate, and the issuer is verified otherwise, if at all. Signed
/// metadata that verifies but gives no valid metadata is an error: the issuer's certificate vouches for those values,
/// so the unsigned ones are not used in their place.
pub async fn apply_signed_metadata(
    metadata: Map<String, Value>,
    credential_issuer: &Url,
    trust_anchors: &[String],
) -> anyhow::Result<AppliedMetadata> {
    if let Some(signed) = verified_signed_metadata(&metadata, credential_issuer, trust_anchors).await {
        let mut merged = metadata;
        merged.extend(signed);
        let metadata = serde_json::from_value(Value::Object(merged)).with_context(|| {
            format!(
                "The signed metadata of {} verifies, but gives no valid credential issuer metadata",
                url_host(credential_issuer)
            )
        })?;
        return Ok(AppliedMetadata { metadata, signed: true });
    }
    let metadata = serde_json::from_value(Value::Object(metadata))
        .with_context(|| format!("Failed to parse credential issuer metadata of {credential_issuer}"))?;
    Ok(AppliedMetadata {
        metadata,
        signed: false,
    })
}

/// Credential issuer metadata, see [`apply_signed_metadata`].
#[derive(Debug)]
pub struct AppliedMetadata {
    pub metadata: CredentialIssuerMetadata,
    /// Whether the values of a verified `signed_metadata` were applied: its certificate vouches for them.
    pub signed: bool,
}

/// The metadata parameters of the `signed_metadata` of `metadata`, if it verifies `credential_issuer` under
/// `trust_anchors`.
async fn verified_signed_metadata(
    metadata: &Map<String, Value>,
    credential_issuer: &Url,
    trust_anchors: &[String],
) -> Option<Map<String, Value>> {
    let signed_metadata = metadata.get("signed_metadata")?.as_str()?;
    if trust_anchors.is_empty() {
        return None;
    }
    match verify_signed_metadata(signed_metadata, credential_issuer, trust_anchors).await {
        Ok((_, claims)) => Some(claims.additional.into_iter().collect()),
        Err(err) => {
            warn!(
                "The signed metadata of {} is not applied: {err:#}",
                url_host(credential_issuer)
            );
            None
        }
    }
}

/// Validates `signed_metadata` for `credential_issuer` and returns the identity it verifies and its claims.
async fn verify_signed_metadata(
    signed_metadata: &str,
    credential_issuer: &Url,
    trust_anchors: &[String],
) -> anyhow::Result<(Identity, identity_kit::jose::JwtClaims)> {
    let options = JwtValidationOptions::new().with_required_claims(["sub", "iat"]);
    let (leaf, claims) = validate(signed_metadata, trust_anchors, &options).await?;

    let subject = claims.sub.as_deref().context("`signed_metadata` has no `sub`")?;
    if Url::parse(subject).ok().as_ref() != Some(credential_issuer) {
        bail!("`signed_metadata` is about another credential issuer");
    }
    Ok((identity_of(&leaf, credential_issuer)?, claims))
}

/// Checks that `credential`, a JWT or the issuer-signed JWT of an SD-JWT, is signed under an `x5c` chain that
/// validates against `trust_anchors` and whose leaf certificate certifies `identity`, the X.509 identity of the
/// issuer's connection.
pub async fn validate_credential(
    credential: &str,
    identity: &Identity,
    trust_anchors: &[String],
) -> anyhow::Result<()> {
    let Identity::X509 { dns_name } = identity else {
        bail!("The issuer's identity is not verified with an X.509 certificate");
    };
    // The issuer-signed JWT of an SD-JWT is the part before the first disclosure.
    let issuer_jwt = credential.split('~').next().unwrap_or(credential);

    let (leaf, _) = validate(issuer_jwt, trust_anchors, &JwtValidationOptions::new()).await?;
    if !has_dns_name(&leaf, dns_name) {
        bail!("The credential is signed by a certificate of another party");
    }
    Ok(())
}

/// The X.509 identity `url` has when `leaf` has its host as a DNS name.
fn identity_of(leaf: &X509Certificate, url: &Url) -> anyhow::Result<Identity> {
    let host = match url.host() {
        Some(url::Host::Domain(host)) if url.scheme() == "https" => host.to_ascii_lowercase(),
        _ => bail!("Only an `https` URL with a DNS name can be verified with a certificate"),
    };
    if !has_dns_name(leaf, &host) {
        return Err(anyhow!("The certificate does not name `{host}`"));
    }
    Ok(Identity::X509 { dns_name: host })
}

fn has_dns_name(leaf: &X509Certificate, dns_name: &str) -> bool {
    leaf.dns_names().iter().any(|name| name.eq_ignore_ascii_case(dns_name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::{engine::general_purpose::STANDARD, Engine as _};
    use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
    use p256::pkcs8::{EncodePrivateKey as _, LineEnding};
    use serde_json::{json, Value};

    /// The chain of `tests/fixtures/x509-limbo`: a leaf for `example.com` under an intermediate, and its root.
    struct Signer {
        x5c: Vec<String>,
        trust_anchor: String,
        key: EncodingKey,
    }

    fn signer() -> Signer {
        let signer: Value = serde_json::from_str(include_str!("../tests/fixtures/x509-limbo/signer.json")).unwrap();
        let x5c = serde_json::from_value(signer["x5c"].clone()).unwrap();

        let root = STANDARD.decode(signer["trust_anchor"].as_str().unwrap()).unwrap();
        let lines: Vec<String> = STANDARD
            .encode(root)
            .as_bytes()
            .chunks(64)
            .map(|line| String::from_utf8(line.to_vec()).unwrap())
            .collect();
        let trust_anchor = format!(
            "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
            lines.join("\n")
        );

        let secret_key = p256::SecretKey::from_jwk_str(&signer["private_jwk"].to_string()).unwrap();
        let pem = secret_key.to_pkcs8_pem(LineEnding::LF).unwrap();
        let key = EncodingKey::from_ec_pem(pem.as_bytes()).unwrap();

        Signer { x5c, trust_anchor, key }
    }

    impl Signer {
        fn sign(&self, claims: Value) -> String {
            let mut header = Header::new(Algorithm::ES256);
            header.x5c = Some(self.x5c.clone());
            encode(&header, &claims, &self.key).unwrap()
        }
    }

    fn now() -> i64 {
        chrono::Utc::now().timestamp()
    }

    fn example_com() -> Identity {
        Identity::X509 {
            dns_name: "example.com".to_owned(),
        }
    }

    #[tokio::test]
    async fn signed_metadata_proves_the_credential_issuer() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({ "sub": "https://example.com", "iat": now() }));

        let identity = credential_issuer_identity(
            &signed_metadata,
            &credential_issuer,
            std::slice::from_ref(&signer.trust_anchor),
        )
        .await
        .unwrap();

        assert_eq!(identity, example_com());
        assert_eq!(identity.to_string(), "x509_san_dns:example.com");
    }

    fn unsigned_metadata(signed_metadata: Option<String>) -> Map<String, Value> {
        let mut metadata: Value = json!({
            "credential_issuer": "https://example.com",
            "credential_endpoint": "https://example.com/retired",
            "credential_configurations_supported": {},
            "display": [{ "name": "Unsigned" }]
        });
        if let Some(signed_metadata) = signed_metadata {
            metadata["signed_metadata"] = json!(signed_metadata);
        }
        metadata.as_object().unwrap().clone()
    }

    #[tokio::test]
    async fn signed_metadata_takes_precedence_over_the_unsigned_metadata() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({
            "sub": "https://example.com",
            "iat": now(),
            "credential_endpoint": "https://example.com/credential"
        }));
        let anchors = std::slice::from_ref(&signer.trust_anchor);

        let applied = apply_signed_metadata(unsigned_metadata(Some(signed_metadata)), &credential_issuer, anchors)
            .await
            .unwrap();
        assert!(applied.signed);
        let metadata = applied.metadata;

        assert_eq!(metadata.credential_endpoint.as_str(), "https://example.com/credential");
        // What the signed metadata does not name stays as the unsigned metadata has it.
        assert_eq!(metadata.display.unwrap()[0]["name"], "Unsigned");
    }

    #[tokio::test]
    async fn signed_metadata_supplies_the_parameters_the_unsigned_metadata_leaves_out() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({
            "sub": "https://example.com",
            "iat": now(),
            "credential_endpoint": "https://example.com/credential"
        }));
        let mut metadata = unsigned_metadata(Some(signed_metadata));
        metadata.remove("credential_endpoint");

        let applied = apply_signed_metadata(
            metadata.clone(),
            &credential_issuer,
            std::slice::from_ref(&signer.trust_anchor),
        )
        .await
        .unwrap();
        assert_eq!(
            applied.metadata.credential_endpoint.as_str(),
            "https://example.com/credential"
        );

        // Without the signed values, the metadata is incomplete.
        assert!(apply_signed_metadata(metadata, &credential_issuer, &[]).await.is_err());
    }

    #[tokio::test]
    async fn signed_metadata_that_does_not_verify_is_not_applied() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({
            "sub": "https://other.example",
            "iat": now(),
            "credential_endpoint": "https://example.com/credential"
        }));

        for anchors in [vec![], vec![signer.trust_anchor.clone()]] {
            let metadata = apply_signed_metadata(
                unsigned_metadata(Some(signed_metadata.clone())),
                &credential_issuer,
                &anchors,
            )
            .await
            .unwrap();
            assert!(!metadata.signed);
            assert_eq!(
                metadata.metadata.credential_endpoint.as_str(),
                "https://example.com/retired"
            );
        }
    }

    #[tokio::test]
    async fn signed_metadata_that_verifies_but_gives_no_valid_metadata_is_an_error() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({
            "sub": "https://example.com",
            "iat": now(),
            "credential_endpoint": "not a URL"
        }));
        let metadata = unsigned_metadata(Some(signed_metadata));

        // Not the unsigned `credential_endpoint` in its place.
        assert!(apply_signed_metadata(
            metadata.clone(),
            &credential_issuer,
            std::slice::from_ref(&signer.trust_anchor)
        )
        .await
        .is_err());
        // Without a trust anchor, the signed metadata does not verify and the unsigned metadata is used.
        let unsigned = apply_signed_metadata(metadata, &credential_issuer, &[]).await.unwrap();
        assert_eq!(
            unsigned.metadata.credential_endpoint.as_str(),
            "https://example.com/retired"
        );
    }

    #[tokio::test]
    async fn signed_metadata_proves_nothing_without_a_trust_anchor() {
        let signer = signer();
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        let signed_metadata = signer.sign(json!({ "sub": "https://example.com", "iat": now() }));

        assert!(credential_issuer_identity(&signed_metadata, &credential_issuer, &[])
            .await
            .is_err());
    }

    #[tokio::test]
    async fn signed_metadata_must_be_about_the_credential_issuer_and_named_by_the_certificate() {
        let signer = signer();
        let anchors = [signer.trust_anchor.clone()];

        // About another credential issuer.
        let signed_metadata = signer.sign(json!({ "sub": "https://other.example", "iat": now() }));
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        assert!(
            credential_issuer_identity(&signed_metadata, &credential_issuer, &anchors)
                .await
                .is_err()
        );

        // About a credential issuer the certificate does not name.
        let credential_issuer: Url = "https://other.example".parse().unwrap();
        assert!(
            credential_issuer_identity(&signed_metadata, &credential_issuer, &anchors)
                .await
                .is_err()
        );

        // An `iss` the certificate does not name: a DID is never bound by a certificate.
        let signed_metadata =
            signer.sign(json!({ "iss": "did:web:example.com", "sub": "https://example.com", "iat": now() }));
        let credential_issuer: Url = "https://example.com".parse().unwrap();
        assert!(
            credential_issuer_identity(&signed_metadata, &credential_issuer, &anchors)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn credentials_must_be_signed_under_a_certificate_of_the_identity() {
        let signer = signer();
        let anchors = [signer.trust_anchor.clone()];
        let credential = signer.sign(json!({ "iss": "https://example.com", "iat": now(), "vc": {} }));

        validate_credential(&credential, &example_com(), &anchors)
            .await
            .unwrap();

        // An SD-JWT: the issuer-signed JWT, then disclosures.
        let sd_jwt = format!("{credential}~WyJzYWx0IiwgImdpdmVuX25hbWUiLCAiRXJpa2EiXQ~");
        validate_credential(&sd_jwt, &example_com(), &anchors).await.unwrap();

        // Another X.509 identity, a DID identity, no trust anchors, or no chain at all.
        let other = Identity::X509 {
            dns_name: "other.example".to_owned(),
        };
        assert!(validate_credential(&credential, &other, &anchors).await.is_err());
        let did = Identity::did("did:web:example.com").unwrap();
        assert!(validate_credential(&credential, &did, &anchors).await.is_err());
        assert!(validate_credential(&credential, &example_com(), &[]).await.is_err());
        let unsigned = encode(&Header::new(Algorithm::ES256), &json!({ "iat": now() }), &signer.key).unwrap();
        assert!(validate_credential(&unsigned, &example_com(), &anchors).await.is_err());
    }
}
