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
pnpm tauri android build

echo "==> Building the iOS release bundle"
pnpm tauri ios build

echo "==> Done. Run scripts/copy-release-artifacts.sh to collect the artifacts into out/."
