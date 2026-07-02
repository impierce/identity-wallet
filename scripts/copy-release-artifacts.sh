# Copies the release artifacts (iOS, Android) from the generated resources to the /gen root folder to be uploaded to the app stores.

cp -v ../unime/src-tauri/gen/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab ../unime/src-tauri/gen/
cp -v ../unime/src-tauri/gen/apple/build/arm64/UniMe.ipa ../unime/src-tauri/gen/
