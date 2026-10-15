use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager, State};
use crate::clipboard::ClipboardManager;
use crate::db::Database;
use crate::models::{AppSettings, Clip};
use crate::position::position_window_near_cursor;

pub struct AppState {
    pub db: Arc<Mutex<Database>>,
    pub clipboard_mgr: Arc<ClipboardManager>,
    pub previous_active_window: Arc<Mutex<Option<String>>>,
}

pub fn capture_active_window() -> Option<String> {
    if let Ok(output) = std::process::Command::new("xdotool")
        .args(["getactivewindow"])
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !s.is_empty() {
                return Some(s);
            }
        }
    }
    None
}

#[tauri::command]
pub fn get_clips(
    state: State<AppState>,
    search: Option<String>,
    category: Option<String>,
) -> Result<Vec<Clip>, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_clips(search.as_deref(), category.as_deref())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn toggle_pin(state: State<AppState>, id: String) -> Result<bool, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.toggle_pin(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_clip(state: State<AppState>, id: String) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.delete_clip(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn clear_history(state: State<AppState>) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.clear_unpinned().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn copy_clip(app_handle: AppHandle, state: State<AppState>, id: String) -> Result<(), String> {
    let clip = {
        let db = state.db.lock().map_err(|e| e.to_string())?;
        let clips = db.get_clips(None, None).map_err(|e| e.to_string())?;
        clips.into_iter().find(|c| c.id == id)
    };

    if let Some(c) = clip {
        // Move to top of history and deduplicate
        {
            let db = state.db.lock().map_err(|e| e.to_string())?;
            let _ = db.touch_and_deduplicate(&c.id);
        }

        if c.content_type == "image" {
            if let Some(path_str) = &c.text_content {
                state.clipboard_mgr.copy_image_from_file(path_str, c.image_hash)?;
            }
        } else if let Some(t) = &c.text_content {
            state.clipboard_mgr.copy_text(t)?;
        }

        let _ = app_handle.emit("clipboard-updated", &c);
    }
    Ok(())
}

#[tauri::command]
pub fn paste_clip(app_handle: AppHandle, state: State<AppState>, id: String) -> Result<(), String> {
    // 1. Copy clip (text or image) to system clipboard, touch and deduplicate
    copy_clip(app_handle.clone(), state.clone(), id)?;

    // 2. Hide window so user returns to previous application
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }

    // 3. Get recorded target window ID
    let prev_win = state.previous_active_window.lock().ok().and_then(|g| g.clone());

    // 4. Automatically paste into target window via uinput (ydotool) / X11 (xdotool)
    std::thread::spawn(move || {
        // Small delay to allow window manager to hide Zoma and prepare focus
        std::thread::sleep(std::time::Duration::from_millis(80));

        // If we know the previous window, explicitly refocus it
        if let Some(ref win_id) = prev_win {
            let _ = std::process::Command::new("xdotool")
                .args(["windowactivate", "--sync", win_id])
                .output();
            std::thread::sleep(std::time::Duration::from_millis(60));
        }

        // Primary: Inject Ctrl+V via ydotool (kernel uinput -> Mutter -> all Wayland & X11 apps)
        let ydo_res = std::process::Command::new("ydotool")
            .args(["key", "ctrl+v"])
            .status();

        // Fallback: Also try xdotool if ydotool did not succeed
        if ydo_res.is_err() || !ydo_res.unwrap().success() {
            let _ = std::process::Command::new("xdotool")
                .args(["key", "--clearmodifiers", "ctrl+v"])
                .output();
        }
    });

    Ok(())
}

#[tauri::command]
pub fn get_settings(state: State<AppState>) -> Result<AppSettings, String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn save_settings(state: State<AppState>, settings: AppSettings) -> Result<(), String> {
    let db = state.db.lock().map_err(|e| e.to_string())?;
    db.save_settings(&settings).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn force_focus_window() {
    std::thread::spawn(move || {
        for delay in [20, 50, 100] {
            std::thread::sleep(std::time::Duration::from_millis(delay));
            if let Ok(output) = std::process::Command::new("xdotool")
                .args(["search", "--name", "Zoma Clipboard"])
                .output()
            {
                let s = String::from_utf8_lossy(&output.stdout);
                for id in s.lines() {
                    let id = id.trim();
                    if !id.is_empty() {
                        let _ = std::process::Command::new("xdotool")
                            .args(["windowactivate", id])
                            .output();
                    }
                }
            }
        }
    });
}

#[tauri::command]
pub fn hide_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn show_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(state) = app_handle.try_state::<AppState>() {
        if let Some(win_id) = capture_active_window() {
            if let Ok(mut prev) = state.previous_active_window.lock() {
                *prev = Some(win_id);
            }
        }
    }

    if let Some(window) = app_handle.get_webview_window("main") {
        position_window_near_cursor(&window);
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
        position_window_near_cursor(&window);
        force_focus_window();
        let _ = app_handle.emit("focus-search", ());
    }
    Ok(())
}

#[tauri::command]
pub fn toggle_window(app_handle: AppHandle) -> Result<(), String> {
    if let Some(window) = app_handle.get_webview_window("main") {
        if let Ok(is_visible) = window.is_visible() {
            if is_visible {
                let _ = window.hide();
            } else {
                if let Some(state) = app_handle.try_state::<AppState>() {
                    if let Some(win_id) = capture_active_window() {
                        if let Ok(mut prev) = state.previous_active_window.lock() {
                            *prev = Some(win_id);
                        }
                    }
                }
                position_window_near_cursor(&window);
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
                position_window_near_cursor(&window);
                force_focus_window();
                let _ = app_handle.emit("focus-search", ());
            }
        }
    }
    Ok(())
}
