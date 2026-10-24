# Identity Wallet AI Instructions

Use this file as the primary source of truth for this repository. Prefer these instructions over assumptions, and search the codebase only when the guidance here is incomplete or contradicted.

## Repository context
- Project: UniMe Identity Wallet, a Svelte 5 + Tauri 2.0 application for mobile and desktop.
- Domain: European Digital Identity Wallet workflows around credentials, verifiable credentials, and secure identity management.
- Architecture: the frontend lives in the Svelte app under unime, while backend state and command flow are coordinated through Rust crates under identity-wallet and unime/src-tauri.

## Key paths
- Repo root: `./`
- Frontend workspace: `unime/`
- App routes: `unime/src/routes/`
- Shared UI and utilities: `unime/src/lib/`
- Shared UI primitives: `unime/src/lib/components/`
- Base English translations: `unime/src/i18n/en/index.ts`
- Rust backend package: `identity-wallet/`
- Tauri app: `unime/src-tauri/`
- Tauri config: `unime/src-tauri/tauri.conf.json`
- Generated TypeScript bindings: `identity-wallet/bindings/`

## Environment and workflow
- Start from unime/.env.example to create a local .env. Do not overwrite an existing .env file.
- Use pnpm for Node-based work. Do not add npm.lock or yarn.lock.
- Tauri prerequisites are required before running the app.
- From the repository root, you can run the app with:
  - Desktop: pnpm tauri dev
  - Android: pnpm tauri android init then pnpm tauri android dev
  - iOS: pnpm tauri ios init then pnpm tauri ios dev
- If app icons need to be regenerated, run: cd unime/src-tauri && cargo tauri icon

## Required validation
Before finishing work, run the relevant checks:
- Repo root: pnpm format, pnpm lint, pnpm test, pnpm check
- Tauri app: cd unime/src-tauri && cargo fmt && cargo clippy && cargo test
- Backend package: cd identity-wallet && cargo fmt && cargo clippy && cargo test

## Coding conventions
- Use Svelte 5 runes syntax only: $state, $derived, and $props.
- Do not introduce legacy Svelte 4 patterns.
- Never hardcode user-facing text. Add it to unime/src/i18n/en/index.ts first, then run pnpm typesafe-i18n from the unime folder.
- Prefer shared primitives in unime/src/lib/components/ and existing project patterns over introducing new abstractions.
- When changing backend actions or reducers, inspect the existing bindings in identity-wallet/bindings/ before relying on command signatures.
- Treat generated files as generated artifacts; avoid hand-editing them unless the change is intentional and validated.

## Styling and quality rules
- Follow the existing formatting and linting rules enforced by Prettier and ESLint.
- Preserve existing TODO comments unless explicitly asked to remove them.
- Keep changes surgical and consistent with the surrounding code.

## Troubleshooting
- If cargo tauri build fails, clear the cached Rust git checkouts:
  - Linux/macOS: rm -rf ~/.cargo/git/checkouts/*
  - Windows: rd /s /q "%USERPROFILE%\.cargo\git\checkouts"

## Agent behavior
- Prefer minimal, targeted edits over broad rewrites.
- Verify behavior with the relevant lint, typecheck, or test command before claiming completion.
- If a requirement is ambiguous, inspect the repository and follow existing conventions rather than inventing a pattern.