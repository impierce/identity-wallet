# Profile fixture: UniMe 0.18.0

A wallet data directory as written by UniMe 0.18.0 (`AppState` version 1). `tests/tests/profile_fixture.rs` loads it
through `[App] Get state` and `[Storage] Unlock` to make sure that profiles created with this version still open
after changes to the persisted state, the Stronghold snapshot or the key handling.

Do not regenerate or edit these files to make the test pass: a failing test means existing wallets would no longer
load. Add a migration instead, and add a new fixture next to this one when the storage format changes.

## Contents

- `state.json`: the persisted `AppState`.
- `stronghold.bin`: the Stronghold snapshot with the keys and credentials.
- `.gitattributes`: keeps both files byte for byte as written by the app.

The assets folder (downloaded logos) is left out; the wallet works without it.

## Test data only

- Profile name: `Fixture Holder`
- Password: `Fixture123!`

The password and the keys in the snapshot were created only for this fixture. The credentials hold made-up data and
were issued by the Impierce test instances Mira and Capella (`*.dev2.impierce.com`).

| Credential             | Issuer  | Format | Status in the fixture                 |
| ---------------------- | ------- | ------ | ------------------------------------- |
| Dutch Driver's Licence | Mira    | W3C VC | Valid                                 |
| My first badge         | Mira    | OB 3.0 | Expired (`validUntil` 2026-10-01)     |
| Complete VC            | Mira    | W3C VC | Revoked (status list entry `INVALID`) |
| Full Badge             | Capella | OB 3.0 | Valid                                 |
| Testing Credential     | Capella | OB 3.0 | Valid, with an ESCO skill alignment   |

The connections are Mira and Capella, with 7 history events.

## How it was created (2026-10-26)

1. Ran `pnpm dev:instance --name fixture` from a checkout of `dev` at `d3d404c8` (Windows desktop app, fresh data
   directory `%APPDATA%\com.impierce.identity-wallet.fixture`).
2. Went through the regular onboarding: accepted the terms, profile name `Fixture Holder`, password `Fixture123!`.
3. Turned on developer mode (Settings > About UniMe, tap the logo 7 times) to get the paste field on the Scan screen.
4. Created the credential offers with the UniTrust MCP `issue_credential` tool (`send: false`), the expired one with
   `expiresAt` in the past, pasted each `openid-credential-offer://` link on the Scan screen, and accepted the
   connection and the credentials.
5. Revoked "Complete VC" on Mira, locked and unlocked the wallet so the status refresh stored `INVALID`.
6. Turned developer mode off again, closed the app and copied `state.json` and `stronghold.bin` from the data directory.
