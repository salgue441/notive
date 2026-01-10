// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let start_minimized = std::env::args().any(|arg| arg == "--minimized");
    notive_lib::run(start_minimized);
}
