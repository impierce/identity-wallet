#[cfg(target_os = "android")]
use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};

#[cfg(not(feature = "test_utils"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use identity_wallet::{
        persistence::{clear_assets_tmp_folder, initialize_storage, ASSETS_DIR},
        state::AppStateContainer,
    };
    use log::{info, LevelFilter};
    use tauri::Manager;
    use tauri_plugin_log::{fern::colors::Color, fern::colors::ColoredLevelConfig, Target, TargetKind};

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
            info!("New app instance opened via deep link: {argv:?}");
        }));
    }

    // Lets the Tauri MCP server drive the UI in dev builds; localhost only.
    #[cfg(all(desktop, debug_assertions))]
    {
        builder = builder.plugin(
            tauri_plugin_mcp_bridge::Builder::new()
                .bind_address("127.0.0.1")
                .base_port(
                    std::env::var("UNIME_MCP_PORT")
                        .ok()
                        .and_then(|port| port.parse().ok())
                        .unwrap_or(9223),
                )
                .build(),
        );
    }

    // Registered on the builder: from `setup`, `AppHandle::plugin` holds the plugin lock while the Android
    // plugin waits for the main thread, which can be blocked on that lock in the WebView's navigation hook.
    #[cfg(mobile)]
    {
        builder = builder
            .plugin(tauri_plugin_barcode_scanner::init())
            .plugin(tauri_plugin_biometric::init())
            .plugin(tauri_plugin_keystore::init());
    }

    builder
        .invoke_handler(tauri::generate_handler![tauri_command::handle_action])
        .setup(move |app| {
            info!("setting up tauri app");
            let storage_initialized = initialize_storage(app.handle()).is_ok();
            clear_assets_tmp_folder().ok();
            // The asset protocol scope from the config does not match an assets folder that did not exist yet at
            // startup (e.g. on first launch), so allow the folder again now that it exists. Without initialized
            // storage, `ASSETS_DIR` is empty and would allow every path.
            if storage_initialized {
                let assets_dir = ASSETS_DIR.lock().unwrap().clone();
                app.asset_protocol_scope().allow_directory(&assets_dir, true)?;
            }
            dotenvy::dotenv().ok();
            Ok(())
        })
        .manage(AppStateContainer(Default::default()))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([Target::new(TargetKind::Stdout), Target::new(TargetKind::Webview)])
                .level(LevelFilter::Info)
                .level_for("unime", LevelFilter::Debug)
                .level_for("identity_wallet", LevelFilter::Debug)
                .level_for("oid4vc", LevelFilter::Debug)
                .level_for("oid4vc_core", LevelFilter::Debug)
                .level_for("oid4vci", LevelFilter::Debug)
                .level_for("oid4vp", LevelFilter::Debug)
                .level_for("siopv2", LevelFilter::Debug)
                .level_for("oid4vc_manager", LevelFilter::Debug)
                .with_colors(
                    ColoredLevelConfig::new()
                        .trace(Color::White)
                        .debug(Color::Cyan)
                        .info(Color::Green)
                        .warn(Color::Yellow)
                        .error(Color::Red),
                )
                .build(),
        )
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod tauri_command {
    use identity_wallet::{
        command::Runtime,
        state::{actions::Action, AppStateContainer},
    };

    #[tauri::command]
    pub async fn handle_action(
        action: Action,
        app_handle: tauri::AppHandle<Runtime>,
        container: tauri::State<'_, AppStateContainer>,
        window: tauri::Window<Runtime>,
    ) -> Result<(), String> {
        identity_wallet::command::handle_action(action, app_handle, container, window).await
    }
}

/// Called by `MainActivity.kt` during Android startup to initialize the Android
/// platform certificate verifier environment.
///
/// Note: While the wallet application uses `webpki-roots` for its HTTP clients,
/// transitive dependencies (such as `jsonrpsee` via `iota-sdk`) require this
/// initialization when running on Android.
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_impierce_identity_1wallet_MainActivity_java_1init(
    mut env: JNIEnv,
    _class: JClass,
    context: JObject,
) {
    rustls_platform_verifier::android::init_hosted(&mut env, context)
        .expect("Failed to initialize Android platform verifier");
}
