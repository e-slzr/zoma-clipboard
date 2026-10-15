use std::path::Path;
use rusqlite::{params, Connection, Result};
use crate::models::{Clip, AppSettings};
use chrono::Utc;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        if let Some(parent) = path.as_ref().parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS clips (
                id TEXT PRIMARY KEY,
                content_type TEXT NOT NULL,
                text_content TEXT,
                title TEXT,
                metadata TEXT,
                category TEXT NOT NULL,
                color_hex TEXT,
                thumbnail_data TEXT,
                image_width INTEGER,
                image_height INTEGER,
                image_size_bytes INTEGER,
                image_hash INTEGER,
                is_pinned INTEGER NOT NULL DEFAULT 0,
                created_at INTEGER NOT NULL,
                last_used_at INTEGER NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_clips_created_at ON clips(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_clips_last_used_at ON clips(last_used_at DESC);
            CREATE INDEX IF NOT EXISTS idx_clips_is_pinned ON clips(is_pinned);
            CREATE INDEX IF NOT EXISTS idx_clips_category ON clips(category);

            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            "
        )?;

        // Ensure image_hash column exists on existing databases
        let _ = self.conn.execute("ALTER TABLE clips ADD COLUMN image_hash INTEGER", []);
        let _ = self.conn.execute("CREATE INDEX IF NOT EXISTS idx_clips_image_hash ON clips(image_hash)", []);

        Ok(())
    }

    pub fn insert_clip(&self, clip: &Clip) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO clips (
                id, content_type, text_content, title, metadata, category,
                color_hex, thumbnail_data, image_width, image_height,
                image_size_bytes, image_hash, is_pinned, created_at, last_used_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                clip.id,
                clip.content_type,
                clip.text_content,
                clip.title,
                clip.metadata,
                clip.category,
                clip.color_hex,
                clip.thumbnail_data,
                clip.image_width,
                clip.image_height,
                clip.image_size_bytes,
                clip.image_hash.map(|h| h as i64),
                if clip.is_pinned { 1 } else { 0 },
                clip.created_at,
                clip.last_used_at,
            ],
        )?;
        Ok(())
    }

    pub fn find_by_text(&self, text: &str) -> Result<Option<Clip>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, text_content, title, metadata, category,
                    color_hex, thumbnail_data, image_width, image_height,
                    image_size_bytes, image_hash, is_pinned, created_at, last_used_at
             FROM clips WHERE text_content = ?1 LIMIT 1"
        )?;

        let mut rows = stmt.query(params![text])?;
        if let Some(row) = rows.next()? {
            let hash_val: Option<i64> = row.get(11)?;
            let is_pinned: i64 = row.get(12)?;
            Ok(Some(Clip {
                id: row.get(0)?,
                content_type: row.get(1)?,
                text_content: row.get(2)?,
                title: row.get(3)?,
                metadata: row.get(4)?,
                category: row.get(5)?,
                color_hex: row.get(6)?,
                thumbnail_data: row.get(7)?,
                image_width: row.get(8)?,
                image_height: row.get(9)?,
                image_size_bytes: row.get(10)?,
                image_hash: hash_val.map(|h| h as u64),
                is_pinned: is_pinned != 0,
                created_at: row.get(13)?,
                last_used_at: row.get(14)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn find_by_image_hash(&self, hash: u64) -> Result<Option<Clip>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, content_type, text_content, title, metadata, category,
                    color_hex, thumbnail_data, image_width, image_height,
                    image_size_bytes, image_hash, is_pinned, created_at, last_used_at
             FROM clips WHERE image_hash = ?1 LIMIT 1"
        )?;

        let mut rows = stmt.query(params![hash as i64])?;
        if let Some(row) = rows.next()? {
            let hash_val: Option<i64> = row.get(11)?;
            let is_pinned: i64 = row.get(12)?;
            Ok(Some(Clip {
                id: row.get(0)?,
                content_type: row.get(1)?,
                text_content: row.get(2)?,
                title: row.get(3)?,
                metadata: row.get(4)?,
                category: row.get(5)?,
                color_hex: row.get(6)?,
                thumbnail_data: row.get(7)?,
                image_width: row.get(8)?,
                image_height: row.get(9)?,
                image_size_bytes: row.get(10)?,
                image_hash: hash_val.map(|h| h as u64),
                is_pinned: is_pinned != 0,
                created_at: row.get(13)?,
                last_used_at: row.get(14)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn touch_clip(&self, id: &str) -> Result<()> {
        let now = Utc::now().timestamp_millis();
        self.conn.execute(
            "UPDATE clips SET last_used_at = ?1, created_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;
        Ok(())
    }

    pub fn touch_and_deduplicate(&self, id: &str) -> Result<()> {
        let now = Utc::now().timestamp_millis();
        self.conn.execute(
            "UPDATE clips SET last_used_at = ?1, created_at = ?1 WHERE id = ?2",
            params![now, id],
        )?;

        let clip_info: Option<(String, Option<String>, Option<i64>)> = self.conn.query_row(
            "SELECT content_type, text_content, image_hash FROM clips WHERE id = ?1",
            params![id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        ).ok();

        if let Some((content_type, text_content, image_hash)) = clip_info {
            if content_type == "image" {
                if let Some(h) = image_hash {
                    let _ = self.conn.execute(
                        "DELETE FROM clips WHERE id != ?1 AND image_hash = ?2",
                        params![id, h],
                    );
                }
            } else if let Some(text) = text_content {
                let _ = self.conn.execute(
                    "DELETE FROM clips WHERE id != ?1 AND text_content = ?2",
                    params![id, text],
                );
            }
        }
        Ok(())
    }

    pub fn get_clips(&self, search: Option<&str>, category: Option<&str>) -> Result<Vec<Clip>> {
        let mut query = "SELECT id, content_type, text_content, title, metadata, category,
                                color_hex, thumbnail_data, image_width, image_height,
                                image_size_bytes, image_hash, is_pinned, created_at, last_used_at
                         FROM clips WHERE 1=1".to_string();

        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(cat) = category {
            if cat == "pinned" || cat == "favorites" {
                query.push_str(" AND is_pinned = 1");
            } else if cat != "all" && !cat.is_empty() {
                query.push_str(" AND category = ?");
                params_vec.push(Box::new(cat.to_string()));
            }
        }

        if let Some(s) = search {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                query.push_str(" AND (text_content LIKE ? OR title LIKE ? OR metadata LIKE ?)");
                let pattern = format!("%{}%", trimmed);
                params_vec.push(Box::new(pattern.clone()));
                params_vec.push(Box::new(pattern.clone()));
                params_vec.push(Box::new(pattern));
            }
        }

        query.push_str(" ORDER BY is_pinned DESC, last_used_at DESC LIMIT 150");

        let mut stmt = self.conn.prepare(&query)?;
        let rusqlite_params: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();

        let rows = stmt.query_map(rusqlite_params.as_slice(), |row| {
            let hash_val: Option<i64> = row.get(11)?;
            let is_pinned: i64 = row.get(12)?;
            Ok(Clip {
                id: row.get(0)?,
                content_type: row.get(1)?,
                text_content: row.get(2)?,
                title: row.get(3)?,
                metadata: row.get(4)?,
                category: row.get(5)?,
                color_hex: row.get(6)?,
                thumbnail_data: row.get(7)?,
                image_width: row.get(8)?,
                image_height: row.get(9)?,
                image_size_bytes: row.get(10)?,
                image_hash: hash_val.map(|h| h as u64),
                is_pinned: is_pinned != 0,
                created_at: row.get(13)?,
                last_used_at: row.get(14)?,
            })
        })?;

        let mut clips = Vec::new();
        for r in rows {
            clips.push(r?);
        }
        Ok(clips)
    }

    pub fn toggle_pin(&self, id: &str) -> Result<bool> {
        let current: i64 = self.conn.query_row(
            "SELECT is_pinned FROM clips WHERE id = ?1",
            params![id],
            |r| r.get(0),
        )?;
        let new_state = if current == 0 { 1 } else { 0 };
        self.conn.execute(
            "UPDATE clips SET is_pinned = ?1 WHERE id = ?2",
            params![new_state, id],
        )?;
        Ok(new_state == 1)
    }

    pub fn delete_clip(&self, id: &str) -> Result<()> {
        self.conn.execute("DELETE FROM clips WHERE id = ?1", params![id])?;
        Ok(())
    }

    pub fn clear_unpinned(&self) -> Result<()> {
        self.conn.execute("DELETE FROM clips WHERE is_pinned = 0", [])?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<AppSettings> {
        let mut settings = AppSettings::default();

        let mut stmt = self.conn.prepare("SELECT key, value FROM settings")?;
        let rows = stmt.query_map([], |row| {
            let k: String = row.get(0)?;
            let v: String = row.get(1)?;
            Ok((k, v))
        })?;

        for r in rows {
            let (k, v) = r?;
            match k.as_str() {
                "theme" => settings.theme = v,
                "language" => settings.language = v,
                "clear_on_exit" => settings.clear_on_exit = v == "true",
                "global_shortcut" => settings.global_shortcut = v,
                "max_history" => {
                    if let Ok(num) = v.parse::<usize>() {
                        settings.max_history = num;
                    }
                }
                _ => {}
            }
        }

        Ok(settings)
    }

    pub fn save_settings(&self, s: &AppSettings) -> Result<()> {
        let entries = [
            ("theme", s.theme.as_str()),
            ("language", s.language.as_str()),
            ("clear_on_exit", if s.clear_on_exit { "true" } else { "false" }),
            ("global_shortcut", s.global_shortcut.as_str()),
            ("max_history", &s.max_history.to_string()),
        ];

        for (k, v) in entries {
            self.conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
                params![k, v],
            )?;
        }
        Ok(())
    }
}
