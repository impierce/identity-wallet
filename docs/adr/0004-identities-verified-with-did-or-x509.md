# ADR 0004: Connections Verified with DIDs and X.509 Certificates

## Status

Proposed. Supersedes [ADR 0001](0001-did-based-client-identification.md). Amends
[ADR 0005](0005-unverified-oid4vci-issuers.md) where noted.

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

A party can also show several pieces of evidence: a DID document at its domain, signed metadata under a certificate,
domain linkage, credentials signed by a DID. Making one of them "the identity" of a connection forces the wallet to pick
one, and loses the connection when the party moves from one to another. The user thinks of the party, not of its
identifiers.

## Decision

### A connection is a party, found by a key and verified by proofs

A **connection** is the party the user interacts with. It has:

- a **key**, what the connection is found by: the DNS name of the party when a proof binds it to one, otherwise its
  DID;
- **proofs**: each an identity of the party, and how the wallet verified it.

An **identity** is a DID, or the DNS name a certificate chain certifies, written `x509_san_dns:<DNS name>` (the client
identifier syntax of OpenID4VP 1.0 §5.9.3). For a certificate, the DNS name and not the certificate is the identity:
certificates are renewed and re-keyed while the name stays, and the name is what a trust anchor's name constraints
cover.

| Method                    | Identity | Verified by                                                                     | Key      |
| ------------------------- | -------- | ------------------------------------------------------------------------------- | -------- |
| `well_known_did_document` | DID      | its DID document, served at `/.well-known/did.json` of the party's DNS name     | DNS name |
| `signed_metadata`         | DNS name | `signed_metadata` under an `x5c` chain that validates against a trust anchor    | DNS name |
| `signed_request`          | DID      | a SIOPv2 or OpenID4VP request signed with a key of the DID                      | DID      |
| `credential_signature`    | DID      | nothing: the DID that signs the credentials of an issuer not verified otherwise | DNS name |
| `migrated`                | DID      | a version of UniMe that did not record how (`AppState` version 1)               | DID      |

A proof joins a connection only when it binds its identity to the connection's key, or is found under the key. The
wallet never links two identities because they merely occur together. A `credential_signature` verifies nobody: it
records which DID an unverified issuer signs with, so that its later credentials are held to it (ADR 0005).

A connection is found by its key, or else by an identity it shares with the party, so that a connection stored under
its DID is found by a party now keyed by its DNS name. Each interaction replaces the connection's key and proofs with the
ones just verified; the connection keeps its id.

### Trust anchors

X.509 identities are verified only against the enabled trust anchors the wallet holds, in `AppState::trust_anchors`
(CA certificates). Without an enabled anchor the wallet behaves as under ADR 0001 and ADR 0005: an issuer verified with
neither a DID document nor a certificate is accepted as unverified. Revocation (CRL, OCSP) is not checked; identity-kit
leaves it out of `X509Resolver`, and keeping the anchors current is the mitigation.

The anchors come from two sources:

- **Curated by Impierce:** `identity-wallet/resources/curated_trust_anchors.json`, shipped with each release, for now
  the CAs of the test beds UniMe tests its interoperability with. Each entry says whether it is enabled by default, so
  testers need no developer mode for the anchors Impierce trusts. The user can enable and disable a curated anchor, not
  change or remove it. When the state is loaded, the curated anchors are those of the running release: one no longer
  curated is removed, and one the user left at its default follows the default of the release.
- **Added by the user,** in developer mode: UniMe fetches a CA certificate (PEM or DER) from a URL the user enters and
  shows it, and the user accepts or denies it. A certificate that is no CA certificate, is outside its validity period
  or is already an anchor cannot be accepted. The user can disable and remove an anchor they added.

An ecosystem's trusted list (for the EUDI wallet ecosystem, the trusted lists of ETSI TS 119 612) is not a source yet.

The trust anchors of the public Web PKI are not trust anchors for identities: a TLS certificate only certifies control
of a domain, which fetching the issuer's metadata over HTTPS already shows, and its CAs are not chosen by the wallet.

### OID4VCI credential issuers

The issuer is verified, where it can be, before the user accepts the connection, as ADR 0001 requires; ADR 0005 covers
the one that cannot be:

1. A DID document at `/.well-known/did.json`, found as ADR 0001 describes, gives a `well_known_did_document` proof.
   This comes first, so the proof of an existing connection does not change.
2. Otherwise, the credential issuer metadata's `signed_metadata` gives a `signed_metadata` proof when its `x5c` chain
   validates against a trust anchor, its signature verifies with the leaf's key, its `sub` is the
   `credential_issuer` URL, and the leaf certificate has that URL's host as a DNS name.
   When the signed metadata verifies, its values take precedence over the unsigned metadata, as OID4VCI 1.0
   §11.2.3 requires: UniMe uses its endpoints and credential configurations, whatever proof it ends up with.
   Signed metadata that verifies but gives no valid metadata is an error, not replaced by the unsigned values. An
   issuer with a `signed_metadata` proof gets no presentation in an Interactive Authorization unless the metadata
   loaded for the offer is signed, since its authorization server is taken from that metadata.
3. Otherwise an issuer that publishes no DID document is accepted as unverified, and bound to the DID that signs its
   credential with a `credential_signature` proof ([ADR 0005](0005-unverified-oid4vci-issuers.md)). One whose DID
   document is unavailable is rejected.

The issuer's connection is keyed by its DNS name in each case.

The connection's proof decides how its credentials are checked. Every credential of an issuer verified with
`signed_metadata`, whatever its format, must be signed under an `x5c` chain that validates against a trust anchor and
whose leaf has the identity's DNS name. The credentials of an issuer verified with its DID document, or of an
unverified one, are verified with the DID that signs them, as ADR 0005 describes.

An unverified issuer is held to the DIDs of the connection found under its DNS name: a credential signed with another
DID is refused (amends ADR 0005). The party at that name signs with another key than the one the user connected to
before.

### Verifiers

A verifier identified as `x509_san_dns:` or `x509_hash:` needs its signed request verified with the key of its `x5c`
leaf, and the leaf matched against the client identifier. openid4vc verifies requests only with keys it resolves from
a DID (`oid4vc_core::Verify`, impierce/openid4vc#210). Until it can, a verifier is verified by a `signed_request` proof
of its DID, keyed by that DID, and a client identifier that is not a DID is rejected as before.

### Domain linkage and linked verifiable presentations

Both stay DID-only, and are kept on the connection beside its proofs. A DNS name certified by a certificate is already
bound to its domain.

### Showing a connection

The name and logo of a party come from its own metadata, unless signed metadata vouches for them, so they are claims.
The UI keeps what a party claims apart from what the wallet verified:

- The title is the party's name. Without one, it is the connection's DNS name, or else its DID shortened.
- Without a logo, the avatar is a monogram of the title.
- The connection's key is always shown next to the name, also when the name is the party's own claim.
- The verification shown applies to the key, never to the name.
- The connection's details list its proofs.

## Consequences

- Issuers that follow HAIP can be accepted without publishing a DID document, and their identity is still verified
  before the user accepts the connection.
- An issuer keeps its connection when it moves from a DID document to a certificate for the same DNS name, or the other
  way around.
- The wallet decides which certificate authorities may vouch for an identity. Shipping and updating trust anchors
  becomes part of a release, and an anchor that is compromised or too broad vouches for identities it should not.
- `AppState` version 2 replaces a connection's `did` with `key` and `proofs`, and the pending connection's
  `ClientMetadata.client_id` with `proofs`. A version 1 DID becomes a `migrated` proof, keyed by the DID until the
  connection is verified again. Older versions of UniMe cannot read a state written by version 2.
- An unverified issuer that signs with a new DID under the same DNS name is refused, where ADR 0005 gave it a
  connection of its own. A version 1 state with two such connections keeps both; the second is refused at its next
  issuance.
- X.509 verifiers wait for openid4vc to verify requests under `x5c`.
- UniMe depends on identity-kit 0.5.1, the first release with `x5c` support (impierce/identity-kit#172).
- Follow-ups: collect every proof an issuer offers rather than the first, and accept its credentials under any of them;
  record the trust anchor that vouched for a `signed_metadata` proof; show in the prompt when a known connection's
  proofs changed; derive a verification level (ecosystem, domain, not verified) for the UI; verify Status List Tokens
  signed under `x5c`.
