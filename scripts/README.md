# Release scripts

Run these in order to cut a release. Each one is independent: none of them calls another, and none of
them publishes anything.

| #   | Script                                             | What it does                                                                                                                                       |
| --- | -------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1   | `pnpm version:bump <version>` (`bump-version.mjs`) | Replaces the current version in the nine authoritative locations. `--dry-run` reports the targets without writing.                                 |
| 2   | `override-generated-mobile-files.sh`               | Overwrites Tauri's generated, git-ignored mobile projects (`unime/src-tauri/gen`) with the tracked customizations in `unime/src-tauri/gen-static`. |
| 3   | `build-mobile-releases.sh`                         | Builds both signed release bundles via `pnpm tauri android build` and `pnpm tauri ios build`. macOS only.                                          |
| 4   | `copy-release-artifacts.sh`                        | Collects the `.aab` and `.ipa` into the git-ignored `out/` folder at the repository root.                                                          |

Between steps 2 and 3, run `cargo tauri icon` from `unime/src-tauri` if the icon source changed.
Signing must be set up first: `keystore.properties` for Android, the certificate and a manually
selected provisioning profile for iOS. See [the root README](../README.md#release-a-new-version) for
the full process, including everything after the upload.

The shell scripts resolve paths from their own location, so they can be run from any directory.
