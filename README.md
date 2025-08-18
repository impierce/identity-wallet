![banner.svg](https://github.com/impierce/identity-wallet/raw/HEAD/.github/banner.svg)

---

# UniMe - Identity Wallet

Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Tech stack

This app is built with [Tauri 2.0](https://v2.tauri.app/). It uses [Rust](https://www.rust-lang.org/) in the backend and [SvelteKit](https://kit.svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), and [Melt UI](https://melt-ui.com/) in the frontend.

Checkout the [Prerequisites](https://v2.tauri.app/start/prerequisites/) for running Tauri apps on your system to get started.

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

<!-- The environment variables below are required by `aws-lc-sys` via `rustls-platform-verifier -->

> [!NOTE]
> For Android builds, make sure the `ANDROID_NDK_ROOT` and `ANDROID_NDK` environment variables are set to the correct NDK path, e.g.:
>
> ```sh
> export ANDROID_NDK_ROOT=$NDK_HOME
> export ANDROID_NDK=$NDK_HOME
> export CC="$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/bin/armv7a-linux-androideabi21-clang"
> export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$ANDROID_NDK_ROOT/toolchains/llvm/prebuilt/linux-x86_64/sysroot"
> ```
>
> If you're on macOS, you do not need to set `CC` explicitly. Make sure to use `darwin-x86_64` (also for Apple Silicon).
> Also make sure to have `bindgen` installed (via cargo).

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

### Generate TypeScript bindings

In order to regenerate the TypeScript bindings in `identity-wallet/bindings` after making changes to actions or reducers, you can simply execute all Rust tests by running `cargo test` from within the `/identity-wallet` folder.

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

### Debugging

You can simulate safe area insets during development by overriding CSS variables `--safe-area-inset-top` and `--safe-area-inset-button` in `unime/src/app.css`. You can add styling to the safe area insets by setting `PUBLIC_STYLE_SAFE_AREA_INSETS=true` in your `.env`.

## Release a new version

1. Search the entire project for the current version string (such as `0.6.2`) and replace them with the new version string.
   Be **cautious** not to replace versions of any other dependencies (in `Cargo.toml`, `Cargo.lock`, `package.json`, `package-lock.json`).
2. Run the script in `unime/src-tauri/gen-static/apply.sh` which copies over the changed files into the (untracked) generated folders for Android and iOS.
3. Inside `unime/src-tauri` run `cargo tauri icon`.
4. To create a release build, there is a special tweak for the respective platform:
   - For **iOS**, open Xcode and open the root file `unime.xcodeproj`. Go to `Signing & Capabilities`, disable `Automatically manage signing` and select the `Provisioning Profile` manually.
   - For **Android**, create a `keystore.properties` file in `unime/src-tauri/gen/android` which contains the secrets required in `build.gradle.kts` (such as `keyAlias`, etc.).
5. Run `pnpm tauri ios build` and `pnpm tauri android build` to build the apps. The iOS build (`.ipa`) will be in `unime/src-tauri/gen/apple/build/arm64` and the Android builds (`.apk` and `.aab`) will be in `unime/src-tauri/gen/android/app/build/outputs/`.
