use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum LanguagePreference {
    #[default]
    System,
    Zh,
    En,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemePreference {
    #[default]
    System,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct Preferences {
    pub language: LanguagePreference,
    pub theme: ThemePreference,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            language: LanguagePreference::System,
            theme: ThemePreference::System,
        }
    }
}

fn preferences_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|err| format!("resolve config dir: {err}"))?;
    fs::create_dir_all(&dir).map_err(|err| format!("create config dir: {err}"))?;
    Ok(dir.join("settings.json"))
}

#[tauri::command]
pub fn load_preferences(app: AppHandle) -> Result<Preferences, String> {
    let path = preferences_path(&app)?;
    if !path.exists() {
        return Ok(Preferences::default());
    }
    let raw = fs::read_to_string(&path).map_err(|err| format!("read settings: {err}"))?;
    Ok(serde_json::from_str(&raw).unwrap_or_default())
}

#[tauri::command]
pub fn save_preferences(app: AppHandle, preferences: Preferences) -> Result<(), String> {
    let path = preferences_path(&app)?;
    let raw = serde_json::to_string_pretty(&preferences)
        .map_err(|err| format!("serialize settings: {err}"))?;
    fs::write(&path, raw).map_err(|err| format!("write settings: {err}"))
}

#[tauri::command]
pub fn app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
