# ADR 0004: Identities Verified with a DID or an X.509 Certificate

## Status

Proposed. Supersedes [ADR 0001](0001-did-based-client-identification.md).

## Context

ADR 0001 identifies every connection by a DID and rejects a party whose identifier cannot be parsed as one. An
OID4VCI credential issuer has to publish a DID document at `/.well-known/did.json`, so that the wallet can show a
verified issuer before the user accepts the connection.

A growing part of the ecosystem verifies parties with X.509 certificates instead:

- The OpenID4VC High Assurance Interoperability Profile (HAIP) 1.0, which the EUDI wallet ecosystem builds on, signs
  SD-JWT VCs, Status List Tokens, signed issuer metadata and OpenID4VP requests with an `x5c` certificate chain. The
  chain is validated against trust anchors from the ecosystem's trust lists.
- SD-JWT VC §3.5 (inline X.509 certificates) makes the subject of the leaf certificate the issuer.
- OID4VCI 1.0 §11.2.3 lets an issuer sign its metadata (`signed_metadata`), and OpenID4VP 1.0 §5.9.3 identifies a
  verifier as `x509_san_dns:<DNS name>` or `x509_hash:<certificate hash>`.

identity-kit validates `x5c` chains (`resolver::X509Resolver`, impierce/identity-kit#172). It binds an issuer claim
only as an `https` URL whose host is a DNS name of the leaf certificate, and refuses a DID `iss` under a certificate
chain: a CA can only be trusted for the DNS names its name constraints cover. A party verified with a certificate
therefore has no DID that UniMe could store, and ADR 0001's connection model cannot represent it.

## Decision

### An identity is verified with a DID or with an X.509 certificate

Every connection has an **identity**: who the party is, as the wallet has verified it. There are two ways to verify it.

- **DID.** The identity is the DID. It is verified as before: by the DID document the DID resolves to, the signatures of
  its verification methods, and domain linkage and linked verifiable presentations where the party publishes them.
- **X.509 certificate.** The identity is the DNS name that a certificate chain certifies: the chain validates against
  one of the wallet's trust anchors, and the leaf certificate has the DNS name as a `dNSName` Subject Alternative
  Name. For an OID4VCI issuer, that DNS name is the host of its `credential_issuer` URL.

The DNS name, not the certificate, is the identity: certificates are renewed and re-keyed while the name stays, and
the name is what a trust anchor's name constraints cover. A connection has one identity, verified one way; the wallet
does not link a DID and a DNS name of the same party.

### Representation

An identity is stored as a string, so existing data stays valid:

- a DID is stored as the DID, unchanged;
- an X.509 identity is stored as `x509_san_dns:<DNS name>`, the client identifier syntax of OpenID4VP 1.0 §5.9.3.

`Connection.did` becomes `Connection.identity`, and the pending connection's `ClientMetadata.client_id` becomes
`ClientMetadata.identity`. `AppState` version 2 renames both fields; a DID stored by version 1 is a valid identity.
Connections are found by their identity.

### Trust anchors

X.509 identities are verified only against trust anchors the wallet holds, in `AppState::trust_anchors` (CA
certificates, PEM). UniMe ships none yet, so until anchors are added the wallet behaves as under ADR 0001. Where the
anchors of a release come from is open: an ecosystem's trusted list (for the EUDI wallet ecosystem, the trusted lists
of ETSI TS 119 612), a list Impierce curates, or anchors the user adds. Revocation (CRL, OCSP) is not checked;
identity-kit leaves it out of `X509Resolver`, and keeping the anchors current is the mitigation.

### OID4VCI credential issuers

The identity is verified before the user accepts the connection, as ADR 0001 requires:

1. A DID document at `/.well-known/did.json`, found as ADR 0001 describes, gives a DID identity. This comes first,
   so the identity of an existing connection does not change.
2. Otherwise, the credential issuer metadata's `signed_metadata` gives an X.509 identity when its `x5c` chain
   validates against a trust anchor, its signature verifies with the leaf's key, its `sub` is the
   `credential_issuer` URL, and the leaf certificate has that URL's host as a DNS name.
3. Otherwise the issuer is rejected, as before.

The connection's identity decides how its credentials are checked. Every credential of an issuer with an X.509
identity, whatever its format, must be signed under an `x5c` chain that validates against a trust anchor and whose
leaf has the identity's DNS name. The credentials of an issuer with a DID identity are validated as before.

### Verifiers

A verifier identified as `x509_san_dns:` or `x509_hash:` needs its signed request verified with the key of its `x5c`
leaf, and the leaf matched against the client identifier. openid4vc verifies requests only with keys it resolves from
a DID (`oid4vc_core::Verify`, impierce/openid4vc#210). Until it can, verifiers keep DID identities, and a client
identifier that is not a DID is rejected as before.

### Domain linkage and linked verifiable presentations

Both stay DID-only. An X.509 identity's domain is already bound: it is the DNS name the certificate chain certifies.

## Consequences

- Issuers that follow HAIP can be accepted without publishing a DID document, and their identity is still verified
  before the user accepts the connection.
- The wallet decides which certificate authorities may vouch for an identity. Shipping and updating trust anchors
  becomes part of a release, and an anchor that is compromised or too broad vouches for identities it should not.
- `AppState` version 2 renames `did` to `identity`. Older versions of UniMe cannot read a state written by version 2.
- The frontend shows an identity instead of a DID, and recognizes the `x509_san_dns:` form.
- X.509 verifiers wait for openid4vc to verify requests under `x5c`.
- UniMe depends on identity-kit 0.5.1, the first release with `x5c` support (impierce/identity-kit#172).
