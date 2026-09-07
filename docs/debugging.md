# Debugging UniMe

## App state

When running UniMe via `pnpm tauri dev`, the current state of the app will be written to `debug/state.json`. It can be manually inspected or copied to be diffed later against a new state.

In case you prefer to inspect the state update directly on the console, you can set the environment variable `LOG_STATE_UPDATES_TO_CONSOLE=true` in the `unime/.env` file and then restart the app.
