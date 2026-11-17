//! Identities proven by X.509 certificate chains, see `docs/adr/0004-identities-proven-by-did-or-x509.md`.

use crate::identity::Identity;
use anyhow::{anyhow, bail, Context as _};
use identity_kit::{
    jose::DefaultVerifier,
    resolver::{JwtValidationOptions, JwtValidator, KeyReference, X509Certificate, X509Resolver},
};
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

/// The X.509 identity of the credential issuer `credential_issuer`, proven by its `signed_metadata` (OID4VCI 1.0
/// §11.2.3): a JWT signed under an `x5c` chain that validates against `trust_anchors`, whose `sub` is the credential
/// issuer and whose leaf certificate has the credential issuer's host as a DNS name.
pub async fn credential_issuer_identity(
    signed_metadata: &str,
    credential_issuer: &Url,
    trust_anchors: &[String],
) -> anyhow::Result<Identity> {
    let options = JwtValidationOptions::new().with_required_claims(["sub", "iat"]);
    let (leaf, claims) = validate(signed_metadata, trust_anchors, &options).await?;

    let subject = claims.sub.context("`signed_metadata` has no `sub`")?;
    if Url::parse(&subject).ok().as_ref() != Some(credential_issuer) {
        bail!("`signed_metadata` is about another credential issuer");
    }
    identity_of(&leaf, credential_issuer)
}

/// Checks that `credential`, a JWT or the issuer-signed JWT of an SD-JWT, is signed under an `x5c` chain that
/// validates against `trust_anchors` and whose leaf certificate proves `identity`, the X.509 identity of the issuer's
/// connection.
pub async fn validate_credential(
    credential: &str,
    identity: &Identity,
    trust_anchors: &[String],
) -> anyhow::Result<()> {
    let Identity::X509 { dns_name } = identity else {
        bail!("The issuer's identity is not proven by an X.509 certificate");
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
        _ => bail!("Only an `https` URL with a DNS name can be proven by a certificate"),
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
