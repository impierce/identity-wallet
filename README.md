![banner.svg](https://github.com/impierce/identity-wallet/raw/HEAD/.github/banner.svg)

---

# UniMe - Identity Wallet

Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Tech stack

This app is built with [Tauri 2.0](https://v2.tauri.app/). It uses [Rust](https://www.rust-lang.org/) in the backend and [SvelteKit](https://kit.svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), and [Melt UI](https://melt-ui.com/) in the frontend.

## Develop

This repository uses [pnpm workspaces](https://pnpm.io/workspaces):

| Path                | Description                                             |
| :------------------ | :------------------------------------------------------ |
| `./`                | Project root with Prettier configuration.               |
| `./unime`           | pnpm workspace with UniMe frontend.                     |
| `./unime/src-tauri` | Tauri Rust app (not a pnpm workspace).                  |
| `./identity-wallet` | Rust package with UniMe backend (not a pnpm workspace). |

Get up and running with these steps:

### 1. Set environment variables

Copy the file `.env.example` to `.env` and set the values accordingly.

### 2. Install dependencies

```sh
pnpm i
```

### 3. Generate icons

```sh
cd unime/src-tauri
cargo tauri icon
```

### 4. Run the app

You can run the following commands from the project root:

```sh
# Desktop
pnpm tauri dev

# Android
pnpm tauri android init
pnpm tauri android dev

# iOS
pnpm tauri ios init
pnpm tauri ios dev
```

## Contributing

> [!NOTE]
> Before committing, please make sure the code is formatted, linted, and passes all tests.

### UniMe frontend

Run the following commands from the project root:

```sh
pnpm format
pnpm lint
pnpm test
```

Run these commands from `./unime/src-tauri`:

```sh
cargo fmt
cargo clippy
cargo test
```

### UniMe backend package

Run these commands from `./identity-wallet`:

```sh
cargo fmt
cargo clippy
cargo test
```

### VS Code support

Recommended extensions are listed in `.vscode/extensions.json`.

### Troubleshooting

If you have issues with `cargo tauri build` run the following command.

```sh
# Linux, macOS
rm -rf ~/.cargo/git/checkouts/*

# Windows
rd /s /q "%USERPROFILE%\.cargo\git\checkouts"
```

## Release a new version

1. Search the entire project for the current version string (such as `0.6.2`) and replace them with the new version string.
   Be **cautious** not to replace versions of any other dependencies (in `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`).
2. Run the script in `unime/src-tauri/gen-static/apply.sh` which copies over the changed files into the (untracked) generated folders for Android and iOS.
3. Inside `unime/src-tauri` run `cargo tauri icon`.
4. Run `npm run tauri ios build` and `npm run tauri android build` to build the apps. The iOS build (`.ipa`) will be in `unime/src-tauri/gen/apple/build/arm64` and the Android builds (`.apk` and `.aab`) will be in `unime/src-tauri/gen/android/app/build/outputs/`.
