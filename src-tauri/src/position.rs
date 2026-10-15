use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct MonitorBounds {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub fn get_mouse_location() -> Option<(i32, i32)> {
    if let Ok(output) = std::process::Command::new("xdotool")
        .arg("getmouselocation")
        .output()
    {
        if output.status.success() {
            let s = String::from_utf8_lossy(&output.stdout);
            let mut x = None;
            let mut y = None;
            for part in s.split_whitespace() {
                if let Some(val) = part.strip_prefix("x:") {
                    x = val.parse::<i32>().ok();
                } else if let Some(val) = part.strip_prefix("y:") {
                    y = val.parse::<i32>().ok();
                }
            }
            if let (Some(x), Some(y)) = (x, y) {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn get_monitors_from_xrandr() -> Vec<MonitorBounds> {
    let mut monitors = Vec::new();
    if let Ok(output) = std::process::Command::new("xrandr")
        .arg("--listmonitors")
        .output()
    {
        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            let re = Regex::new(r"(\d+)(?:/\d+)?x(\d+)(?:/\d+)?\+(\d+)\+(\d+)").unwrap();
            for line in text.lines().skip(1) {
                if let Some(caps) = re.captures(line) {
                    let w: i32 = caps.get(1).and_then(|m| m.as_str().parse().ok()).unwrap_or(1920);
                    let h: i32 = caps.get(2).and_then(|m| m.as_str().parse().ok()).unwrap_or(1080);
                    let x: i32 = caps.get(3).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                    let y: i32 = caps.get(4).and_then(|m| m.as_str().parse().ok()).unwrap_or(0);
                    monitors.push(MonitorBounds { x, y, w, h });
                }
            }
        }
    }
    monitors
}

pub fn position_window_near_cursor(window: &WebviewWindow) {
    let _win_size = window.outer_size().unwrap_or(PhysicalSize { width: 528, height: 808 });

    let (cursor_x, cursor_y) = match get_mouse_location() {
        Some(pos) => pos,
        None => {
            let _ = window.center();
            return;
        }
    };

    // Query monitors from xrandr or fallback to available_monitors
    let mut monitors = get_monitors_from_xrandr();
    if monitors.is_empty() {
        if let Ok(tauri_monitors) = window.available_monitors() {
            for mon in tauri_monitors {
                let pos = mon.position();
                let sz = mon.size();
                monitors.push(MonitorBounds {
                    x: pos.x,
                    y: pos.y,
                    w: sz.width as i32,
                    h: sz.height as i32,
                });
            }
        }
    }

    // Find monitor that contains cursor (both horizontally and vertically)
    let active_mon = monitors.iter().find(|m| {
        cursor_x >= m.x && cursor_x < m.x + m.w && cursor_y >= m.y && cursor_y < m.y + m.h
    }).or_else(|| {
        monitors.iter().find(|m| cursor_x >= m.x && cursor_x < m.x + m.w)
    }).or_else(|| monitors.first());

    let (mon_x, mon_y, mon_w, mon_h) = if let Some(m) = active_mon {
        (m.x, m.y, m.w, m.h)
    } else {
        (0, 0, 1920, 1080)
    };

    // The Tauri window has a 44px transparent margin around the 440x720 content shell for shadow rendering.
    // So the visible content card starts at (window_x + 44, window_y + 44).
    const SHADOW_MARGIN: i32 = 44;
    const CARD_W: i32 = 440;
    const CARD_H: i32 = 720;

    let mut target_x = cursor_x + 10 - SHADOW_MARGIN;
    let mut target_y = cursor_y + 15 - SHADOW_MARGIN;

    // Flip to left of cursor if it overflows monitor on the right
    if target_x + SHADOW_MARGIN + CARD_W > mon_x + mon_w {
        target_x = cursor_x - CARD_W - 10 - SHADOW_MARGIN;
    }
    // Clamp to left
    if target_x + SHADOW_MARGIN < mon_x {
        target_x = mon_x - SHADOW_MARGIN;
    }

    // Flip above cursor if it overflows monitor at the bottom
    if target_y + SHADOW_MARGIN + CARD_H > mon_y + mon_h {
        target_y = cursor_y - CARD_H - 15 - SHADOW_MARGIN;
    }
    // Clamp to top
    if target_y + SHADOW_MARGIN < mon_y {
        target_y = mon_y - SHADOW_MARGIN;
    }

    // Move via Tauri API
    let _ = window.set_position(tauri::Position::Physical(PhysicalPosition {
        x: target_x,
        y: target_y,
    }));

    // Also move via xdotool directly to guarantee instant X11 / Xwayland positioning
    let target_x_str = target_x.to_string();
    let target_y_str = target_y.to_string();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(25));
        if let Ok(output) = std::process::Command::new("xdotool")
            .args(["search", "--name", "Zoma Clipboard"])
            .output()
        {
            let s = String::from_utf8_lossy(&output.stdout);
            for id in s.lines() {
                let id = id.trim();
                if !id.is_empty() {
                    let _ = std::process::Command::new("xdotool")
                        .args(["windowmove", id, &target_x_str, &target_y_str])
                        .output();
                }
            }
        }
    });
}
