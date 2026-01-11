// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

const VERSION: &str = env!("CARGO_PKG_VERSION");
const APP_NAME: &str = "Notive";

fn print_version() {
    println!("{} {}", APP_NAME, VERSION);
}

fn print_help() {
    println!("{} - A high-performance Notion desktop wrapper for Linux", APP_NAME);
    println!();
    println!("Usage: notive [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --version     Show version information");
    println!("  --help        Show this help message");
    println!("  --minimized   Start minimized to tray");
    println!();
    println!("Keyboard Shortcuts:");
    println!("  Ctrl+Shift+N  Toggle window visibility");
    println!("  Ctrl+Shift+C  Quick capture");
    println!("  Ctrl+R        Reload page");
    println!("  Ctrl+=        Zoom in");
    println!("  Ctrl+-        Zoom out");
    println!("  Ctrl+0        Reset zoom");
    println!("  F11           Toggle fullscreen");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Handle --version
    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        print_version();
        return;
    }
    
    // Handle --help
    if args.iter().any(|arg| arg == "--help" || arg == "-h") {
        print_help();
        return;
    }
    
    // Check for --minimized flag
    let start_minimized = args.iter().any(|arg| arg == "--minimized");
    
    notive_lib::run(start_minimized);
}
