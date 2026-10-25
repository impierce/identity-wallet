# Vendored tauri-plugin-mcp-bridge 0.13.0

Upstream: https://github.com/hypothesi/mcp-server-tauri (crate `tauri-plugin-mcp-bridge` 0.13.0, MIT). Only the files
needed to build the crate are copied. Wired in through `[patch.crates-io]` in the workspace `Cargo.toml`.

The plugin only runs in debug desktop builds (see `unime/src-tauri/src/lib.rs`).

## Impierce changes

- `src/websocket.rs`: `reject_browser_origin` refuses WebSocket upgrades that carry an `Origin` header (HTTP 403).
  Upstream accepts any connection, so a web page open in the user's browser could connect to the bridge on
  `127.0.0.1` and execute JavaScript or IPC commands in the wallet. MCP clients (Node `ws`) send no `Origin`.

Drop this folder and the patch entry once an upstream release validates `Origin` and is compatible with our Tauri
version.
