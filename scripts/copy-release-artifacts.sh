#!/usr/bin/env bash
# Copies the signed release artifacts (Android `.aab`, iOS `.ipa`) out of Tauri's nested build
# output directories into /out, ready to be uploaded to the app stores.
#
# /out is git-ignored and created here if it does not exist. It sits outside unime/src-tauri/gen so
# that re-running `tauri android init` / `tauri ios init`, which rewrites that tree, cannot take the
# collected artifacts with it.
#
# Run this after `pnpm tauri android build` and `pnpm tauri ios build` have both produced an
# artifact. Paths resolve from this script's location, so it can be run from anywhere.

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
generated_dir="${repository_root}/unime/src-tauri/gen"
output_dir="${repository_root}/out"

mkdir -p "${output_dir}"

cp -v "${generated_dir}/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab" "${output_dir}/"
cp -v "${generated_dir}/apple/build/arm64/UniMe.ipa" "${output_dir}/"
