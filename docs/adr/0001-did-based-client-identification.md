# ADR 0001: DID-Based Client Identification and OID4VCI Issuer DID Discovery

## Status

Accepted

## Context

UniMe needs a verifiable identifier for every party shown on the connection acceptance screen, so it can validate domain linkage, discover linked verifiable presentations, and store connections against a stable identity.

SIOPv2 and OID4VP authorization requests already carry a client identifier that can be parsed as a DID after removing any OpenID4VP client identifier prefix.

OID4VCI credential offers do not. They only provide a credential issuer URL, so the wallet has no issuer DID before the user is asked to accept the connection.

## Decision

All connections are identified by a DID. A client identifier that cannot be parsed as a DID is rejected.

For OID4VCI, the issuer DID is discovered by fetching:

```text
{credential_issuer_url}/.well-known/did.json
```

The DID document's `id` becomes the issuer DID in `ClientMetadata`. This makes `/.well-known/did.json` mandatory for every OID4VCI credential issuer accepted by UniMe.

We accept reduced OID4VCI interoperability for now in exchange for a clear, verifiable trust model. DID-based identification is the industry direction, and `did:web` is currently the most generic practical method for business wallets. `did:jwk` and `did:key` support neither service endpoints nor key rotation, which makes them suitable only for identity wallets rather than issuers and verifiers.

## Consequences

OID4VCI issuers that do not publish `/.well-known/did.json` cannot be accepted as connections.

In practice this will limit OID4VCI issuers to `did:web`, since this endpoint is defined only in the `did:web` specification and probably doesnt combine with other dids suitable for Issuers and Verifiers. However, support for additional DID methods can be added later.

The connection model uses DIDs consistently across SIOPv2, OID4VP, and OID4VCI.

Domain linkage and linked verifiable presentation validation can run before the user accepts an OID4VCI issuer.
