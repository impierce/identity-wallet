mod command;
mod state;

use command::handle_action;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[allow(unused_variables)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            #[cfg(mobile)]
            app.handle()
                .plugin(tauri_plugin_camera::init())
                .expect("error initializing camera plugin");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![handle_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
