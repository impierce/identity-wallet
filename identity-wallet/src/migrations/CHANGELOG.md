# App State and Data Migrations

Since UniMe is not semantically versioned, the following overview of migrations can be helpful for debugging and testing backwards compatibility.

| UniMe Version | `AppState` Version | Git commit | Description                                |
| ------------- | -----------------: | ---------- | ------------------------------------------ |
| `0.8.2`       |                `1` | `ae1b555`  | Initial version, introduce `version` field |

In order to get hold of a serialized version of the `AppState` at a specific time, simply checkout the source code at the corresponding version or git commit hash, load Ferris' profile and inspect the `state.json` in the data dir specific to your system.
