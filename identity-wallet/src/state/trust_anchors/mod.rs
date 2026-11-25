//! The trust anchors that X.509 identities are verified against (ADR 0004): the anchors Impierce curates, which come
//! with the app, and the anchors the user added.

pub mod actions;
pub mod certificate;
pub mod reducers;

use certificate::CertificateDetails;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The trust anchors of the wallet, and a certificate the user is asked to accept as one.
#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq, Default)]
#[ts(export, export_to = "bindings/trust_anchors/TrustAnchors.ts")]
#[serde(default)]
pub struct TrustAnchors {
    pub anchors: Vec<TrustAnchor>,
    /// A certificate fetched from a URL, shown for the user to accept or deny.
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<PendingTrustAnchor>,
}

/// A CA certificate that X.509 identities are verified against while it is enabled.
#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_anchors/TrustAnchor.ts")]
pub struct TrustAnchor {
    /// The SHA-256 hash of the certificate, base64url-encoded (`x5t#S256`).
    pub id: String,
    pub name: String,
    pub source: TrustAnchorSource,
    pub enabled: bool,
    /// The certificate, in PEM.
    pub certificate: String,
    pub details: CertificateDetails,
}

#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_anchors/TrustAnchorSource.ts")]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TrustAnchorSource {
    /// Curated by Impierce and shipped with the app: it can be enabled and disabled, not changed or removed.
    Curated {
        /// Whether the anchor is enabled until the user decides otherwise.
        enabled_by_default: bool,
    },
    /// Added by the user, from the certificate at `url` (unknown for an anchor of `AppState` version 2).
    Added {
        #[ts(optional)]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
}

/// A certificate fetched from `url` for the user to accept as a trust anchor, or the reason it cannot be one.
#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_anchors/PendingTrustAnchor.ts")]
pub struct PendingTrustAnchor {
    pub anchor: TrustAnchor,
    #[ts(optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<TrustAnchorRefusal>,
}

/// Why a certificate cannot be accepted as a trust anchor.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, TS, PartialEq, Eq)]
#[ts(export, export_to = "bindings/trust_anchors/TrustAnchorRefusal.ts")]
#[serde(rename_all = "snake_case")]
pub enum TrustAnchorRefusal {
    /// Not a CA certificate: its key would sign credentials, not certificates.
    NotCa,
    /// Outside its validity period, so no chain validates against it.
    Expired,
    NotYetValid,
    /// Already one of the trust anchors.
    AlreadyTrusted,
}

/// An anchor of the curated list, as `resources/curated_trust_anchors.json` holds it.
#[derive(Deserialize)]
struct CuratedTrustAnchor {
    name: String,
    enabled_by_default: bool,
    certificate: String,
}

impl TrustAnchors {
    /// The certificates, in PEM, of the enabled trust anchors.
    pub fn enabled(&self) -> Vec<String> {
        self.anchors
            .iter()
            .filter(|anchor| anchor.enabled)
            .map(|anchor| anchor.certificate.clone())
            .collect()
    }

    /// The trust anchors with the curated ones of this version of the app: a curated anchor no longer curated is
    /// removed, a new one gets its default. One the user enabled or disabled against its default keeps that choice;
    /// otherwise it follows the default of this version. The anchors the user added stay as they are, and so does a
    /// review in progress.
    pub fn with_curated(self) -> Self {
        self.with(curated())
    }

    fn with(self, curated: Vec<TrustAnchor>) -> Self {
        let TrustAnchors { anchors, pending } = self;
        let (previous, added): (Vec<_>, Vec<_>) = anchors
            .into_iter()
            .partition(|anchor| matches!(anchor.source, TrustAnchorSource::Curated { .. }));

        let curated = curated.into_iter().map(|mut anchor| {
            let previous_choice = previous
                .iter()
                .find(|previous| previous.id == anchor.id)
                .and_then(|previous| {
                    let TrustAnchorSource::Curated { enabled_by_default } = previous.source else {
                        return None;
                    };
                    (previous.enabled != enabled_by_default).then_some(previous.enabled)
                });
            if let Some(enabled) = previous_choice {
                anchor.enabled = enabled;
            }
            anchor
        });

        let mut anchors: Vec<_> = curated.collect();
        let added: Vec<_> = added
            .into_iter()
            .filter(|anchor| anchors.iter().all(|curated| curated.id != anchor.id))
            .collect();
        anchors.extend(added);
        TrustAnchors { anchors, pending }
    }

    pub fn find_mut(&mut self, id: &str) -> Option<&mut TrustAnchor> {
        self.anchors.iter_mut().find(|anchor| anchor.id == id)
    }
}

/// The curated trust anchors of this version of the app.
fn curated() -> Vec<TrustAnchor> {
    let curated: Vec<CuratedTrustAnchor> =
        serde_json::from_str(include_str!("../../../resources/curated_trust_anchors.json"))
            .expect("`curated_trust_anchors.json` holds the curated trust anchors");
    curated
        .into_iter()
        .map(
            |CuratedTrustAnchor {
                 name,
                 enabled_by_default,
                 certificate,
             }| {
                let (certificate, details) = certificate::read(certificate.as_bytes())
                    .unwrap_or_else(|error| panic!("the curated trust anchor {name} is a certificate: {error:#}"));
                TrustAnchor {
                    id: details.sha256_thumbprint.clone(),
                    name,
                    source: TrustAnchorSource::Curated { enabled_by_default },
                    enabled: enabled_by_default,
                    certificate,
                    details,
                }
            },
        )
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use base64::{engine::general_purpose::STANDARD, Engine as _};

    /// The root CA, DER-encoded, of the x509-limbo chain the `x509` tests use.
    pub fn root_der() -> Vec<u8> {
        let signer: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/fixtures/x509-limbo/signer.json")).unwrap();
        STANDARD.decode(signer["trust_anchor"].as_str().unwrap()).unwrap()
    }

    pub fn anchor(source: TrustAnchorSource, enabled: bool) -> TrustAnchor {
        let (certificate, details) = certificate::read(&root_der()).unwrap();
        TrustAnchor {
            id: details.sha256_thumbprint.clone(),
            name: "x509-limbo root".to_string(),
            source,
            enabled,
            certificate,
            details,
        }
    }

    fn curated_anchor(id: &str, enabled_by_default: bool) -> TrustAnchor {
        TrustAnchor {
            id: id.to_string(),
            ..anchor(TrustAnchorSource::Curated { enabled_by_default }, enabled_by_default)
        }
    }

    fn added_anchor(id: &str) -> TrustAnchor {
        TrustAnchor {
            id: id.to_string(),
            ..anchor(
                TrustAnchorSource::Added {
                    url: Some("https://ca.example/root.pem".to_string()),
                },
                true,
            )
        }
    }

    fn ids_and_enabled(trust_anchors: &TrustAnchors) -> Vec<(&str, bool)> {
        trust_anchors
            .anchors
            .iter()
            .map(|anchor| (anchor.id.as_str(), anchor.enabled))
            .collect()
    }

    #[test]
    fn the_curated_trust_anchors_are_certificates() {
        let curated = curated();
        for anchor in &curated {
            assert!(anchor.details.ca, "{} is a CA certificate", anchor.name);
        }
        let mut ids: Vec<_> = curated.iter().map(|anchor| &anchor.id).collect();
        ids.dedup();
        assert_eq!(ids.len(), curated.len());
    }

    #[test]
    fn the_curated_authbound_issuer_ca_validates_the_chain_of_the_authbound_test_bed_issuer() {
        use identity_kit::resolver::X509Resolver;

        let fixture: serde_json::Value = serde_json::from_str(include_str!(
            "../../../tests/fixtures/authbound-testbed-issuer-x5c.json"
        ))
        .unwrap();
        let chain: Vec<Vec<u8>> = fixture["x5c"]
            .as_array()
            .unwrap()
            .iter()
            .map(|certificate| STANDARD.decode(certificate.as_str().unwrap()).unwrap())
            .collect();
        let authbound = curated()
            .into_iter()
            .find(|anchor| anchor.name == "Authbound EUDI Testbed Issuer Issuing CA")
            .unwrap();
        // 2026-11-24, when the leaf is valid (2026-09-08 to 2027-09-08).
        let now = 1_795_478_400;

        let resolver = X509Resolver::from_pem(&authbound.certificate).unwrap();
        assert!(resolver.validate(&chain, now).is_ok());

        let others = TrustAnchors::default().with(vec![anchor(
            TrustAnchorSource::Curated {
                enabled_by_default: true,
            },
            true,
        )]);
        let resolver = X509Resolver::from_pem(&others.enabled().join("\n")).unwrap();
        assert!(resolver.validate(&chain, now).is_err());
    }

    #[test]
    fn a_new_wallet_gets_the_curated_trust_anchors_with_their_defaults() {
        let trust_anchors = TrustAnchors::default().with(vec![curated_anchor("a", true), curated_anchor("b", false)]);

        assert_eq!(ids_and_enabled(&trust_anchors), vec![("a", true), ("b", false)]);
    }

    #[test]
    fn a_choice_against_the_default_is_kept_and_an_untouched_anchor_follows_a_new_default() {
        let mut stored =
            TrustAnchors::default().with(vec![curated_anchor("chosen", true), curated_anchor("untouched", false)]);
        stored.find_mut("chosen").unwrap().enabled = false;

        let trust_anchors = stored.with(vec![curated_anchor("chosen", true), curated_anchor("untouched", true)]);

        assert_eq!(
            ids_and_enabled(&trust_anchors),
            vec![("chosen", false), ("untouched", true)]
        );
    }

    #[test]
    fn an_anchor_no_longer_curated_is_removed_and_added_anchors_stay() {
        let mut stored = TrustAnchors::default().with(vec![curated_anchor("dropped", true)]);
        stored.anchors.push(added_anchor("mine"));

        let trust_anchors = stored.with(vec![curated_anchor("new", false)]);

        assert_eq!(ids_and_enabled(&trust_anchors), vec![("new", false), ("mine", true)]);
    }

    #[test]
    fn an_added_anchor_that_becomes_curated_is_curated() {
        let mut stored = TrustAnchors::default();
        stored.anchors.push(added_anchor("both"));

        let trust_anchors = stored.with(vec![curated_anchor("both", false)]);

        assert_eq!(ids_and_enabled(&trust_anchors), vec![("both", false)]);
        assert!(matches!(
            trust_anchors.anchors[0].source,
            TrustAnchorSource::Curated { .. }
        ));
    }

    #[test]
    fn only_enabled_anchors_are_used() {
        let mut trust_anchors =
            TrustAnchors::default().with(vec![curated_anchor("on", true), curated_anchor("off", false)]);
        trust_anchors.anchors.push(added_anchor("mine"));
        trust_anchors.find_mut("mine").unwrap().enabled = false;

        assert_eq!(trust_anchors.enabled().len(), 1);
    }
}
