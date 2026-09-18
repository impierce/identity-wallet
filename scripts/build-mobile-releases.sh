#!/usr/bin/env bash
# Builds the signed Android (`.aab`) and iOS (`.ipa`) release bundles by running `pnpm tauri
# android build` and `pnpm tauri ios build` from the repository root. Both are always built, so a
# release is never assembled from artifacts of two different runs.
#
# Prerequisites, all of them local and none of them created here:
#   - `scripts/override-generated-mobile-files.sh` has been run over the generated projects.
#   - Android: `unime/src-tauri/gen/android/keystore.properties` holds the signing values.
#   - iOS: the signing certificate is installed and the provisioning profile is selected manually in
#     Xcode (`Signing & Capabilities`, with `Automatically manage signing` disabled).
#
# Building does not publish anything. Collect the results with `scripts/copy-release-artifacts.sh`.
# Paths resolve from this script's location, so it can be run from anywhere.

set -euo pipefail

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script builds for iOS as well, which requires macOS and Xcode. Detected $(uname -s)." >&2
  exit 1
fi

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "${repository_root}"

echo "==> Building the Android release bundle"
pnpm tauri android build

echo "==> Building the iOS release bundle"
pnpm tauri ios build

echo "==> Done. Run scripts/copy-release-artifacts.sh to collect the artifacts into out/."
