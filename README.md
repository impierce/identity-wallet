![banner.svg](https://github.com/impierce/identity-wallet/raw/HEAD/.github/banner.svg)

---

# UniMe - Identity Wallet

Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

### Tech stack

- [Tauri 2.0](https://beta.tauri.app/)
- [SvelteKit](https://kit.svelte.dev/), [Tailwind CSS](https://tailwindcss.com/), [Melt UI](https://melt-ui.com/)

## Development

```sh
cd unime
```

### 1. Install dependencies

```sh
npm install
```

### 2. Generate icons

```sh
cd src-tauri
cargo tauri icon
```

### 3. Run the app

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
> Before committing, please make sure to run:

```sh
npm run lint
npm run format
npm run test
```

### Troubleshooting

If you have issues with `cargo tauri build` run the following command.

```sh
# Linux, macOS
rm -rf ~/.cargo/git/checkouts/*

# Windows
rd /s /q "%USERPROFILE%\.cargo\git\checkouts"
```

_This project was initialized using `create-tauri-app v3.1.1`._
