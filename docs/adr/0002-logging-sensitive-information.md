# ADR 0002: Logging Sensitive Information

## Status

Accepted

## Context

UniMe logs protocol payloads and state transitions for development and diagnostics. Every dispatched action is logged at INFO in `identity-wallet/src/command.rs`, and the `identity_wallet` and `oid4vc*` crates are set to DEBUG in `unime/src-tauri/src/lib.rs`. These logs contain credential contents, authorization request details, and other wallet data.

The configured targets are `Stdout` and `Webview`. There is no file target, so UniMe itself never writes a log file into the app container. That removes app-private log files from device backups and file-level extraction, but it does not mean the logs are ephemeral: on mobile, `tauri-plugin-log` does not write to real stdout, it hands records to the platform logging system, and the OS retains them.

On Android, `TargetKind::Stdout` maps to `android_logger::log`, so records go to logcat and are retained in `logd`'s in-memory ring buffers. Those buffers are exactly what a bug report exports. A bug report can be produced on-device through Developer options, the Quick Settings tile, or the power-menu shortcut, and shared through the normal share sheet. It requires no root, no USB cable, and no attacker-side access; on fully managed devices an MDM can request one remotely. `adb` itself is also not USB-bound, since Android 11 supports wireless debugging. Logcat applies no privacy redaction.

On iOS the same records go through `os_log`. Two details of Tauri's Swift `Logger` matter. Its `enabled` flag is `true` only under `#if DEBUG` and otherwise defaults to `false`, and no code in `tauri` or `tauri-plugin-log` sets it, so release builds currently emit nothing. When logging is enabled, INFO and DEBUG map to `OSLogType.info` and `OSLogType.debug`, which are memory-backed and normally absent from the on-disk store that `sysdiagnose` collects. However, messages are emitted as `%{public}@`, so anything that is captured is captured unredacted.

Exposure is therefore platform-asymmetric, and the Android side dominates the threat model. The iOS behaviour is a side effect of a third-party dependency's defaults rather than a property UniMe controls.

Credentials are encrypted at rest by Stronghold. Values written to logs in plaintext bypass that protection, so logs can disclose data that device access alone would not yield.

Shipped release artifacts are Android `.aab` and iOS `.ipa` only (`scripts/copy-release-artifacts.sh`). Desktop code paths exist for development.

## Decision

Credential contents and protocol payloads may be logged.

Secrets that grant access must never be logged and are redacted with a manual `Debug` implementation, the pattern already used for `CheckPassword`, `UnlockStorage`, and `CreateNew`. This covers at least:

- profile and Stronghold passwords
- transaction codes (`tx_code`)
- authorization codes and pre-authorized codes
- PKCE `code_verifier`
- access and refresh tokens
- private key material

The distinction is deliberate. Logged credential data is a disclosure risk, whereas a logged bearer secret enables active theft for as long as it remains valid.

The following constraints are part of this decision, not incidental implementation details:

- no file log target, so UniMe writes no log file into the app container
- no remote or network log target
- full-state dumps stay behind `cfg!(debug_assertions)` or the `LOG_STATE_UPDATES_TO_CONSOLE` environment variable

## Consequences

Diagnostics stay detailed enough to debug protocol flows against real issuers and verifiers.

On Android release builds, credential contents reach logcat and can leave the device whenever a user is asked to share a bug report, which is a normal and actively encouraged support workflow. This is accepted, and it partially defeats Stronghold's at-rest protection for whatever is logged.

iOS release builds are currently silent, but this depends on a dependency default and may change on any upgrade. It must not be treated as a guarantee.

Existing call sites that log access-granting secrets have to be brought in line with this decision, in particular the `code` and `tx_code` fields on `CodeReceived` and `CredentialOffersSelected`, and the token request and response logging in `send_token_request.rs`.

This decision must be revisited if a file or remote log target is introduced, if UniMe ships desktop, web, or server-side builds, if shared devices are supported, or if the iOS logging default changes.
