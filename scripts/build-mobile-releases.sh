#!/usr/bin/env bash
# Builds both signed release bundles, never one, so a release cannot be assembled from two runs.
#
# Expects `override-generated-mobile-files.sh` to have run, Android signing values in
# `unime/src-tauri/gen/android/keystore.properties`, and an iOS certificate with a manually selected
# provisioning profile in Xcode.

set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script builds for iOS as well, which requires macOS and Xcode. Detected $(uname -s)." >&2
  exit 1
fi

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repository_root}"

echo "==> Building the Android release bundle"
# libsodium's `./configure` archives with the host `ar` and `ranlib`. Since Xcode 27 these silently drop non-Mach-O
# objects, so libsodium.a ends up empty and the app crashes on launch with `sodium_free` undefined. Use the NDK's
# LLVM tools instead, and rebuild libsodium because its build script does not rerun when `AR` changes.
ndk_bin="${NDK_HOME:?Set NDK_HOME to the Android NDK directory}/toolchains/llvm/prebuilt/darwin-x86_64/bin"
cargo clean --release --package libsodium-sys-stable --target aarch64-linux-android \
  --target armv7-linux-androideabi --target i686-linux-android --target x86_64-linux-android
AR="${ndk_bin}/llvm-ar" RANLIB="${ndk_bin}/llvm-ranlib" pnpm tauri android build

echo "==> Building the iOS release bundle"
pnpm tauri ios build

echo "==> Done. Run scripts/copy-release-artifacts.sh to collect the artifacts into out/."
