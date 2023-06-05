// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(desktop)]
    tauri_plugin_deep_link::prepare("com.impierce.dev");

    #[cfg(desktop)]
    identity_wallet::run();
}
