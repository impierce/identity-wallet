# Repository guide

This repository contains **UniMe**, a Tauri 2 identity wallet for decentralized identities and verifiable credentials.
The application has a SvelteKit frontend and a Rust backend split across two Cargo crates. Treat the root as both a
pnpm workspace and a Cargo workspace.

## Prerequisites

- Node.js 22 or newer and pnpm 10 or newer (enforced by `unime/package.json` and `.npmrc`).
- A stable Rust toolchain with `rustfmt` and `clippy`. The workspace declares Rust 1.76 as its minimum supported
  version, but development and CI use stable.
- The platform prerequisites from the Tauri 2 documentation. Linux needs WebKitGTK and related system libraries;
  mobile development additionally needs Android Studio/SDK/NDK or Xcode.
- Network access and Git credentials, when needed, because several Rust dependencies are pinned Git dependencies.

## Initial setup

Run commands from the repository root unless a different directory is shown.

```sh
pnpm install
cp unime/.env.example unime/.env
cp identity-wallet/.env.example identity-wallet/.env
```

The environment files are ignored by Git. Frontend feature and debugging flags live in `unime/.env`. The backend
email-verification settings live in `identity-wallet/.env`; its build script supplies production-host/empty-key
defaults if they are absent. Never commit real keys or secrets.

Generate application icons when setting up a fresh checkout or after changing `unime/src-tauri/app-icon.png`:

```sh
cd unime/src-tauri
cargo tauri icon
```

## Common commands

The root `package.json` proxies frontend and Tauri commands into the `unime` workspace.

```sh
# Run the desktop application (Vite plus Tauri)
pnpm tauri dev

# Frontend-only development server at http://localhost:4173
pnpm --filter unime dev

# Frontend build and static/type checks
pnpm build
pnpm check

# Formatting, linting, and tests
pnpm format
pnpm format:check
pnpm lint
pnpm test

# Non-watch frontend test run and coverage
pnpm --filter unime exec vitest run
pnpm --filter unime coverage
```

Mobile commands are `pnpm tauri android init|dev|build` and `pnpm tauri ios init|dev|build`. Do not initialize or
regenerate a mobile project unless the task calls for it. Tracked platform customizations live under
`unime/src-tauri/gen-static/`; `scripts/override-generated-mobile-files.sh` overwrites the matching files in Tauri's
generated, ignored platform directories with them.

Rust validation can be run per package, matching CI:

```sh
cd identity-wallet
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test

cd ../unime/src-tauri
cargo fmt -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Build the frontend with `pnpm build` before checking the Tauri crate when its configured `frontendDist` does not
exist. For a focused Rust test, use `cargo test <test_name>` in the owning crate. Run `cargo audit` and `pnpm audit`
for dependency audits; documented Rust advisory exceptions are in `.cargo/audit.toml`.

## Repository structure

```text
.
├── Cargo.toml                 Rust workspace (`identity-wallet`, `unime/src-tauri`)
├── package.json               Root pnpm command façade and shared Prettier tooling
├── pnpm-workspace.yaml        pnpm workspace (the `unime` package)
├── identity-wallet/           Reusable Rust business-logic and persistence crate
│   ├── bindings/              Generated, checked-in TypeScript types consumed by the frontend
│   ├── resources/             JSON schemas and bundled backend images
│   └── src/
│       ├── command.rs         Root action reducer, persistence, and Tauri event emission
│       ├── persistence.rs     State and Stronghold persistence
│       ├── migrations.rs      Serialized AppState migrations
│       ├── stronghold.rs      Sensitive storage integration
│       └── state/             AppState plus feature actions and reducers
├── unime/
│   ├── src/                   SvelteKit frontend
│   │   ├── lib/               Shared components, stores, dispatcher, utilities, and tests
│   │   ├── routes/            File-based UI routes and route-local components
│   │   └── i18n/              Base translations, locales, and generated typesafe-i18n files
│   ├── static/                Frontend static assets
│   └── src-tauri/             Tauri application shell and Rust integration tests
│       ├── capabilities/      Tauri permission capabilities
│       ├── gen-static/        Source-of-truth mobile platform customizations
│       ├── resources/         Bundled app resources
│       ├── src/lib.rs         Plugin setup and the `handle_action` Tauri command
│       ├── tauri.conf.json    App, build, bundle, CSP, and deep-link configuration
│       └── tests/             Backend/Tauri integration tests and JSON fixtures
├── docs/                      Architecture decisions and debugging notes
└── scripts/                   Release helper scripts
```

The important frontend route groups are `(app)` for the unlocked wallet, `(journey)` for guided journeys,
`welcome` for onboarding, `prompt` for backend-requested interactions, and `credentials/[id]` for credential detail
renderers.

## Architecture and data flow

The Rust backend is the source of truth for application state. It follows a Redux-like action/reducer design:

1. Frontend code calls `dispatch()` in `unime/src/lib/dispatcher.ts` with a generated `Action` type.
2. Tauri invokes the single `handle_action` command in `unime/src-tauri/src/lib.rs`.
3. `identity-wallet/src/command.rs` runs every reducer registered for that action, persists the result, and emits a
   `state-changed` event (or an `error` event).
4. The root frontend layout receives that event and replaces the `state` store in `unime/src/lib/stores.ts`.

Do not mutate the frontend `state` store to implement business behavior. Add or reuse a typed Rust action and reducer
and let the emitted state update drive the UI. Keep business and protocol logic in `identity-wallet`; the Tauri crate
should remain an integration shell around platform plugins and the wallet crate.

Feature modules under `identity-wallet/src/state/` generally contain a state model plus `actions/` and `reducers/`.
Actions implement `ActionTrait`, use the serialized `[Feature] Action name` convention, and declare their reducers.
Reducers return a new `AppState` or `AppError`; on failure, the previous state is retained and an error is emitted.

## Generated files and schema changes

- `identity-wallet/bindings/` is generated by `ts-rs` and is intentionally committed. Do not hand-edit it. After
  changing exported state or action types, run `cargo test` from `identity-wallet/` and commit the resulting binding
  changes. Ensure new frontend-dispatchable actions are also represented in the binding enum in
  `identity-wallet/src/state/actions.rs`.
- For translations, edit `unime/src/i18n/en/index.ts`, run `pnpm --filter unime typesafe-i18n`, then update every
  locale under `unime/src/i18n/*/index.ts` until `pnpm check` succeeds. Most top-level i18n support files are
  generated.
- `AppState` is persisted. For an incompatible serialized-state change, increment `APP_STATE_VERSION`, add a
  sequential migration and tests in `identity-wallet/src/migrations.rs`, and update
  `identity-wallet/src/migrations/CHANGELOG.md`. Preserve backward compatibility rather than relying only on Serde
  defaults.
- Keep Tauri npm and Rust packages on compatible versions and update both sides together.
- Do not edit ignored Tauri-generated Android/iOS directories directly. Change `gen-static` and apply it through its
  script when a generated project needs updating.

## Coding conventions

- Prettier is the source of truth for JavaScript, TypeScript, Svelte, JSON, Markdown, and CSS. It uses two spaces,
  single quotes, semicolons, a 120-column width, sorted imports, and Tailwind class sorting.
- Rust uses `cargo fmt` with a 120-column width. Clippy warnings are CI failures.
- Frontend imports use `$lib`, `$i18n`, and `@bindings` aliases. Prefer shared UI in `unime/src/lib/components`; keep
  page-specific components and helpers next to their routes.
- Place frontend unit tests beside the source as `*.test.ts` or `*.spec.ts`. Vitest runs them in jsdom. Mock Tauri
  boundaries and keep these tests focused on frontend behavior.
- Test Rust reducers thoroughly. Unit tests live beside Rust modules; full action-to-state tests live in
  `unime/src-tauri/tests/` and pair action fixtures with expected-state fixtures.
- Follow existing naming and serialized action strings exactly; they form the frontend/backend protocol.
- Review relevant decisions in `docs/adr/` before changing identity/trust behavior or logging.

## Release process

UniMe does not strictly follow semantic versioning or use an automated semantic-release workflow. There can only be
one current version live in each app store, so releases use a manually selected version number and a controlled,
manual promotion process.

The current version string occurs in eleven authoritative places across nine files (the Apple plist contains it
twice, and `Cargo.lock` carries an entry per workspace member):

1. `Cargo.toml` (`workspace.package.version`)
2. `identity-wallet/Cargo.toml`
3. `unime/package.json`
4. `unime/src-tauri/Cargo.toml`
5. `unime/src-tauri/tauri.conf.json`
6. `unime/src-tauri/gen-static/android/app/tauri.properties`
7. `unime/src-tauri/gen-static/apple/unime_iOS/Info.plist` (`CFBundleShortVersionString` and `CFBundleVersion`)
8. `unime/src/routes/(app)/me/settings/about/+page.svelte`
9. `Cargo.lock` (the `identity-wallet` and `unime` package entries)

For a release:

1. Choose the next version and run `pnpm version:bump <new-version>`. Use `--dry-run` to verify the targets without
   changing them. The script infers the current version and replaces it only after finding all eleven expected
   occurrences; alternatively, update the same locations by hand. In `Cargo.lock` it rewrites only the two workspace
   package entries, so the bump needs no Cargo run to be complete. Review the resulting diff. Do not globally replace
   unrelated dependency versions or other lockfile values; allow Cargo and pnpm to update the rest of their lockfile
   metadata when the relevant commands run.
2. Commit the version bump as a dedicated, reviewable change.
3. Run `scripts/override-generated-mobile-files.sh` to overwrite Tauri's ignored generated mobile projects with the
   tracked customizations, then regenerate icons with `cargo tauri icon` from `unime/src-tauri` when the icon source
   changed.
4. Build and sign both apps locally. Android requires the local signing values in
   `unime/src-tauri/gen/android/keystore.properties`; iOS requires the appropriate signing certificate and manually
   selected provisioning profile in Xcode. Signing credentials are local secrets and must never be committed.
5. Produce both store artifacts with `scripts/build-mobile-releases.sh`, which runs `pnpm tauri android build` and
   `pnpm tauri ios build`. The Android outputs are under
   `unime/src-tauri/gen/android/app/build/outputs/`, and the iOS `.ipa` is under
   `unime/src-tauri/gen/apple/build/arm64/`. `scripts/copy-release-artifacts.sh` collects the `.aab` and `.ipa` into
   the git-ignored `out/` directory at the repository root for upload.
6. Upload the signed artifacts to their respective app stores and manually make the new builds available to the team
   through the stores' internal-testing channels.
7. After the team has tested and approved both builds, manually submit them for store review and public release.

Do not publish a build, submit it for review, or change store availability unless the task explicitly authorizes that
external action. A successful local build is not by itself approval to release.

## Git and remote operations

- Never commit or push code unless the user explicitly approves that specific operation.
- Do not perform any other remote operation unless it has been explicitly approved.
- You may stage changes and propose a short, concise commit message that summarizes the current changes.
- Do not include a commit message body unless the user requests one. All commits are squashed when merged, so a body
  is not normally needed.

## Security and debugging

Wallet state, credentials, protocol payloads, passwords, authorization codes, tokens, and key material are sensitive.
Never add access-granting secrets to logs. Use redacted/manual `Debug` implementations for action payloads containing
secrets, following `docs/adr/0002-logging-sensitive-information.md`. Do not add file or remote logging without
revisiting that decision.

In debug builds, state updates are written to `debug/state.json` by default. Set
`LOG_STATE_UPDATES_TO_CONSOLE=true` in `unime/.env` to print them instead. The `debug/`, `.env`, build output,
coverage, `out/`, `node_modules/`, and `target/` paths are local artifacts and must not be committed.

## Before handing off a change

Run the narrowest relevant tests while iterating, then validate all affected layers. A typical cross-stack change
should pass:

```sh
pnpm format:check
pnpm lint
pnpm test
pnpm check
(cd identity-wallet && cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test)
(cd unime/src-tauri && cargo fmt -- --check && cargo clippy --all-targets --all-features -- -D warnings && cargo test)
```

Report any checks that could not be run because of missing platform tooling, network services, or credentials.
