//! Reading a certificate offered as a trust anchor.

use super::TrustAnchorRefusal;
use anyhow::{bail, Context as _};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use identity_kit::resolver::{X509Certificate, X509Error, X509Resolver};
use rustls::pki_types::{pem::PemObject as _, CertificateDer};
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use x509_cert::{
    der::Decode as _,
    ext::pkix::{name::GeneralName, NameConstraints},
    Certificate,
};

/// What the user sees of a trust anchor's certificate.
#[derive(Serialize, Deserialize, Clone, Debug, TS, PartialEq)]
#[ts(export, export_to = "bindings/trust_anchors/CertificateDetails.ts")]
pub struct CertificateDetails {
    /// The subject's distinguished name (RFC 4514).
    pub subject: String,
    /// The issuer's distinguished name (RFC 4514).
    pub issuer: String,
    /// Whether subject and issuer are the same: a root CA, as a rule.
    pub self_issued: bool,
    /// Whether `basicConstraints` makes the subject a CA.
    pub ca: bool,
    /// The validity period, in seconds since the Unix epoch.
    #[ts(type = "number")]
    pub not_before: i64,
    #[ts(type = "number")]
    pub not_after: i64,
    /// The SHA-256 hash of the certificate, base64url-encoded (`x5t#S256`): the id of the trust anchor.
    pub sha256_thumbprint: String,
    /// The same hash as colon-separated hexadecimal bytes, the way certificate viewers show it.
    pub sha256_fingerprint: String,
    /// The DNS names the name constraints permit, and those they exclude. A CA without name constraints may certify
    /// any name.
    pub permitted_dns_names: Vec<String>,
    pub excluded_dns_names: Vec<String>,
}

/// The largest certificate read, in bytes. Root certificates are a few kilobytes.
pub const MAX_CERTIFICATE_SIZE: usize = 64 * 1024;

/// Reads `bytes`, one certificate in PEM or DER, and returns it in PEM with its details.
pub fn read(bytes: &[u8]) -> anyhow::Result<(String, CertificateDetails)> {
    if bytes.len() > MAX_CERTIFICATE_SIZE {
        bail!("More than {MAX_CERTIFICATE_SIZE} bytes, too large for a certificate");
    }
    let der = if bytes.trim_ascii_start().starts_with(b"-----BEGIN") {
        let mut certificates = CertificateDer::pem_slice_iter(bytes);
        let certificate = certificates.next().context("The PEM holds no certificate")??;
        if certificates.next().is_some() {
            bail!("The PEM holds more than one certificate");
        }
        certificate.to_vec()
    } else {
        bytes.to_vec()
    };

    let certificate = X509Certificate::from_der(&der).context("Not a certificate")?;
    let name_constraints = Certificate::from_der(&der)
        .context("Not a certificate")?
        .tbs_certificate()
        .get_extension::<NameConstraints>()
        .context("Invalid name constraints")?
        .map(|(_, constraints)| constraints);
    let dns_names = |subtrees: Option<Vec<x509_cert::ext::pkix::constraints::name::GeneralSubtree>>| {
        subtrees
            .unwrap_or_default()
            .into_iter()
            .filter_map(|subtree| match subtree.base {
                GeneralName::DnsName(name) => Some(name.to_string()),
                _ => None,
            })
            .collect()
    };
    let (permitted, excluded) = name_constraints
        .map(|constraints| (constraints.permitted_subtrees, constraints.excluded_subtrees))
        .unwrap_or_default();

    let details = CertificateDetails {
        subject: certificate.subject().to_string(),
        issuer: certificate.issuer().to_string(),
        self_issued: certificate.is_self_issued(),
        ca: certificate.is_ca(),
        not_before: certificate.not_before(),
        not_after: certificate.not_after(),
        sha256_thumbprint: certificate.sha256_thumbprint(),
        sha256_fingerprint: fingerprint(&der),
        permitted_dns_names: dns_names(permitted),
        excluded_dns_names: dns_names(excluded),
    };
    Ok((pem(&der), details))
}

/// The name of the certificate's subject: its common name, else its organization, else the whole name.
pub fn subject_name(der_or_pem: &[u8]) -> Option<String> {
    let der = if der_or_pem.trim_ascii_start().starts_with(b"-----BEGIN") {
        CertificateDer::from_pem_slice(der_or_pem).ok()?.to_vec()
    } else {
        der_or_pem.to_vec()
    };
    let certificate = Certificate::from_der(&der).ok()?;
    let subject = certificate.tbs_certificate().subject();
    let name = subject
        .common_name()
        .ok()
        .flatten()
        .or_else(|| subject.organization().ok().flatten())
        .map(String::from)
        .unwrap_or_else(|| subject.to_string());
    Some(name)
}

/// Why `pem`, read by [`read`], cannot be a trust anchor at `now` (seconds since the Unix epoch), if it cannot.
pub fn refusal(pem: &str, details: &CertificateDetails, now: i64) -> anyhow::Result<Option<TrustAnchorRefusal>> {
    match X509Resolver::from_pem(pem) {
        Err(X509Error::TrustAnchorNotCa { .. }) => return Ok(Some(TrustAnchorRefusal::NotCa)),
        Err(error) => return Err(error).context("Not a trust anchor"),
        Ok(_) => {}
    }
    Ok(if now < details.not_before {
        Some(TrustAnchorRefusal::NotYetValid)
    } else if now > details.not_after {
        Some(TrustAnchorRefusal::Expired)
    } else {
        None
    })
}

fn pem(der: &[u8]) -> String {
    let base64 = STANDARD.encode(der);
    let lines: Vec<&str> = base64
        .as_bytes()
        .chunks(64)
        .map(|line| std::str::from_utf8(line).expect("base64 is ASCII"))
        .collect();
    format!(
        "-----BEGIN CERTIFICATE-----\n{}\n-----END CERTIFICATE-----\n",
        lines.join("\n")
    )
}

fn fingerprint(der: &[u8]) -> String {
    let hex = sha256::digest(der).to_uppercase();
    hex.as_bytes()
        .chunks(2)
        .map(|byte| std::str::from_utf8(byte).expect("hexadecimal is ASCII"))
        .collect::<Vec<_>>()
        .join(":")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::trust_anchors::tests::root_der;

    #[test]
    fn reads_a_certificate_in_der_and_in_pem_alike() {
        let (pem, details) = read(&root_der()).unwrap();

        assert!(pem.starts_with("-----BEGIN CERTIFICATE-----\n"));
        assert_eq!(read(pem.as_bytes()).unwrap(), (pem.clone(), details.clone()));
        assert!(details.ca);
        assert!(details.self_issued);
        assert_eq!(details.subject, details.issuer);
        assert_eq!(details.sha256_fingerprint.len(), 32 * 3 - 1);
        assert_eq!(refusal(&pem, &details, details.not_before + 1).unwrap(), None);
    }

    #[test]
    fn refuses_a_certificate_outside_its_validity_period() {
        let (pem, details) = read(&root_der()).unwrap();

        assert_eq!(
            refusal(&pem, &details, details.not_after + 1).unwrap(),
            Some(TrustAnchorRefusal::Expired)
        );
        assert_eq!(
            refusal(&pem, &details, details.not_before - 1).unwrap(),
            Some(TrustAnchorRefusal::NotYetValid)
        );
    }

    #[test]
    fn refuses_a_certificate_that_is_no_ca() {
        let signer: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/fixtures/x509-limbo/signer.json")).unwrap();
        let leaf = STANDARD.decode(signer["x5c"][0].as_str().unwrap()).unwrap();
        let (pem, details) = read(&leaf).unwrap();

        assert!(!details.ca);
        assert_eq!(
            refusal(&pem, &details, details.not_before + 1).unwrap(),
            Some(TrustAnchorRefusal::NotCa)
        );
    }

    #[test]
    fn does_not_read_what_is_no_single_certificate() {
        let (pem, _) = read(&root_der()).unwrap();

        assert!(read(b"<html>Not found</html>").is_err());
        assert!(read(format!("{pem}{pem}").as_bytes()).is_err());
        assert!(read(b"-----BEGIN PRIVATE KEY-----\nAAAA\n-----END PRIVATE KEY-----\n").is_err());
        assert!(read(&vec![0; MAX_CERTIFICATE_SIZE + 1]).is_err());
    }

    #[test]
    fn names_a_certificate_by_its_subject() {
        assert!(subject_name(&root_der()).is_some_and(|name| !name.is_empty()));
    }
}
