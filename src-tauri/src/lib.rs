pub mod models;
pub mod db;
pub mod clipboard;
pub mod commands;
pub mod position;

use std::io::Read;
use std::os::unix::net::UnixListener;
use std::sync::{Arc, Mutex};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use crate::commands::AppState;
use crate::db::Database;
use crate::clipboard::ClipboardManager;

pub fn get_socket_path() -> std::path::PathBuf {
    if let Ok(runtime_dir) = std::env::var("XDG_RUNTIME_DIR") {
        std::path::PathBuf::from(runtime_dir).join("zoma-clipboard.sock")
    } else {
        std::path::PathBuf::from("/tmp/zoma-clipboard.sock")
    }
}

fn start_ipc_server(app_handle: AppHandle) {
    let sock_path = get_socket_path();
    let _ = std::fs::remove_file(&sock_path);

    std::thread::spawn(move || {
        let listener = match UnixListener::bind(&sock_path) {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to bind IPC socket: {}", e);
                return;
            }
        };

        for stream in listener.incoming() {
            if let Ok(mut s) = stream {
                let mut buf = [0u8; 64];
                if let Ok(n) = s.read(&mut buf) {
                    let cmd = String::from_utf8_lossy(&buf[..n]);
                    let mut parts = cmd.split_whitespace();
                    let action = parts.next().unwrap_or("toggle");
                    let win_id = parts.next().map(|s| s.to_string());

                    if let Some(state) = app_handle.try_state::<AppState>() {
                        if let Some(w) = win_id {
                            if let Ok(mut prev) = state.previous_active_window.lock() {
                                *prev = Some(w);
                            }
                        }
                    }

                    if action == "hide" {
                        let _ = commands::hide_window(app_handle.clone());
                    } else if action == "show" {
                        let _ = commands::show_window(app_handle.clone());
                    } else {
                        // default: toggle
                        let _ = commands::toggle_window(app_handle.clone());
                    }
                }
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Locate database in app local data dir
            let app_data_dir = app.path().app_data_dir().unwrap_or_else(|_| {
                std::path::PathBuf::from("./data")
            });
            let db_path = app_data_dir.join("zoma_clipboard.db");
            let database = Database::new(db_path).expect("Failed to initialize database");
            let db_arc = Arc::new(Mutex::new(database));

            // Start clipboard listener
            let clipboard_mgr = Arc::new(ClipboardManager::new(Arc::clone(&db_arc), app_data_dir.clone()));
            clipboard_mgr.start_monitor(app_handle.clone());

            // Manage state
            let previous_active_window = Arc::new(Mutex::new(None));
            app.manage(AppState {
                db: Arc::clone(&db_arc),
                clipboard_mgr,
                previous_active_window,
            });

            // Start IPC listener for single-instance toggle
            start_ipc_server(app_handle.clone());

            // Configure main window: skip taskbar / dock
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_skip_taskbar(true);
            }

            // Setup Tray Menu
            let toggle_item = MenuItem::with_id(app, "toggle", "Mostrar / Ocultar Zoma Clipboard", true, None::<&str>)?;
            let clear_item = MenuItem::with_id(app, "clear", "Limpiar historial reciente", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Salir de Zoma Clipboard", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&toggle_item, &clear_item, &quit_item])?;

            let _tray = TrayIconBuilder::new()
                .menu(&tray_menu)
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Zoma Clipboard")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "toggle" => {
                            let _ = commands::toggle_window(app.clone());
                        }
                        "clear" => {
                            let state = app.state::<AppState>();
                            let _ = commands::clear_history(state);
                        }
                        "quit" => {
                            let sock = get_socket_path();
                            let _ = std::fs::remove_file(sock);
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        let _ = commands::toggle_window(app.clone());
                    }
                })
                .build(app)?;

            // Register global shortcuts: Super+V, Alt+V, and Ctrl+Shift+V
            let shortcuts_to_try = ["Super+V", "Alt+V", "Ctrl+Shift+V"];
            for sc in shortcuts_to_try {
                let app_h = app_handle.clone();
                if let Ok(shortcut) = sc.parse::<Shortcut>() {
                    let _ = app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            let _ = commands::toggle_window(app_h.clone());
                        }
                    });
                }
            }

            // If launched without --background or --daemon, show near cursor on start
            let is_background = std::env::args().any(|a| a == "--background" || a == "--daemon");
            if !is_background {
                let app_h = app_handle.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(150));
                    let _ = commands::show_window(app_h);
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_clips,
            commands::toggle_pin,
            commands::delete_clip,
            commands::clear_history,
            commands::copy_clip,
            commands::paste_clip,
            commands::get_settings,
            commands::save_settings,
            commands::hide_window,
            commands::show_window,
            commands::toggle_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running zoma clipboard application");
}
