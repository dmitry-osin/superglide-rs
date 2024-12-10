use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub trigger_key: String,
    pub jump_key: String,
    pub crouch_key: String,
    pub fps: u32,
    pub delay_ms: f64,
    #[serde(default = "default_jump_hold_us")]
    pub jump_hold_us: u64,
    pub crouch_hold_ms: u64,
    pub theme: String,
    pub language: String,
}

fn default_jump_hold_us() -> u64 {
    500
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            trigger_key: "CapsLock".to_string(),
            jump_key: "Space".to_string(),
            crouch_key: "ControlLeft".to_string(),
            fps: 240,
            delay_ms: 5.0,
            jump_hold_us: 500,
            crouch_hold_ms: 300,
            theme: "light".to_string(),
            language: "en".to_string(),
        }
    }
}

fn settings_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("pro.osin.superglide");
    path.push("settings.json");
    path
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        Settings::default()
    }
}

pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = settings_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let content = serde_json::to_string_pretty(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
}