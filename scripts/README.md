# Release scripts

Run these in order to cut a release. Each one is independent: none of them calls another. Only the
last one publishes anything, and only to the stores' internal testing tracks.

| #   | Script                                             | What it does                                                                                                                                                         |
| --- | -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | `pnpm version:bump <version>` (`bump-version.mjs`) | Replaces the current version in the eleven authoritative locations, including `Cargo.lock`'s two workspace entries. `--dry-run` reports the targets without writing. |
| 2   | `override-generated-mobile-files.sh`               | Overwrites Tauri's generated, git-ignored mobile projects (`unime/src-tauri/gen`) with the tracked customizations in `unime/src-tauri/gen-static`.                   |
| 3   | `build-mobile-releases.sh`                         | Builds both signed release bundles via `pnpm tauri android build` and `pnpm tauri ios build`. macOS only.                                                            |
| 4   | `copy-release-artifacts.sh`                        | Collects the `.aab` and `.ipa` into the git-ignored `out/` folder at the repository root.                                                                            |
| 5   | `upload-mobile-releases.sh`                        | Uploads both artifacts with fastlane to the Google Play internal testing track and to TestFlight's internal groups. `--dry-run` checks everything without uploading. |

Step 5 needs fastlane (`brew install fastlane`) and the store credentials configured in the git-ignored
`.env` at the repository root, see [`.env.example`](../.env.example). The TestFlight group names are
read from there and are not printed. `--only android|ios` uploads to one store, for example to retry.
It never submits a build for review or to a production or external track.

Between steps 2 and 3, run `cargo tauri icon` from `unime/src-tauri` if the icon source changed.
Signing must be set up first: `keystore.properties` for Android, the certificate and a manually
selected provisioning profile for iOS. See [the root README](../README.md#release-a-new-version) for
the full process, including everything after the upload.

The shell scripts resolve paths from their own location, so they can be run from any directory.
