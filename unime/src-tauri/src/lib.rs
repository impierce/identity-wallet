use fern::colors::Color;
use identity_wallet::{
    persistence::{clear_assets_tmp_folder, initialize_storage},
    state::AppStateContainer,
};
use log::{info, LevelFilter};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, Target, TargetKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tauri_command::handle_action])
        .setup(move |app| {
            info!("setting up tauri app");
            initialize_storage(app.handle()).ok();
            clear_assets_tmp_folder().ok();
            dotenvy::dotenv().ok();

            #[cfg(mobile)]
            {
                app.handle().plugin(tauri_plugin_barcode_scanner::init())?;
            }
            Ok(())
        })
        .manage(AppStateContainer(Default::default()))
        .plugin(
            tauri_plugin_log::Builder::new()
                // .clear_targets()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                    // Target::new(TargetKind::LogDir {
                    //     file_name: Some("webview".into()),
                    // })
                    // .filter(|metadata| metadata.target() == WEBVIEW_TARGET),
                    // Target::new(TargetKind::LogDir {
                    //     file_name: Some("rust".into()),
                    // })
                    // .filter(|metadata| metadata.target() != WEBVIEW_TARGET),
                ])
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
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod tauri_command {
    use identity_wallet::state::{actions::Action, AppStateContainer};

    #[tauri::command]
    pub async fn handle_action<R: tauri::Runtime>(
        action: Action,
        app_handle: tauri::AppHandle<R>,
        container: tauri::State<'_, AppStateContainer>,
        window: tauri::Window<R>,
    ) -> Result<(), String> {
        identity_wallet::command::handle_action(action, app_handle, container, window).await
    }
}
