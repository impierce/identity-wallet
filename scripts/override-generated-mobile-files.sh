#!/usr/bin/env bash
# Overwrites Tauri's generated, git-ignored mobile projects (unime/src-tauri/gen) with the tracked
# customizations in unime/src-tauri/gen-static. Any manual edit made directly in /gen is lost.

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
static_dir="${repository_root}/unime/src-tauri/gen-static"
generated_dir="${repository_root}/unime/src-tauri/gen"

cp -v "${static_dir}/android/app/build.gradle.kts" "${generated_dir}/android/app/build.gradle.kts"
cp -v "${static_dir}/android/app/proguard-rules.pro" "${generated_dir}/android/app/proguard-rules.pro"
cp -v "${static_dir}/android/app/tauri.properties" "${generated_dir}/android/app/tauri.properties"
cp -v "${static_dir}/android/app/src/main/java/com/impierce/identity_wallet/MainActivity.kt" "${generated_dir}/android/app/src/main/java/com/impierce/identity_wallet/MainActivity.kt"
cp -v "${static_dir}/android/app/src/main/AndroidManifest.xml" "${generated_dir}/android/app/src/main/AndroidManifest.xml"
cp -v "${static_dir}/android/gradle/libs.versions.toml" "${generated_dir}/android/gradle/libs.versions.toml"
cp -v "${static_dir}/apple/ExportOptions.plist" "${generated_dir}/apple/ExportOptions.plist"
cp -v "${static_dir}/apple/unime_iOS/Info.plist" "${generated_dir}/apple/unime_iOS/Info.plist"
