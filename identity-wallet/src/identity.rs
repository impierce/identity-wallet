//! Who a party the wallet connects to is, and how the wallet verified it, see
//! `docs/adr/0004-identities-verified-with-did-or-x509.md`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use ts_rs::TS;

/// The prefix of an identity verified with an X.509 certificate: the client identifier prefix of OpenID4VP 1.0
/// §5.9.3 for a party named by a `dNSName` of its certificate.
const X509_SAN_DNS: &str = "x509_san_dns:";

/// Who a party is, as the wallet has verified it: by a DID, or by an X.509 certificate chain that validates against one
/// of the wallet's trust anchors and whose leaf certificate has the DNS name.
///
/// Serialized as a string: a DID as the DID, an X.509 identity as `x509_san_dns:<DNS name>`. Connections stored before
/// X.509 identities existed hold a DID string, which reads as [`Identity::Did`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Identity {
    /// A DID, verified with its DID document.
    Did(String),
    /// A DNS name, verified with an X.509 certificate chain.
    X509 { dns_name: String },
}

impl Identity {
    /// The identity of `did`, checking that it is a DID.
    pub fn did(did: &str) -> Result<Self, identity_kit::did::DidError> {
        Ok(Self::Did(identity_kit::did::Did::parse(did)?.to_string()))
    }

    /// The DID, for an identity verified with one.
    pub fn as_did(&self) -> Option<&str> {
        match self {
            Self::Did(did) => Some(did),
            Self::X509 { .. } => None,
        }
    }
}

/// An empty DID, as the `did` of a connection stored without one read before.
impl Default for Identity {
    fn default() -> Self {
        Self::Did(String::new())
    }
}

impl fmt::Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Did(did) => f.write_str(did),
            Self::X509 { dns_name } => write!(f, "{X509_SAN_DNS}{dns_name}"),
        }
    }
}

impl From<&str> for Identity {
    /// Reads a stored identity. A DID is not checked again: it was checked when the identity was verified.
    fn from(identity: &str) -> Self {
        match identity.strip_prefix(X509_SAN_DNS) {
            Some(dns_name) => Self::X509 {
                dns_name: dns_name.to_owned(),
            },
            None => Self::Did(identity.to_owned()),
        }
    }
}

impl Serialize for Identity {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Identity {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from(String::deserialize(deserializer)?.as_str()))
    }
}

/// How the wallet verified an identity of a party.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/connections/ProofMethod.ts")]
#[serde(rename_all = "snake_case")]
pub enum ProofMethod {
    /// A DID document served at the party's DNS name, `/.well-known/did.json` (ADR 0001).
    WellKnownDidDocument,
    /// `signed_metadata` signed under an `x5c` certificate chain that validates against a trust anchor and whose leaf
    /// certificate has the party's DNS name (OID4VCI 1.0 §11.2.3).
    SignedMetadata,
    /// A request signed with a key of the DID (SIOPv2, OpenID4VP).
    SignedRequest,
    /// The DID that signs the credentials of an issuer that is not verified otherwise (ADR 0005). It verifies nobody: it
    /// only records the DID, so that later credentials of the issuer are held to it.
    CredentialSignature,
    /// Verified by a version of UniMe that did not record how (`AppState` version 1).
    Migrated,
}

impl ProofMethod {
    /// Whether the method verifies who the party is.
    pub fn verifies(&self) -> bool {
        !matches!(self, Self::CredentialSignature)
    }
}

/// An identity of a party, and how the wallet verified it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/connections/Proof.ts")]
pub struct Proof {
    #[ts(type = "string")]
    pub identity: Identity,
    pub method: ProofMethod,
}

impl Proof {
    pub fn new(identity: Identity, method: ProofMethod) -> Self {
        Self { identity, method }
    }
}

/// What a connection is found by: the DNS name of the party when a proof binds it to one, otherwise its DID.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export, export_to = "bindings/connections/ConnectionKey.ts")]
#[serde(rename_all = "snake_case")]
pub enum ConnectionKey {
    Domain(String),
    Did(String),
}

impl ConnectionKey {
    /// The key of the party at `connection_url` verified by `proofs`:
    /// - the DNS name of a certificate, for a party verified with one;
    /// - the DID of a party verified by its signed request, or by a version of UniMe that did not record how;
    /// - otherwise the host of `connection_url`: its DID document is served there, or it is an issuer that is not
    ///   verified, whose credentials are bound to it (ADR 0005).
    pub fn of(connection_url: &str, proofs: &[Proof]) -> Option<Self> {
        for proof in proofs {
            match (&proof.method, &proof.identity) {
                (_, Identity::X509 { dns_name }) => return Some(Self::Domain(dns_name.to_ascii_lowercase())),
                (ProofMethod::SignedRequest | ProofMethod::Migrated, Identity::Did(did)) => {
                    return Some(Self::Did(did.clone()))
                }
                _ => {}
            }
        }
        url::Url::parse(connection_url)
            .ok()?
            .host_str()
            .map(|host| Self::Domain(host.to_ascii_lowercase()))
    }
}

/// An empty DID, as the key of a connection read without one.
impl Default for ConnectionKey {
    fn default() -> Self {
        Self::Did(String::new())
    }
}

impl fmt::Display for ConnectionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Domain(domain) => f.write_str(domain),
            Self::Did(did) => f.write_str(did),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn identities_are_strings() {
        let did = Identity::did("did:web:issuer.example").unwrap();
        let x509 = Identity::X509 {
            dns_name: "issuer.example".to_owned(),
        };

        assert_eq!(serde_json::to_value(&did).unwrap(), json!("did:web:issuer.example"));
        assert_eq!(
            serde_json::to_value(&x509).unwrap(),
            json!("x509_san_dns:issuer.example")
        );
        for identity in [did, x509] {
            let value = serde_json::to_value(&identity).unwrap();
            assert_eq!(serde_json::from_value::<Identity>(value).unwrap(), identity);
        }
    }

    #[test]
    fn only_a_did_is_a_did_identity() {
        assert!(Identity::did("issuer.example").is_err());
        assert_eq!(
            Identity::did("did:key:z6MkhbvNd9m3jLBCv7WhPVM2WxMcCEECEbFEMn7Cy8wCkSMx")
                .unwrap()
                .as_did(),
            Some("did:key:z6MkhbvNd9m3jLBCv7WhPVM2WxMcCEECEbFEMn7Cy8wCkSMx")
        );
    }

    fn did(did: &str) -> Identity {
        Identity::did(did).unwrap()
    }

    #[test]
    fn proofs_and_keys_serialize_for_the_frontend() {
        let proof = Proof::new(did("did:web:issuer.example"), ProofMethod::WellKnownDidDocument);
        assert_eq!(
            serde_json::to_value(&proof).unwrap(),
            json!({ "identity": "did:web:issuer.example", "method": "well_known_did_document" })
        );
        assert_eq!(
            serde_json::to_value(ConnectionKey::Domain("issuer.example".to_owned())).unwrap(),
            json!({ "domain": "issuer.example" })
        );
        assert_eq!(
            serde_json::to_value(ConnectionKey::Did("did:key:z6Mk".to_owned())).unwrap(),
            json!({ "did": "did:key:z6Mk" })
        );
    }

    #[test]
    fn a_party_is_keyed_by_its_dns_name_unless_only_its_did_is_verified() {
        let url = "https://Issuer.Example:8443";
        let domain = || Some(ConnectionKey::Domain("issuer.example".to_owned()));

        // Its DID document is served at its DNS name, or it is an issuer that is not verified (yet).
        assert_eq!(ConnectionKey::of(url, &[]), domain());
        assert_eq!(
            ConnectionKey::of(
                url,
                &[Proof::new(did("did:iota:0x42"), ProofMethod::WellKnownDidDocument)]
            ),
            domain()
        );
        assert_eq!(
            ConnectionKey::of(url, &[Proof::new(did("did:jwk:eyJ"), ProofMethod::CredentialSignature)]),
            domain()
        );
        // A certificate certifies the DNS name, whatever the URL.
        assert_eq!(
            ConnectionKey::of(
                "https://other.example",
                &[Proof::new(
                    Identity::X509 {
                        dns_name: "issuer.example".to_owned()
                    },
                    ProofMethod::SignedMetadata
                )]
            ),
            domain()
        );
        // A signed request verifies the DID only.
        for method in [ProofMethod::SignedRequest, ProofMethod::Migrated] {
            assert_eq!(
                ConnectionKey::of(url, &[Proof::new(did("did:key:z6Mk"), method)]),
                Some(ConnectionKey::Did("did:key:z6Mk".to_owned()))
            );
        }
        assert_eq!(ConnectionKey::of("not a url", &[]), None);
    }

    #[test]
    fn only_a_credential_signature_verifies_nobody() {
        assert!(!ProofMethod::CredentialSignature.verifies());
        for method in [
            ProofMethod::WellKnownDidDocument,
            ProofMethod::SignedMetadata,
            ProofMethod::SignedRequest,
            ProofMethod::Migrated,
        ] {
            assert!(method.verifies());
        }
    }
}
