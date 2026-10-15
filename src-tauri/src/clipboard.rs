use std::sync::{Arc, Mutex};
use std::time::Duration;
use arboard::{Clipboard, ImageData};
use chrono::Utc;
use regex::Regex;
use tauri::{AppHandle, Emitter};
use crate::db::Database;
use crate::models::Clip;
use std::io::Cursor;
use image::{ImageBuffer, Rgba};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use std::path::PathBuf;

pub struct ClipboardManager {
    db: Arc<Mutex<Database>>,
    app_data_dir: PathBuf,
    last_text: Arc<Mutex<Option<String>>>,
    last_img_hash: Arc<Mutex<Option<u64>>>,
}

impl ClipboardManager {
    pub fn new(db: Arc<Mutex<Database>>, app_data_dir: PathBuf) -> Self {
        let images_dir = app_data_dir.join("images");
        let _ = std::fs::create_dir_all(&images_dir);

        Self {
            db,
            app_data_dir,
            last_text: Arc::new(Mutex::new(None)),
            last_img_hash: Arc::new(Mutex::new(None)),
        }
    }

    pub fn start_monitor(&self, app_handle: AppHandle) {
        let db = Arc::clone(&self.db);
        let last_text = Arc::clone(&self.last_text);
        let last_img_hash = Arc::clone(&self.last_img_hash);
        let images_dir = self.app_data_dir.join("images");

        std::thread::spawn(move || {
            let mut clipboard = match Clipboard::new() {
                Ok(cb) => cb,
                Err(e) => {
                    eprintln!("Failed to initialize clipboard listener: {}", e);
                    return;
                }
            };

            let color_re = Regex::new(r"^#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{4}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$").unwrap();
            let url_re = Regex::new(r"^https?://[^\s]+$").unwrap();

            loop {
                std::thread::sleep(Duration::from_millis(500));

                // 1. Check for text
                if let Ok(text) = clipboard.get_text() {
                    let trimmed = text.trim();
                    if !trimmed.is_empty() {
                        let mut last = last_text.lock().unwrap();
                        if last.as_deref() != Some(trimmed) {
                            *last = Some(trimmed.to_string());
                            drop(last);
                            if let Ok(mut lh) = last_img_hash.lock() {
                                *lh = None;
                            }

                            let now = Utc::now().timestamp_millis();
                            let text_len = trimmed.len();

                            let (content_type, category, title, metadata, color_hex) = if color_re.is_match(trimmed) {
                                (
                                    "color".to_string(),
                                    "text".to_string(),
                                    Some(trimmed.to_string()),
                                    Some("Código de Color Hex".to_string()),
                                    Some(trimmed.to_string()),
                                )
                            } else if url_re.is_match(trimmed) {
                                (
                                    "url".to_string(),
                                    "url".to_string(),
                                    Some(trimmed.to_string()),
                                    Some("Enlace Web · Hace un momento".to_string()),
                                    None,
                                )
                            } else if is_code_snippet(trimmed) {
                                let lang = detect_language(trimmed);
                                (
                                    "code".to_string(),
                                    "code".to_string(),
                                    Some(format!("Fragmento de código ({})", lang)),
                                    Some(format!("{} · Hace un momento · ({} B)", lang, text_len)),
                                    None,
                                )
                            } else {
                                (
                                    "text".to_string(),
                                    "text".to_string(),
                                    Some(truncate_string(trimmed, 60)),
                                    Some(format!("Texto · Hace un momento · ({} B)", text_len)),
                                    None,
                                )
                            };

                            let id = format!("clip-{}", now);
                            let clip = Clip {
                                id,
                                content_type,
                                text_content: Some(trimmed.to_string()),
                                title,
                                metadata,
                                category,
                                color_hex,
                                thumbnail_data: None,
                                image_width: None,
                                image_height: None,
                                image_size_bytes: None,
                                image_hash: None,
                                is_pinned: false,
                                created_at: now,
                                last_used_at: now,
                            };

                            let db_guard = db.lock().unwrap();
                            if let Ok(Some(existing)) = db_guard.find_by_text(trimmed) {
                                let _ = db_guard.touch_and_deduplicate(&existing.id);
                                let _ = app_handle.emit("clipboard-updated", &existing);
                            } else {
                                let _ = db_guard.insert_clip(&clip);
                                let _ = app_handle.emit("clipboard-updated", &clip);
                            }
                        }
                    }
                }

                // 2. Check for image
                if let Ok(img) = clipboard.get_image() {
                    let hash = simple_image_hash(&img);
                    let mut last_hash = last_img_hash.lock().unwrap();
                    if *last_hash != Some(hash) {
                        *last_hash = Some(hash);
                        drop(last_hash);
                        if let Ok(mut lt) = last_text.lock() {
                            *lt = None;
                        }

                        let db_guard = db.lock().unwrap();
                        if let Ok(Some(existing)) = db_guard.find_by_image_hash(hash) {
                            let _ = db_guard.touch_and_deduplicate(&existing.id);
                            let _ = app_handle.emit("clipboard-updated", &existing);
                        } else {
                            let now = Utc::now().timestamp_millis();
                            let (w, h) = (img.width as u32, img.height as u32);
                            let bytes_len = img.bytes.len() as u64;

                            let clip_id = format!("clip-img-{}", now);
                            let img_filename = format!("{}.png", clip_id);
                            let full_img_path = images_dir.join(&img_filename);

                            // Save full resolution image as PNG to disk
                            if let Some(buf) = ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(w, h, img.bytes.to_vec()) {
                                let _ = buf.save(&full_img_path);
                            }

                            // Create compact thumbnail
                            let thumb_data = create_image_thumbnail(&img);

                            let clip = Clip {
                                id: clip_id,
                                content_type: "image".to_string(),
                                text_content: Some(full_img_path.to_string_lossy().to_string()),
                                title: Some(format!("Imagen_{}x{}.png", w, h)),
                                metadata: Some(format!("{} × {} px · Hace un momento", w, h)),
                                category: "image".to_string(),
                                color_hex: None,
                                thumbnail_data: thumb_data,
                                image_width: Some(w),
                                image_height: Some(h),
                                image_size_bytes: Some(bytes_len),
                                image_hash: Some(hash),
                                is_pinned: false,
                                created_at: now,
                                last_used_at: now,
                            };

                            let _ = db_guard.insert_clip(&clip);
                            let _ = app_handle.emit("clipboard-updated", &clip);
                        }
                    }
                }
            }
        });
    }

    pub fn copy_text(&self, text: &str) -> Result<(), String> {
        let trimmed = text.trim();
        {
            let mut last = self.last_text.lock().unwrap();
            *last = Some(trimmed.to_string());
            if let Ok(mut lh) = self.last_img_hash.lock() {
                *lh = None;
            }
        }

        // Arboard clipboard
        if let Ok(mut cb) = Clipboard::new() {
            let _ = cb.set_text(text);
        }

        // Wayland clipboard using wl-copy
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            let mut cmd = std::process::Command::new("wl-copy");
            cmd.stdin(std::process::Stdio::piped());
            if let Ok(mut child) = cmd.spawn() {
                if let Some(mut stdin) = child.stdin.take() {
                    let _ = std::io::Write::write_all(&mut stdin, text.as_bytes());
                }
                let _ = child.wait();
            }
        }

        // X11 clipboard using xclip
        let mut cmd_x = std::process::Command::new("xclip");
        cmd_x.args(["-selection", "clipboard"]);
        cmd_x.stdin(std::process::Stdio::piped());
        if let Ok(mut child) = cmd_x.spawn() {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = std::io::Write::write_all(&mut stdin, text.as_bytes());
            }
            let _ = child.wait();
        }

        Ok(())
    }

    pub fn copy_image_from_file(&self, path_str: &str, image_hash: Option<u64>) -> Result<(), String> {
        let path = std::path::Path::new(path_str);
        if !path.exists() {
            return Err("Image file does not exist".to_string());
        }

        // Register hash to avoid duplicate detection by monitor and clear text cache
        {
            let mut last = self.last_img_hash.lock().unwrap();
            *last = image_hash;
            if let Ok(mut lt) = self.last_text.lock() {
                *lt = None;
            }
        }

        // 1. Wayland clipboard using wl-copy
        if std::env::var("WAYLAND_DISPLAY").is_ok() {
            if let Ok(file) = std::fs::File::open(path) {
                let _ = std::process::Command::new("wl-copy")
                    .args(["-t", "image/png"])
                    .stdin(file)
                    .spawn();
            }
        }

        // 2. X11 clipboard using xclip
        let _ = std::process::Command::new("xclip")
            .args(["-selection", "clipboard", "-t", "image/png", "-i", path_str])
            .output();

        // 3. Arboard internal clipboard buffer
        if let Ok(dyn_img) = image::open(path) {
            let rgba = dyn_img.to_rgba8();
            let (w, h) = rgba.dimensions();
            if let Ok(mut cb) = Clipboard::new() {
                let img_data = ImageData {
                    width: w as usize,
                    height: h as usize,
                    bytes: std::borrow::Cow::Borrowed(&rgba),
                };
                let _ = cb.set_image(img_data);
            }
        }

        Ok(())
    }
}

fn truncate_string(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max).collect();
        format!("{}...", truncated)
    }
}

fn is_code_snippet(s: &str) -> bool {
    let s_trim = s.trim();
    if (s_trim.starts_with('{') && s_trim.ends_with('}')) || (s_trim.starts_with('[') && s_trim.ends_with(']')) {
        return true;
    }

    let code_keywords = [
        "function", "const ", "let ", "var ", "import ", "export ", "class ", "def ",
        "fn ", "pub fn", "impl ", "struct ", "enum ", "return ", "console.log",
        "sudo ", "apt ", "cargo ", "npm ", "git ", "docker ", "meson ", "ninja ",
        "ssh ", "curl ", "wget ", "chmod ", "chown ", "systemctl ", "echo "
    ];

    let lines: Vec<&str> = s_trim.lines().collect();
    if lines.len() > 1 && (s.contains("    ") || s.contains('\t') || s.contains("};")) {
        return true;
    }

    for kw in code_keywords {
        if s_trim.starts_with(kw) || s.contains(kw) {
            return true;
        }
    }

    false
}

fn detect_language(s: &str) -> &'static str {
    let s_trim = s.trim();
    if (s_trim.starts_with('{') && s_trim.ends_with('}')) || (s_trim.starts_with('[') && s_trim.ends_with(']')) {
        return "JSON";
    }
    if s_trim.starts_with("ssh ") || s_trim.starts_with("sudo ") || s_trim.starts_with("meson ") || s_trim.starts_with("git ") {
        return "Bash";
    }
    if s.contains("fn ") || s.contains("let mut ") || s.contains("impl ") {
        return "Rust";
    }
    if s.contains("const ") || s.contains("function ") || s.contains("=>") {
        return "JavaScript";
    }
    if s.contains("def ") || s.contains("import numpy") || s.contains("if __name__") {
        return "Python";
    }
    "Código"
}

fn simple_image_hash(img: &ImageData) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    let step = (img.bytes.len() / 64).max(1);
    for i in (0..img.bytes.len()).step_by(step) {
        hash ^= img.bytes[i] as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^ ((img.width as u64) << 32) ^ (img.height as u64)
}

fn create_image_thumbnail(img: &ImageData) -> Option<String> {
    let width = img.width as u32;
    let height = img.height as u32;
    if width == 0 || height == 0 {
        return None;
    }

    let raw_rgba = img.bytes.to_vec();
    let image_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = match ImageBuffer::from_raw(width, height, raw_rgba) {
        Some(buf) => buf,
        None => return None,
    };

    let target_w = 160;
    let target_h = (height * target_w) / width;
    let resized = image::imageops::resize(&image_buf, target_w, target_h.max(1), image::imageops::FilterType::Nearest);

    let mut cursor = Cursor::new(Vec::new());
    if resized.write_to(&mut cursor, image::ImageFormat::Png).is_ok() {
        let b64 = BASE64.encode(cursor.into_inner());
        Some(format!("data:image/png;base64,{}", b64))
    } else {
        None
    }
}
