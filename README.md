# Identity Wallet

A Tauri-based Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development

### Setup

The npm package `@impierce/ui-components` can only be accessed if you have a [GitHub token](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token).

> GitHub > Settings > Developer settings > Personal access tokens > Tokens (classic)

> Name: "Read private packages", Expiration: 90 days, Scopes: "read:packages"

Then add the following line to your local `~/.npmrc`

```sh
//npm.pkg.github.com/:_authToken=ghp_...
```

### Start the app

```sh
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

Before commiting, please make sure to run

```sh
npm run lint
npm run format
npm run test
```

### How this project was initialized

```sh
cargo install create-tauri-app # v3.1.1
cargo create-tauri-app --alpha
✔ Project name · identity-wallet
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm)
✔ Choose your package manager · npm
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript
✔ Would you like to setup the project for mobile as well? · yes
```
