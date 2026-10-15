// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Write;
use std::os::unix::net::UnixStream;

fn main() {
    // Force X11 backend so GTK window can be accurately positioned across dual monitors
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }

    let args: Vec<String> = std::env::args().collect();
    let sock_path = zoma_clipboard_lib::get_socket_path();

    // If another instance is already running, send command with target window ID and exit immediately
    if let Ok(mut stream) = UnixStream::connect(&sock_path) {
        let win_id = std::process::Command::new("xdotool")
            .arg("getactivewindow")
            .output()
            .ok()
            .and_then(|o| {
                if o.status.success() {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if !s.is_empty() {
                        Some(s)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let action = if args.iter().any(|a| a == "--hide") {
            "hide"
        } else if args.iter().any(|a| a == "--show") {
            "show"
        } else {
            "toggle"
        };
        let msg = format!("{} {}\n", action, win_id);
        let _ = stream.write_all(msg.as_bytes());
        return;
    }

    zoma_clipboard_lib::run();
}
