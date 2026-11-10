#!/usr/bin/env bash
# Uploads the release artifacts in /out to the internal testing tracks of both stores, never further:
#   - Android: the `.aab` to the Google Play internal testing track, rolled out to its testers.
#   - iOS: the `.ipa` to TestFlight, then the processed build to the internal testing groups.
# It never submits a build for (beta) review or to a production or external track.
#
# Usage: scripts/upload-mobile-releases.sh [--only android|ios] [--dry-run]
#   --only     Upload to one store only, for example to retry after the other upload succeeded.
#   --dry-run  Check tools, configuration and artifacts, then stop before uploading anything.
#
# Configuration comes from the git-ignored .env at the repository root (see .env.example) or from the environment,
# which takes precedence. Group names are not printed, so they stay out of terminal and CI logs.

set -euo pipefail

repository_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output_dir="${repository_root}/out"
aab_path="${output_dir}/app-universal-release.aab"
ipa_path="${output_dir}/UniMe.ipa"
android_package_name="com.impierce.identity_wallet"
ios_bundle_id="com.impierce.identity-wallet"

config_keys=(
  PLAY_SERVICE_ACCOUNT_JSON
  ASC_KEY_ID
  ASC_ISSUER_ID
  ASC_KEY_P8_PATH
  TESTFLIGHT_INTERNAL_GROUPS
  IOS_USES_NON_EXEMPT_ENCRYPTION
)

upload_android=true
upload_ios=true
dry_run=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --only)
      case "${2:-}" in
        android) upload_ios=false ;;
        ios) upload_android=false ;;
        *)
          echo "--only expects 'android' or 'ios'." >&2
          exit 2
          ;;
      esac
      shift 2
      ;;
    --dry-run)
      dry_run=true
      shift
      ;;
    -h | --help)
      sed -n '2,12p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      exit 2
      ;;
  esac
done

# Reads the known keys from .env without executing it. Variables already set in the environment win.
load_env_file() {
  local env_file="${repository_root}/.env"
  [[ -f "${env_file}" ]] || return 0
  local line key value
  while IFS= read -r line || [[ -n "${line}" ]]; do
    [[ "${line}" =~ ^[[:space:]]*(export[[:space:]]+)?([A-Z_][A-Z0-9_]*)=(.*)$ ]] || continue
    key="${BASH_REMATCH[2]}"
    value="${BASH_REMATCH[3]}"
    [[ " ${config_keys[*]} " == *" ${key} "* ]] || continue
    [[ -z "${!key:-}" ]] || continue
    if [[ "${value}" =~ ^\"(.*)\"[[:space:]]*$ || "${value}" =~ ^\'(.*)\'[[:space:]]*$ ]]; then
      value="${BASH_REMATCH[1]}"
    else
      value="${value%%[[:space:]]#*}"
      value="${value%"${value##*[![:space:]]}"}"
    fi
    printf -v "${key}" '%s' "${value}"
  done <"${env_file}"
}

# Expands a leading ~ and resolves paths relative to the repository root.
resolve_path() {
  local path="$1"
  [[ "${path}" == "~/"* ]] && path="${HOME}/${path#"~/"}"
  [[ "${path}" == /* ]] || path="${repository_root}/${path}"
  printf '%s' "${path}"
}

problems=()
problem() { problems+=("$1"); }

load_env_file

echo "==> Checking tools"
if ! command -v fastlane >/dev/null 2>&1; then
  problem "fastlane is not installed. Install it with 'brew install fastlane' (see https://docs.fastlane.tools)."
fi
if ! command -v node >/dev/null 2>&1; then
  problem "node is not installed. It is needed to read the app version and write the App Store Connect key file."
fi
if [[ "${upload_ios}" == true ]]; then
  if [[ "$(uname -s)" != "Darwin" ]]; then
    problem "Uploading to TestFlight requires macOS and Xcode. Detected $(uname -s); use '--only android'."
  else
    xcrun --find altool >/dev/null 2>&1 || problem "Xcode's altool was not found. Install Xcode and select it with 'xcode-select'."
    [[ -x /usr/libexec/PlistBuddy ]] || problem "/usr/libexec/PlistBuddy was not found; it is needed to read the .ipa's Info.plist."
  fi
  command -v unzip >/dev/null 2>&1 || problem "unzip is not installed; it is needed to read the .ipa's Info.plist."
fi

echo "==> Checking configuration and artifacts"
if [[ "${upload_android}" == true ]]; then
  if [[ -z "${PLAY_SERVICE_ACCOUNT_JSON:-}" ]]; then
    problem "PLAY_SERVICE_ACCOUNT_JSON is not set. Point it to the Google Play service account's JSON key."
  else
    PLAY_SERVICE_ACCOUNT_JSON="$(resolve_path "${PLAY_SERVICE_ACCOUNT_JSON}")"
    [[ -f "${PLAY_SERVICE_ACCOUNT_JSON}" ]] || problem "PLAY_SERVICE_ACCOUNT_JSON points to a missing file."
  fi
  [[ -f "${aab_path}" ]] || problem "Missing ${aab_path}. Run scripts/copy-release-artifacts.sh first."
fi

if [[ "${upload_ios}" == true ]]; then
  [[ -n "${ASC_KEY_ID:-}" ]] || problem "ASC_KEY_ID is not set (the App Store Connect API key's ID)."
  [[ -n "${ASC_ISSUER_ID:-}" ]] || problem "ASC_ISSUER_ID is not set (the App Store Connect API issuer ID)."
  if [[ -z "${ASC_KEY_P8_PATH:-}" ]]; then
    problem "ASC_KEY_P8_PATH is not set. Point it to the App Store Connect API key's .p8 file."
  else
    ASC_KEY_P8_PATH="$(resolve_path "${ASC_KEY_P8_PATH}")"
    [[ -f "${ASC_KEY_P8_PATH}" ]] || problem "ASC_KEY_P8_PATH points to a missing file."
  fi
  [[ -n "${TESTFLIGHT_INTERNAL_GROUPS:-}" ]] ||
    problem "TESTFLIGHT_INTERNAL_GROUPS is not set (comma-separated names of the internal TestFlight groups)."
  case "${IOS_USES_NON_EXEMPT_ENCRYPTION:-}" in
    "" | true | false) ;;
    *) problem "IOS_USES_NON_EXEMPT_ENCRYPTION must be 'true' or 'false'." ;;
  esac
  [[ -f "${ipa_path}" ]] || problem "Missing ${ipa_path}. Run scripts/copy-release-artifacts.sh first."
fi

if [[ ${#problems[@]} -gt 0 ]]; then
  echo "Cannot upload:" >&2
  printf '  - %s\n' "${problems[@]}" >&2
  exit 1
fi

expected_version="$(node -p "require('${repository_root}/unime/src-tauri/tauri.conf.json').version")"

if [[ "${upload_ios}" == true ]]; then
  ipa_info_plist="$(mktemp)"
  trap 'rm -f "${ipa_info_plist}"' EXIT
  unzip -p "${ipa_path}" 'Payload/*.app/Info.plist' >"${ipa_info_plist}"
  ipa_version="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "${ipa_info_plist}")"
  if [[ "${ipa_version}" != "${expected_version}" ]]; then
    echo "The .ipa has version ${ipa_version}, but the repository is at ${expected_version}. Rebuild it first." >&2
    exit 1
  fi
  # Without the Info.plist key, App Store Connect holds the build until export compliance is answered, and fastlane
  # would answer it with its default 'false'. Require an explicit answer instead.
  if ! /usr/libexec/PlistBuddy -c 'Print :ITSAppUsesNonExemptEncryption' "${ipa_info_plist}" >/dev/null 2>&1 &&
    [[ -z "${IOS_USES_NON_EXEMPT_ENCRYPTION:-}" ]]; then
    echo "The .ipa does not declare ITSAppUsesNonExemptEncryption, so export compliance must be answered." >&2
    echo "Set IOS_USES_NON_EXEMPT_ENCRYPTION to 'true' or 'false', or add the key to the tracked Info.plist." >&2
    exit 1
  fi
fi

# fastlane splits the groups on commas without trimming, so normalize "A, B" to "A,B".
group_list=()
IFS=',' read -r -a raw_groups <<<"${TESTFLIGHT_INTERNAL_GROUPS:-}"
for group in "${raw_groups[@]+"${raw_groups[@]}"}"; do
  group="${group#"${group%%[![:space:]]*}"}"
  group="${group%"${group##*[![:space:]]}"}"
  [[ -n "${group}" ]] && group_list+=("${group}")
done
testflight_groups="$(IFS=','; printf '%s' "${group_list[*]+"${group_list[*]}"}")"
if [[ "${upload_ios}" == true && -z "${testflight_groups}" ]]; then
  echo "TESTFLIGHT_INTERNAL_GROUPS contains no group names." >&2
  exit 1
fi

echo "Version: ${expected_version}"
[[ "${upload_android}" == true ]] && echo "Android: ${aab_path} -> Google Play internal testing track"
[[ "${upload_ios}" == true ]] && echo "iOS:     ${ipa_path} -> TestFlight, ${#group_list[@]} internal group(s)"

if [[ "${dry_run}" == true ]]; then
  echo "==> Dry run: everything is in place, nothing was uploaded."
  exit 0
fi

# Keep fastlane's option tables (which would list the group names), update checks and usage metrics out of it.
export FASTLANE_SKIP_ALL_LANE_SUMMARIES=1
export FASTLANE_SKIP_UPDATE_CHECK=1
export FASTLANE_OPT_OUT_USAGE=1
export FASTLANE_HIDE_CHANGELOG=1

if [[ "${upload_android}" == true ]]; then
  echo "==> Uploading the Android bundle to the Google Play internal testing track"
  fastlane supply \
    --aab "${aab_path}" \
    --package_name "${android_package_name}" \
    --json_key "${PLAY_SERVICE_ACCOUNT_JSON}" \
    --track internal \
    --release_status completed \
    --skip_upload_apk true \
    --skip_upload_metadata true \
    --skip_upload_changelogs true \
    --skip_upload_images true \
    --skip_upload_screenshots true
fi

if [[ "${upload_ios}" == true ]]; then
  echo "==> Uploading the iOS build to TestFlight and distributing it to the internal groups"
  # fastlane takes the App Store Connect key as a JSON file; keep it private and remove it afterwards.
  api_key_json="$(mktemp)"
  trap 'rm -f "${ipa_info_plist:-}" "${api_key_json}"' EXIT
  chmod 600 "${api_key_json}"
  ASC_KEY_ID="${ASC_KEY_ID}" ASC_ISSUER_ID="${ASC_ISSUER_ID}" ASC_KEY_P8_PATH="${ASC_KEY_P8_PATH}" node -e '
    const { writeFileSync } = require("node:fs");
    writeFileSync(process.argv[1], JSON.stringify({
      key_id: process.env.ASC_KEY_ID,
      issuer_id: process.env.ASC_ISSUER_ID,
      key_filepath: process.env.ASC_KEY_P8_PATH,
      in_house: false,
    }));
  ' "${api_key_json}"

  encryption_args=()
  [[ -n "${IOS_USES_NON_EXEMPT_ENCRYPTION:-}" ]] &&
    encryption_args=(--uses_non_exempt_encryption "${IOS_USES_NON_EXEMPT_ENCRYPTION}")

  # Internal groups need no beta review, and fastlane would otherwise request one as soon as groups are given.
  # The groups are passed through the environment rather than the command line.
  PILOT_GROUPS="${testflight_groups}" fastlane pilot upload \
    --ipa "${ipa_path}" \
    --app_identifier "${ios_bundle_id}" \
    --api_key_path "${api_key_json}" \
    --distribute_external false \
    --submit_beta_review false \
    --notify_external_testers false \
    --skip_waiting_for_build_processing false \
    "${encryption_args[@]+"${encryption_args[@]}"}"
fi

echo "==> Done. Check both stores' internal testing pages before telling the team."
