#!/usr/bin/env bash
# Collects the signed `.aab` and `.ipa` into the git-ignored /out, created here if missing. It sits
# outside unime/src-tauri/gen so `tauri android init` / `tauri ios init` cannot wipe them.

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
generated_dir="${repository_root}/unime/src-tauri/gen"
output_dir="${repository_root}/out"

mkdir -p "${output_dir}"

cp -v "${generated_dir}/android/app/build/outputs/bundle/universalRelease/app-universal-release.aab" "${output_dir}/"
cp -v "${generated_dir}/apple/build/arm64/UniMe.ipa" "${output_dir}/"
