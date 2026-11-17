# ADR 0005: Unverified OID4VCI Issuers, Bound to the DID That Signs Their Credential

## Status

Accepted. Amends [ADR 0001](0001-did-based-client-identification.md): `/.well-known/did.json` is no longer mandatory
for an OID4VCI credential issuer.

## Context

ADR 0001 identifies every connection by a DID. An OID4VCI credential offer carries only the credential issuer URL, so
UniMe verifies the issuer's DID with `/.well-known/did.json` before the user accepts. An issuer that publishes no
DID document is rejected. In practice this limits issuers to `did:web`.

The Decentralized Identity Interop Profile (DIIP) v5 requires more:

> DIIP-compliant implementations MUST support did:jwk and did:web as the identifiers of the Issuers, Holders, and
> Verifiers.

A `did:jwk` issuer has no DID document to publish, so UniMe refuses its offers. A DIIP v5 conformance run against a
`did:jwk` issuer showed this: no issuance completed, and every check that needs a credential in the wallet stayed
undecided.

UniMe also verified the issuer signature only of `jwt_vc_json` credentials. A `dc+sd-jwt` or `vc+sd-jwt` credential was
stored without a check, and nothing tied it to the connection's DID.

## Decision

An OID4VCI issuer that publishes no DID document is accepted as **not verified**. Its DID is taken from the credential
it issues.

1. **Before acceptance.** When no candidate URL of ADR 0001 serves a DID document, the issuer has no DID yet
   (`ClientMetadata.client_id` is `None`). The prompt shows the credential issuer URL and that the issuer's identity is
   not verified. Domain linkage and linked verifiable presentations are checked on a DID, so they are skipped. A request
   that fails outright, or a DID document whose `id` is not a DID, is still refused.
2. **On receipt.** Every received credential's issuer signature is verified before anything is stored: `jwt_vc_json`,
   `dc+sd-jwt` and `vc+sd-jwt`. The key is the one the JWT's `kid` names: a DID URL resolved with the wallet's
   resolver, to a method listed under `verificationMethod`, as for every other signature UniMe verifies. When the JWT
   has an `iss`, it must be the DID of that key. For an SD-JWT, every disclosure must match a digest of the signed
   payload. The DID of the key is the **signing DID**.
3. **Binding.** An issuer verified with its DID document must have signed every credential with that DID; otherwise the
   credentials are refused. An unverified issuer is bound to the signing DID, which all its credentials must share. The
   connection is created, or found, under that DID only after the credentials verified. A credential that fails leaves
   neither a credential nor a connection behind.

An issuer that publishes a DID document is verified with it before the user accepts, as ADR 0001 decided.

## Consequences

`did:jwk` issuers, and other issuers without a DID document, can issue to UniMe, as DIIP v5 requires.

For these issuers the user accepts before the issuer's identity is known. The prompt says so. The DID becomes known when
the credential's signature verifies, and the connection is stored under it, so a later offer from the same issuer finds
the same connection.

A stored connection always has a DID, so the persisted state does not change. A new connection's id is derived from
its name, URL and DID, so an issuer that signs with a new DID gets a connection of its own; stored connections keep
their ids. Only the prompt and the pending connection
data can be without one.

Credentials whose issuer signature does not verify, whose `kid` is not a DID URL, or whose `iss` names another DID than
the signing key's are now refused, in every supported format. Before, only `jwt_vc_json` credentials were verified.

A did:web issuer whose credentials are signed by another DID than its DID document's `id` is now refused.
