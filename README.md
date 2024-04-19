![banner.svg](https://github.com/impierce/identity-wallet/raw/HEAD/.github/banner.svg)

---

# UniMe - Identity Wallet

Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials

### Tech stack

- [Tauri 2.0](https://beta.tauri.app/)
- [SvelteKit](https://kit.svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), [Melt UI](https://melt-ui.com/)

## Development

Run `npm install` in the project root to install [Prettier](https://prettier.io/). Then

```sh
cd unime
```

### 1. Set environment variables

Copy the file `.env.example` to `.env` and set the values accordingly.

### 2. Install dependencies

```sh
npm install
```

### 3. Generate icons

```sh
cd src-tauri
cargo tauri icon
```

### 4. Run the app

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
> Before committing, please make sure the code is formatted correctly, passes all tests and is linted.

Run Prettier from the project root:

```sh
npm run format
```

### Frontend

```sh
# /unime
npm run lint
npm run test

# /unime/src-tauri
cargo fmt
cargo clippy
cargo test
```

### Core (backend)

```sh
# /identity-wallet
cargo fmt
cargo clippy
cargo test
```

### VS Code support

- Recommended extensions are listed with their extension IDs in `.vscode/extensions.json`.

- It is also recommended to enable `Format On Save` in your VS Code settings.

### Troubleshooting

If you have issues with `cargo tauri build` run the following command.

```sh
# Linux, macOS
rm -rf ~/.cargo/git/checkouts/*

# Windows
rd /s /q "%USERPROFILE%\.cargo\git\checkouts"
```

_This project was initialized using `create-tauri-app v3.1.1`._
