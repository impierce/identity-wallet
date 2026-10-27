/// The app manifest `tauri-build` uses by default. Its Common Controls v6 dependency is needed by every executable that
/// loads Tauri on Windows, including test executables, which otherwise fail with `STATUS_ENTRYPOINT_NOT_FOUND`.
const WINDOWS_APP_MANIFEST: &str = r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" />
    </dependentAssembly>
  </dependency>
</assembly>
"#;

fn main() {
    let mut windows = tauri_build::WindowsAttributes::new();

    // `tauri-build` only embeds the manifest into the binary, so it is linked into all executables instead.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        windows = tauri_build::WindowsAttributes::new_without_app_manifest();

        let manifest = std::path::Path::new(&std::env::var("OUT_DIR").unwrap()).join("windows-app-manifest.xml");
        std::fs::write(&manifest, WINDOWS_APP_MANIFEST).unwrap();
        println!("cargo:rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg=/MANIFESTINPUT:{}", manifest.display());
    }

    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
        .expect("failed to run build script");
}
