use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub content_type: String,       // "text", "code", "url", "image", "color"
    pub text_content: Option<String>,
    pub title: Option<String>,
    pub metadata: Option<String>,
    pub category: String,           // "text", "code", "url", "image"
    pub color_hex: Option<String>,
    pub thumbnail_data: Option<String>,
    pub image_width: Option<u32>,
    pub image_height: Option<u32>,
    pub image_size_bytes: Option<u64>,
    #[serde(default)]
    pub image_hash: Option<u64>,
    pub is_pinned: bool,
    pub created_at: i64,
    pub last_used_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,             // "pantheon-light", "pantheon-dark", "modern-flat-light", "modern-flat-dark"
    #[serde(default = "default_language")]
    pub language: String,          // "es", "en"
    #[serde(default)]
    pub clear_on_exit: bool,
    #[serde(default = "default_shortcut")]
    pub global_shortcut: String,   // "Super+V", "Alt+V", etc.
    #[serde(default = "default_max_history")]
    pub max_history: usize,
}

fn default_language() -> String {
    "es".to_string()
}

fn default_shortcut() -> String {
    "Super+V".to_string()
}

fn default_max_history() -> usize {
    100
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "pantheon-light".to_string(),
            language: "es".to_string(),
            clear_on_exit: false,
            global_shortcut: "Super+V".to_string(),
            max_history: 100,
        }
    }
}
