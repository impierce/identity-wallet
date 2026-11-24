#![allow(clippy::uninlined_format_args)]

/// Test executables load Tauri, which needs the Common Controls v6 dependency from this manifest on Windows, or they
/// fail with `STATUS_ENTRYPOINT_NOT_FOUND`.
const WINDOWS_TEST_MANIFEST: &str = r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" />
    </dependentAssembly>
  </dependency>
</assembly>
"#;

const EMAIL_VERIFICATION_SERVICE_HOST: &str = "EMAIL_VERIFICATION_SERVICE_HOST";
const EMAIL_VERIFICATION_SERVICE_API_KEY: &str = "EMAIL_VERIFICATION_SERVICE_API_KEY";

fn main() {
    // Re-run the build script if the .env file changes, or the variables are set in the environment instead
    println!("cargo:rerun-if-changed=.env");
    println!("cargo:rerun-if-env-changed={}", EMAIL_VERIFICATION_SERVICE_HOST);
    println!("cargo:rerun-if-env-changed={}", EMAIL_VERIFICATION_SERVICE_API_KEY);

    let host = non_empty_var(EMAIL_VERIFICATION_SERVICE_HOST);
    let api_key = non_empty_var(EMAIL_VERIFICATION_SERVICE_API_KEY);

    // A release build without the email verification service compiles fine but cannot verify an email address, and
    // only fails at runtime, as "couldn't reach the email verification service". Store builds are release builds:
    // `PROFILE` is `release` for them, also for a custom profile that inherits from `release`.
    if std::env::var("PROFILE").as_deref() == Ok("release") {
        let mut problems = Vec::new();
        match &host {
            None => problems.push(format!("{} is not set", EMAIL_VERIFICATION_SERVICE_HOST)),
            Some(host) if !host.starts_with("https://") => problems.push(format!(
                "{} does not start with `https://`",
                EMAIL_VERIFICATION_SERVICE_HOST
            )),
            Some(_) => {}
        }
        if api_key.is_none() {
            problems.push(format!("{} is not set", EMAIL_VERIFICATION_SERVICE_API_KEY));
        }
        if !problems.is_empty() {
            eprintln!(
                "error: a release build needs the email verification service: {}. Set both variables in \
                 `identity-wallet/.env` (see `.env.example`) or in the environment.",
                problems.join(", ")
            );
            std::process::exit(1);
        }
    }

    // Debug builds may leave them out, for example to run the tests. Email verification then fails at runtime.
    for (name, value) in [
        (EMAIL_VERIFICATION_SERVICE_HOST, &host),
        (EMAIL_VERIFICATION_SERVICE_API_KEY, &api_key),
    ] {
        if value.is_none() {
            println!(
                "cargo:warning={} is not set in `identity-wallet/.env` or the environment; email verification will not work",
                name
            );
        }
    }

    println!(
        "cargo:rustc-env={}={}",
        EMAIL_VERIFICATION_SERVICE_HOST,
        host.unwrap_or_default()
    );
    println!(
        "cargo:rustc-env={}={}",
        EMAIL_VERIFICATION_SERVICE_API_KEY,
        api_key.unwrap_or_default()
    );

    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let manifest = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("windows-test-manifest.xml");
        std::fs::write(&manifest, WINDOWS_TEST_MANIFEST).unwrap();
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }
}

/// The value of `name` from the environment or `.env`, or `None` when it is unset or empty. The `.env.example` sets
/// both variables to `""`, so an unedited copy counts as unset.
fn non_empty_var(name: &str) -> Option<String> {
    dotenvy::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
