# App State and Data Migrations

Since UniMe is not semantically versioned, the following overview of migrations can be helpful for debugging and testing backwards compatibility.

| UniMe Version | `AppState` Version | Git commit  | Description                                                                                                   |
| ------------- | -----------------: | ----------- | ------------------------------------------------------------------------------------------------------------- |
| `0.8.2`       |                `1` | `ae1b555`   | Initial version, introduce `version` field                                                                    |
| unreleased    |                `2` | (prototype) | Replace `did` of connections with `key` and `proofs`, `client_id` of client metadata with `proofs` (ADR 0004) |

In order to get hold of a serialized version of the `AppState` at a specific time, simply checkout the source code at the corresponding version or git commit hash, load Ferris' profile and inspect the `state.json` in the data dir specific to your system.
On Linux, this is usually in `~/.local/share/com.impierce.identity-wallet/` and on macOS in `~/Library/Application Support/com.impierce.identity-wallet/`.
