# ADR 0003: No Personal Data in Logs

## Status

Accepted. Supersedes the decision in [ADR 0002](0002-logging-sensitive-information.md) that credential contents and
protocol payloads may be logged. The context and the remaining constraints of ADR 0002 still apply.

## Context

ADR 0002 accepted that credential contents reach logcat on Android release builds and can leave the device in a bug
report. GDPR data minimisation (Art. 5(1)(c)) and data protection by design (Art. 25) ask for the opposite default:
UniMe should not write a Holder's personal data to logs it does not need for debugging. Logs also reach the webview
console, so anyone looking at a shared debug session sees them.

## Decision

Log lines contain only:

- IDs (credential, connection, query, and session IDs, DIDs, key IDs)
- credential types and credential configuration IDs
- counts, statuses, and durations
- issuer and verifier names and hosts, and the endpoints the wallet calls

Log lines never contain:

- credentials, claim values, or credential offers
- access and refresh tokens, authorization and pre-authorized codes, transaction codes, PKCE verifiers, `state`,
  nonces, `auth_session` values, or request URIs
- VP tokens, ID tokens, proof JWTs, Data Access Consent Tokens, or public links to a credential
- the Holder's profile name, email address, search terms, or other data the Holder typed
- action payloads, the app state, QR code contents, or deep link URLs
- passwords and key material

Release builds log at INFO. DEBUG is only available in debug builds. A developer who needs full payloads sets
`LOG_FULL_PAYLOADS=true` in `unime/.env` in a debug build; `identity_wallet::log_payload!` then logs them at DEBUG. It
is a no-op in release builds, whatever the environment says. The full-state dump behind `LOG_STATE_UPDATES_TO_CONSOLE`
is likewise limited to debug builds.

Full payload logging is tied to debug builds and not to Dev Mode, because Dev Mode can be switched on in release
builds through the settings screen.

The same rules apply to the openid4vc library, whose `oid4vc*` and `siopv2` crates log through the same pipeline.

## Consequences

Release logs no longer disclose credential data or secrets, and they stay useful for following a flow: which action
ran, which issuer or verifier was involved, how many credentials matched, and how it ended.

Debugging a payload-level problem needs a debug build. Error messages from the openid4vc library report the size of
an unparseable response instead of its raw body, for the responses that can carry codes, tokens, or credentials.

New log lines must follow these rules. `AGENTS.md` repeats them for contributors and agents.
