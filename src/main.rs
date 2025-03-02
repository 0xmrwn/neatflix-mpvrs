use log::{error, info};
use neatflix_mpvrs::{config, setup_logging};
use std::env;
use std::process::Command;

fn check_mpv_installed() -> bool {
    match Command::new("which").arg("mpv").output() {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

fn main() {
    // Initialize logging
    setup_logging();
    info!("neatflix-mpvrs v{}", neatflix_mpvrs::version());
    
    // Check if mpv is installed
    if !check_mpv_installed() {
        eprintln!("Error: mpv is not installed or not in your PATH.");
        eprintln!("Please install mpv before using neatflix-mpvrs.");
        eprintln!("On macOS, you can install it with: brew install mpv");
        std::process::exit(1);
    }
    
    // Initialize default configuration
    if let Err(e) = config::initialize_default_config() {
        error!("Failed to initialize configuration: {}", e);
        std::process::exit(1);
    }
    
    // Get media file from command line arguments or use a default
    let args: Vec<String> = env::args().collect();
    let media = if args.len() > 1 {
        &args[1]
    } else {
        println!("Usage: neatflix-mpvrs <media_file_or_url>");
        println!("No media file specified. Please provide a media file path or URL.");
        std::process::exit(1);
    };
    
    // Optional extra arguments to override defaults
    let extra_args: Vec<&str> = if args.len() > 2 {
        args[2..].iter().map(|s| s.as_str()).collect()
    } else {
        vec![]
    };
    
    info!("Playing media: {}", media);
    if let Err(e) = neatflix_mpvrs::spawn_mpv(media, &extra_args) {
        error!("Error launching video player: {}", e);
        std::process::exit(1);
    }
}
