# UniMe - Identity Wallet

A Tauri-based Identity Wallet for people to manage Decentralized Identities and Verifiable Credentials.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Development

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

## Release

### Signing Android App Bundle (AAB)

https://docs.flutter.dev/deployment/android#signing-the-app
https://next--tauri.netlify.app/next/guides/distribution/sign-android

Edit `src-tauri/gen/android/app/build.gradle.kts`:

<!-- ```kotlin
import java.util.Properties
import java.nio.file.Files

plugins { ... }

val keystoreProperties = Properties()
val keystorePropertiesFile = rootProject.file("key.properties").toPath()
if (Files.exists(keystorePropertiesFile)) {
    Files.newBufferedReader(keystorePropertiesFile).use { reader ->
        keystoreProperties.load(reader)
    }
}

android {
    compileSdk = 33
    namespace = "com.impierce.unime"
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "com.impierce.unime"
        minSdk = 24
        targetSdk = 33
        versionCode = 1
        versionName = "1.0"
    }
    signingConfigs {
        create("release") {
            storeFile = file(keystoreProperties["storeFile"])
            storePassword = keystoreProperties["storePassword"] as String
            keyAlias = keystoreProperties["keyAlias"] as String
            keyPassword = keystoreProperties["keyPassword"] as String
        }
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
            signingConfig = signingConfigs.getByName("release")
        }
    }
    kotlinOptions {
        jvmTarget = "1.8"
    }
}

rust { ... }

dependencies { ... }

apply(from = "tauri.build.gradle.kts")
``` -->

### Create icons

```sh
cd src-tauri
cargo tauri icon
```

### Adding icons
Use phosphoricons: https://phosphoricons.com/

### Troubleshooting

If you have issues with `cargo tauri build` run the following command.
Linux / Mac:

```shell
rm -rf ~/.cargo/git/checkouts/*
```

Windows:

```cmd
rd /s /q "%USERPROFILE%\.cargo\git\checkouts"
```

### How this project was initialized

```sh
cargo install create-tauri-app # v3.1.1
cargo create-tauri-app --alpha
✔ Project name · unime
✔ Choose which language to use for your frontend · TypeScript / JavaScript - (pnpm, yarn, npm)
✔ Choose your package manager · npm
✔ Choose your UI template · Svelte - (https://svelte.dev/)
✔ Choose your UI flavor · TypeScript
✔ Would you like to setup the project for mobile as well? · yes
```
