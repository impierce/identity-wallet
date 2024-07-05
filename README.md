![banner.svg](https://github.com/impierce/identity-wallet/raw/HEAD/.github/banner.svg)

---

# UniMe - Identity Wallet

Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Tech stack

This app is built with [Tauri 2.0](https://v2.tauri.app/). It uses [Rust](https://www.rust-lang.org/) in the back-end and [SvelteKit](https://kit.svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), and [Melt UI](https://melt-ui.com/) in the front-end.

## Develop

This repository uses [NPM workspaces](https://docs.npmjs.com/cli/v10/using-npm/workspaces):

| Path                | Description                                              |
| :------------------ | :------------------------------------------------------- |
| `./`                | Project root with Prettier configuration.                |
| `./unime`           | NPM workspace UniMe front-end.                           |
| `./unime/src-tauri` | Tauri Rust app (not an NPM workspace).                   |
| `./identity-wallet` | Rust package with UniMe back-end (not an NPM workspace). |

Get up and running with these steps:

### 1. Set environment variables

Copy the file `.env.example` to `.env` and set the values accordingly.

### 2. Install dependencies

```sh
npm install
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
npm run tauri dev

# Android
npm run tauri android init
npm run tauri android dev

# iOS
npm run tauri ios init
npm run tauri ios dev
```

## Contributing

> [!NOTE]
> Before committing, please make sure the code is formatted, linted, and passes all tests.

### UniMe frontend

Run the following commands from the project root:

```sh
npm run format
npm run lint
npm run test
```

Run these commands from `./unime/src-tauri`:

```sh
cargo fmt
cargo clippy
cargo test
```

### UniMe back-end package

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
