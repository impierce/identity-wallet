# Identity Wallet

A Tauri-based Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development

```bash
npm install

# Desktop
npm run tauri dev

# Android
npm run tauri android init
npm run tauri android dev

# iOS
npm run tauri ios init
npm run tauri ios dev
```

### How this project was initialized

```bash
cargo install create-tauri-app # v3.1.1
cargo create-tauri-app --alpha
✔ Project name · identity-wallet
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm)
✔ Choose your package manager · npm
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript
✔ Would you like to setup the project for mobile as well? · yes
```
