mod clients;
mod command;
mod state;

use tauri::Manager;
use tracing_subscriber;

use command::execute_command;
use state::state::{AppState, StateStatus};

// Will be removed in future versions.
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .manage(AppState::new(
            Default::default(),
            Default::default(),
            Default::default(),
        ))
        .invoke_handler(tauri::generate_handler![greet, execute_command])
        .setup(|app| {
            // let mut store = StoreBuilder::new("path/to/store.bin".parse()?).build();
            // store.save(&app.handle());
            // let mut store = StoreBuilder::new("path/to/store.bin".parse()?).build();
            // store.load(app.app_handle())
            // store.insert("a".to_string(), json!("b"))
            // StoreBuilder::new("path/to/store.bin".parse()?).build();
            Ok(())
            // store.insert("a".to_string(), json!("b"))
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
