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

1. **Before acceptance.** When the issuer publishes no DID document, and is not verified with an X.509 certificate
   either ([ADR 0004](0004-identities-verified-with-did-or-x509.md)), it has no identity yet
   (`ClientMetadata.identity` is `None`). It publishes none when every candidate URL of ADR 0001 answers 404 or 410, or a success whose body is not
   JSON, such as a site's page served for any path. The prompt shows the credential issuer URL and that the issuer's
   identity is not verified. Domain linkage and linked verifiable presentations are checked on a DID, so they are
   skipped. Any other answer is refused, since the document may exist but be unavailable: a request that fails
   outright, another HTTP error status such as a 503, JSON that is no DID document, or a DID document whose `id` is
   not a DID. An outage must not cost an issuer its verification. An issuer whose signed metadata verifies it with a
   certificate is accepted with that identity, whatever its DID document answers.
2. **On receipt.** Every received credential's issuer signature is verified before anything is stored: `jwt_vc_json`,
   `dc+sd-jwt` and `vc+sd-jwt`. The key is the one the JWT's `kid` names: a DID URL resolved with the wallet's
   resolver, to a method listed under `verificationMethod`, as for every other signature UniMe verifies. When the JWT
   has an `iss`, it must be the DID of that key. For an SD-JWT, every disclosure must match a digest of the signed
   payload. The DID of the key is the **signing DID**.
3. **Binding.** An issuer verified with its DID document must have signed every credential with that DID; otherwise the
   credentials are refused. An unverified issuer is bound to the signing DID, which all its credentials must share. The
   connection is created, or found, under that DID only after the credentials verified. A credential that fails leaves
   neither a credential nor a connection behind.
4. **No presentation before binding.** UniMe presents no credentials to an unverified issuer. An issuer that asks for
   a presentation in an Interactive Authorization is refused: the user would share credentials with a party whose
   identity is unknown, and the share could be recorded on no connection if the issuance then failed.

An issuer that publishes a DID document is verified with it before the user accepts, as ADR 0001 decided. One
verified with an X.509 certificate counts as verified too: its credentials are checked against its certificate chain
(ADR 0004), and it is bound to no DID.

## Consequences

`did:jwk` issuers, and other issuers without a DID document, can issue to UniMe, as DIIP v5 requires.

For these issuers the user accepts before the issuer's identity is known. The prompt says so. The DID becomes known when
the credential's signature verifies, and the connection is stored under it, so a later offer from the same issuer finds
the same connection.

A stored connection always has an identity (a DID, or an X.509 identity under ADR 0004). Only the prompt and the
pending connection data can be without one. A new connection's id is the hash of its name, URL and identity, as a JSON
array, so an issuer that signs with a new DID gets a connection of its own; stored connections keep their ids.

An unverified issuer cannot ask for a presentation in an Interactive Authorization. Interactive Authorization is not
part of DIIP v5.

Credentials whose issuer signature does not verify, whose `kid` is not a DID URL, or whose `iss` names another DID than
the signing key's are now refused, in every supported format. Before, only `jwt_vc_json` credentials were verified.

A did:web issuer whose credentials are signed by another DID than its DID document's `id` is now refused.
