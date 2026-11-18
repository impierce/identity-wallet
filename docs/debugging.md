# Debugging UniMe

## App state

When running UniMe via `pnpm tauri dev`, the current state of the app will be written to `debug/state.json`. It can be manually inspected or copied to be diffed later against a new state.

In case you prefer to inspect the state update directly on the console, you can set the environment variable `LOG_STATE_UPDATES_TO_CONSOLE=true` in the `unime/.env` file and then restart the app This only works in debug builds. To log full action, credential and protocol payloads as well, set `LOG_FULL_PAYLOADS=true` (also debug builds only).

## Trusting a test CA

UniMe trusts only the root certificates compiled into it (`webpki-roots`), on every platform. A debug build can trust
more: set `UNIME_EXTRA_CA_CERTS` to the path of a PEM file, and the CA certificates in it are trusted on top of the
built-in ones. This is for local test servers whose certificate comes from a test CA, such as a self-hosted conformance
suite, or for a TLS-inspecting proxy on the development machine.

```sh
UNIME_EXTRA_CA_CERTS=/absolute/path/to/test-ca.pem pnpm tauri dev
```

- The file can hold several certificates. Use an absolute path.
- Set the variable in the environment UniMe is started in, or in `unime/.env`, which is loaded at startup like the other
  debug settings.
- UniMe logs a warning with the number of certificates it added. A file it cannot read, or one without a certificate,
  is logged as an error and adds nothing.
- Release builds do not contain this: their roots stay the built-in ones, whatever the environment says.
- It applies to UniMe's own HTTP clients. Requests the openid4vc library sends with its own client are not covered.
