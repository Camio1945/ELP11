//! Persistent application settings stored as JSON on disk.
//!
//! On Windows the config file lives at
//! `%APPDATA%\ELP11\settings.json`; on other platforms it lives at
//! `$HOME/.config/ELP11/settings.json`. The directory is created
//! automatically on first save.

use crate::app_state::SidebarTab;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Font size (in pixels) used for on-screen subtitles.
    pub subtitle_font_size: f32,
    /// Whether the file-open history feature is active.
    #[serde(default = "default_history_enabled")]
    pub history_enabled: bool,
    /// Maximum number of recent files to retain.
    #[serde(default = "default_max_history_items")]
    pub max_history_items: usize,
    /// Recently opened video file paths, most-recent first.
    #[serde(default)]
    pub recent_files: Vec<String>,
    /// Which sidebar tab was active when the app was last closed.
    #[serde(default)]
    pub active_tab: SidebarTab,
    /// Last-known playback position (in seconds) for recently opened files,
    /// keyed by absolute file path. Used to resume playback where the user
    /// left off.
    #[serde(default)]
    pub playback_positions: HashMap<String, f64>,
}

fn default_history_enabled() -> bool {
    true
}
fn default_max_history_items() -> usize {
    100
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            subtitle_font_size: 20.0,
            history_enabled: true,
            max_history_items: 100,
            recent_files: Vec::new(),
            active_tab: SidebarTab::default(),
            playback_positions: HashMap::new(),
        }
    }
}

impl AppSettings {
    pub const MIN_FONT_SIZE: f32 = 12.0;
    pub const MAX_FONT_SIZE: f32 = 48.0;
    pub const FONT_STEP: f32 = 2.0;

    pub const MIN_HISTORY_ITEMS: usize = 10;
    pub const MAX_HISTORY_ITEMS: usize = 1000;
    pub const HISTORY_STEP: usize = 10;

    pub fn increase_font(&mut self) {
        self.subtitle_font_size =
            (self.subtitle_font_size + Self::FONT_STEP).min(Self::MAX_FONT_SIZE);
    }

    pub fn decrease_font(&mut self) {
        self.subtitle_font_size =
            (self.subtitle_font_size - Self::FONT_STEP).max(Self::MIN_FONT_SIZE);
    }

    pub fn increase_max_history(&mut self) {
        self.max_history_items =
            (self.max_history_items + Self::HISTORY_STEP).min(Self::MAX_HISTORY_ITEMS);
        self.recent_files.truncate(self.max_history_items);
        self.prune_resume_positions();
    }

    pub fn decrease_max_history(&mut self) {
        self.max_history_items = (self.max_history_items.saturating_sub(Self::HISTORY_STEP))
            .max(Self::MIN_HISTORY_ITEMS);
        self.recent_files.truncate(self.max_history_items);
        self.prune_resume_positions();
    }

    /// Record a successfully opened file. The path is moved to the front;
    /// duplicates are removed. Stale entries beyond `max_history_items`
    /// are dropped.
    pub fn add_recent_file(&mut self, path: &str) {
        if !self.history_enabled || path.is_empty() {
            return;
        }
        self.recent_files.retain(|f| f != path);
        self.recent_files.insert(0, path.to_string());
        self.recent_files.truncate(self.max_history_items);
    }

    /// Record the last-known playback position (in seconds) for `path`.
    /// Entries for files no longer in the recent list are pruned so the map
    /// stays bounded by `max_history_items`.
    pub fn set_resume_position(&mut self, path: &str, position: f64) {
        if path.is_empty() {
            return;
        }
        self.playback_positions.insert(path.to_string(), position);
        self.prune_resume_positions();
    }

    /// Drop resume positions for files that are no longer tracked in the
    /// recent-files list.
    pub fn prune_resume_positions(&mut self) {
        self.playback_positions
            .retain(|k, _| self.recent_files.iter().any(|f| f == k));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Default ─────────────────────────────────────────────────

    #[test]
    fn test_default_values() {
        let settings = AppSettings::default();
        assert_eq!(settings.subtitle_font_size, 20.0);
        assert!(settings.history_enabled);
        assert_eq!(settings.max_history_items, 100);
        assert!(settings.recent_files.is_empty());
        assert!(settings.playback_positions.is_empty());
    }

    // ── Font size ───────────────────────────────────────────────

    #[test]
    fn test_increase_font_normal() {
        let mut s = AppSettings::default();
        s.subtitle_font_size = 20.0;
        s.increase_font();
        assert_eq!(s.subtitle_font_size, 22.0);
    }

    #[test]
    fn test_increase_font_at_max() {
        let mut s = AppSettings::default();
        s.subtitle_font_size = AppSettings::MAX_FONT_SIZE;
        s.increase_font();
        assert_eq!(s.subtitle_font_size, AppSettings::MAX_FONT_SIZE);
    }

    #[test]
    fn test_increase_font_near_max() {
        let mut s = AppSettings::default();
        s.subtitle_font_size = AppSettings::MAX_FONT_SIZE - 1.0;
        s.increase_font();
        // Clamped to MAX_FONT_SIZE (since 47 + 2 = 49 > 48, min(49, 48) = 48)
        assert_eq!(s.subtitle_font_size, AppSettings::MAX_FONT_SIZE);
    }

    #[test]
    fn test_decrease_font_normal() {
        let mut s = AppSettings::default();
        s.subtitle_font_size = 20.0;
        s.decrease_font();
        assert_eq!(s.subtitle_font_size, 18.0);
    }

    #[test]
    fn test_decrease_font_at_min() {
        let mut s = AppSettings::default();
        s.subtitle_font_size = AppSettings::MIN_FONT_SIZE;
        s.decrease_font();
        assert_eq!(s.subtitle_font_size, AppSettings::MIN_FONT_SIZE);
    }

    // ── History items ───────────────────────────────────────────

    #[test]
    fn test_increase_max_history_normal() {
        let mut s = AppSettings::default();
        s.max_history_items = 100;
        s.increase_max_history();
        assert_eq!(s.max_history_items, 110);
    }

    #[test]
    fn test_increase_max_history_at_max() {
        let mut s = AppSettings::default();
        s.max_history_items = AppSettings::MAX_HISTORY_ITEMS;
        s.increase_max_history();
        assert_eq!(s.max_history_items, AppSettings::MAX_HISTORY_ITEMS);
    }

    #[test]
    fn test_decrease_max_history_normal() {
        let mut s = AppSettings::default();
        s.max_history_items = 100;
        s.decrease_max_history();
        assert_eq!(s.max_history_items, 90);
    }

    #[test]
    fn test_decrease_max_history_at_min() {
        let mut s = AppSettings::default();
        s.max_history_items = AppSettings::MIN_HISTORY_ITEMS;
        s.decrease_max_history();
        assert_eq!(s.max_history_items, AppSettings::MIN_HISTORY_ITEMS);
    }

    #[test]
    fn test_decrease_max_history_truncates_recent_files() {
        let mut s = AppSettings::default();
        s.max_history_items = 15;
        s.add_recent_file("a");
        s.add_recent_file("b");
        s.add_recent_file("c");
        // After decreasing to min (10), files beyond 10 are truncated
        s.decrease_max_history(); // 15 - 10 = 5 → clamped to MIN(10)
        assert_eq!(s.max_history_items, AppSettings::MIN_HISTORY_ITEMS);
        assert!(s.recent_files.len() <= AppSettings::MIN_HISTORY_ITEMS);
    }

    // ── add_recent_file ─────────────────────────────────────────

    #[test]
    fn test_add_recent_file_basic() {
        let mut s = AppSettings::default();
        s.add_recent_file("video1.mp4");
        assert_eq!(s.recent_files, vec!["video1.mp4"]);
    }

    #[test]
    fn test_add_recent_file_deduplicates() {
        let mut s = AppSettings::default();
        s.add_recent_file("a.mp4");
        s.add_recent_file("b.mp4");
        s.add_recent_file("a.mp4"); // duplicate — moved to front
        assert_eq!(s.recent_files, vec!["a.mp4", "b.mp4"]);
    }

    #[test]
    fn test_add_recent_file_empty_path() {
        let mut s = AppSettings::default();
        s.add_recent_file("");
        assert!(s.recent_files.is_empty());
    }

    #[test]
    fn test_add_recent_file_history_disabled() {
        let mut s = AppSettings::default();
        s.history_enabled = false;
        s.add_recent_file("video.mp4");
        assert!(s.recent_files.is_empty());
    }

    #[test]
    fn test_add_recent_file_truncates() {
        let mut s = AppSettings::default();
        s.max_history_items = 3;
        for i in 0..5 {
            s.add_recent_file(&format!("video{i}.mp4"));
        }
        assert_eq!(s.recent_files.len(), 3);
        // Most recent first
        assert_eq!(s.recent_files[0], "video4.mp4");
    }

    // ── set_resume_position ─────────────────────────────────────

    #[test]
    fn test_set_resume_position_basic() {
        let mut s = AppSettings::default();
        s.add_recent_file("/path/to/video.mp4");
        s.set_resume_position("/path/to/video.mp4", 42.5);
        assert_eq!(s.playback_positions.get("/path/to/video.mp4"), Some(&42.5));
    }

    #[test]
    fn test_set_resume_position_empty_path() {
        let mut s = AppSettings::default();
        s.set_resume_position("", 10.0);
        assert!(s.playback_positions.is_empty());
    }

    #[test]
    fn test_set_resume_position_prunes_orphaned() {
        let mut s = AppSettings::default();
        s.add_recent_file("/path/to/video.mp4");
        // Set a position for a file that isn't in recent_files
        s.playback_positions
            .insert("/old/stale.mp4".to_string(), 100.0);
        s.set_resume_position("/path/to/video.mp4", 50.0);
        // The stale entry should be pruned
        assert!(!s.playback_positions.contains_key("/old/stale.mp4"));
        assert_eq!(s.playback_positions.get("/path/to/video.mp4"), Some(&50.0));
    }

    // ── prune_resume_positions ──────────────────────────────────

    #[test]
    fn test_prune_resume_positions_keeps_tracked() {
        let mut s = AppSettings::default();
        s.add_recent_file("a.mp4");
        s.playback_positions.insert("a.mp4".to_string(), 10.0);
        s.playback_positions.insert("orphan.mp4".to_string(), 20.0);
        s.prune_resume_positions();
        assert!(s.playback_positions.contains_key("a.mp4"));
        assert!(!s.playback_positions.contains_key("orphan.mp4"));
    }

    #[test]
    fn test_prune_resume_positions_empty() {
        let mut s = AppSettings::default();
        s.prune_resume_positions();
        assert!(s.playback_positions.is_empty());
    }

    // ── Constants ───────────────────────────────────────────────

    #[test]
    fn test_constants_values() {
        assert_eq!(AppSettings::MIN_FONT_SIZE, 12.0);
        assert_eq!(AppSettings::MAX_FONT_SIZE, 48.0);
        assert_eq!(AppSettings::FONT_STEP, 2.0);
        assert_eq!(AppSettings::MIN_HISTORY_ITEMS, 10);
        assert_eq!(AppSettings::MAX_HISTORY_ITEMS, 1000);
        assert_eq!(AppSettings::HISTORY_STEP, 10);
    }
}

fn config_dir() -> std::io::Result<PathBuf> {
    let base = if cfg!(target_os = "windows") {
        std::env::var("APPDATA")
            .map(PathBuf::from)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "APPDATA not set"))?
    } else {
        let home = std::env::var("HOME")
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::NotFound, "HOME not set"))?;
        let mut p = PathBuf::from(home);
        p.push(".config");
        p
    };
    let dir = base.join("ELP11");
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}

fn config_path() -> std::io::Result<PathBuf> {
    Ok(config_dir()?.join("settings.json"))
}

/// Load settings from disk. Returns defaults if the file is missing,
/// unreadable, or contains invalid JSON.
pub fn load() -> AppSettings {
    config_path()
        .ok()
        .and_then(|p| std::fs::read_to_string(p).ok())
        .and_then(|s| serde_json::from_str::<AppSettings>(&s).ok())
        .unwrap_or_default()
}

/// Persist settings to disk. Errors are intentionally swallowed because
/// settings are convenience state — a failed write should not crash the app.
///
/// The write is performed atomically (write to a temp file in the same
/// directory, then rename over the target) so that a crash or power loss
/// mid-write leaves the previous good file intact rather than a truncated or
/// partially-written one. This is important because the file is written
/// periodically (every few seconds) to track playback positions.
pub fn save(settings: &AppSettings) {
    let Ok(path) = config_path() else {
        return;
    };
    let Ok(json) = serde_json::to_string_pretty(settings) else {
        return;
    };

    // Atomic write: temp file + rename (same directory so the rename is
    // atomic on both POSIX and Windows NTFS).
    if let Some(dir) = path.parent() {
        let tmp = dir.join("settings.json.tmp");
        if std::fs::write(&tmp, &json).is_ok() && std::fs::rename(&tmp, &path).is_ok() {
            return;
        }
        // Remove a leftover temp file from a failed rename (best-effort).
        let _ = std::fs::remove_file(&tmp);
    }

    // Fallback: direct write (less crash-safe but better than nothing).
    if let Err(e) = std::fs::write(&path, &json) {
        log::warn!("failed to save settings to {}: {e}", path.display());
    }
}
