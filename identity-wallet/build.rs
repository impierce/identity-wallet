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

fn main() {
    // Re-run the build script if the .env file changes
    println!("cargo:rerun-if-changed=.env");

    let email_verification_service_host = dotenvy::var("EMAIL_VERIFICATION_SERVICE_HOST")
        .unwrap_or_else(|_| "https://email-verification.impierce.com".to_string());
    let email_verification_service_api_key =
        dotenvy::var("EMAIL_VERIFICATION_SERVICE_API_KEY").unwrap_or_else(|_| "".to_string());

    println!(
        "cargo:rustc-env=EMAIL_VERIFICATION_SERVICE_HOST={}",
        email_verification_service_host
    );
    println!(
        "cargo:rustc-env=EMAIL_VERIFICATION_SERVICE_API_KEY={}",
        email_verification_service_api_key
    );

    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let manifest = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("windows-test-manifest.xml");
        std::fs::write(&manifest, WINDOWS_TEST_MANIFEST).unwrap();
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }
}
