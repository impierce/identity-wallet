mod command;
mod state;

use tracing_subscriber;

use command::handle_action;
use state::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![handle_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
