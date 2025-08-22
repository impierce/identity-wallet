// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(not(feature = "test_utils"))]
    #[cfg(desktop)]
    unime::run();
}
