#[cfg(target_os = "android")]
use iota_sdk::IotaClientBuilder;
#[cfg(target_os = "android")]
use jni::{
    objects::{JClass, JObject},
    JNIEnv,
};

#[cfg(not(feature = "test_utils"))]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    use identity_wallet::{
        persistence::{clear_assets_tmp_folder, initialize_storage},
        state::AppStateContainer,
    };
    use log::{info, LevelFilter};
    use tauri_plugin_log::{fern::colors::Color, fern::colors::ColoredLevelConfig, Target, TargetKind};

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, argv, _cwd| {
            info!("New app instance opened via deep link: {argv:?}");
        }));
    }

    builder
        .invoke_handler(tauri::generate_handler![tauri_command::handle_action])
        .plugin(tauri_plugin_deep_link::init())
        .setup(move |app| {
            info!("setting up tauri app");
            initialize_storage(app.handle()).ok();
            clear_assets_tmp_folder().ok();
            dotenvy::dotenv().ok();
            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
                app.handle().plugin(tauri_plugin_biometric::init())?;
                app.handle().plugin(tauri_plugin_keystore::init())?;
            }
            Ok(())
        })
        .manage(AppStateContainer(Default::default()))
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([Target::new(TargetKind::Stdout), Target::new(TargetKind::Webview)])
                .level(LevelFilter::Info)
                .level_for("unime", LevelFilter::Debug)
                .level_for("identity_wallet", LevelFilter::Debug)
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
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
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
