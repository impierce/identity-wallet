//! The identity of a party the wallet connects to, see `docs/adr/0004-identities-proven-by-did-or-x509.md`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// The prefix of an identity proven by an X.509 certificate: the client identifier prefix of OpenID4VP 1.0 §5.9.3 for
/// a party named by a `dNSName` of its certificate.
const X509_SAN_DNS: &str = "x509_san_dns:";

/// Who a party is, as the wallet has proven it: by a DID, or by an X.509 certificate chain that validates against one
/// of the wallet's trust anchors and whose leaf certificate has the DNS name.
///
/// Serialized as a string: a DID as the DID, an X.509 identity as `x509_san_dns:<DNS name>`. Connections stored before
/// X.509 identities existed hold a DID string, which reads as [`Identity::Did`].
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Identity {
    /// A DID, proven by its DID document.
    Did(String),
    /// A DNS name, proven by an X.509 certificate chain.
    X509 { dns_name: String },
}

impl Identity {
    /// The identity of `did`, checking that it is a DID.
    pub fn did(did: &str) -> Result<Self, identity_kit::did::DidError> {
        Ok(Self::Did(identity_kit::did::Did::parse(did)?.to_string()))
    }

    /// The DID, for an identity proven by one.
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
    /// Reads a stored identity. A DID is not checked again: it was checked when the identity was proven.
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
}
