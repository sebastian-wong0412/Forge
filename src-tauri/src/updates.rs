use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

const GITHUB_LATEST: &str = "https://api.github.com/repos/sebastian-wong0412/Forge/releases/latest";
const USER_AGENT: &str = concat!("Forge/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCheck {
    current_version: String,
    latest_version: String,
    notes: String,
    up_to_date: bool,
    asset_name: Option<String>,
    download_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    tag_name: String,
    body: Option<String>,
    assets: Option<Vec<GithubAsset>>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

fn normalize_version(value: &str) -> String {
    value.trim().trim_start_matches(['v', 'V']).to_string()
}

fn compare_semver(left: &str, right: &str) -> i32 {
    let parse = |value: &str| {
        normalize_version(value)
            .split('.')
            .map(|part| part.parse::<u64>().unwrap_or(0))
            .collect::<Vec<_>>()
    };
    let a = parse(left);
    let b = parse(right);
    let len = a.len().max(b.len());
    for index in 0..len {
        let left_part = *a.get(index).unwrap_or(&0);
        let right_part = *b.get(index).unwrap_or(&0);
        if left_part > right_part {
            return 1;
        }
        if left_part < right_part {
            return -1;
        }
    }
    0
}

fn is_windows_x64_installer(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with(".exe") && lower.contains("x64") && lower.contains("setup")
}

#[tauri::command]
pub async fn check_for_updates() -> Result<UpdateCheck, String> {
    let current = env!("CARGO_PKG_VERSION").to_string();
    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .build()
        .map_err(|_| "network".to_string())?;
    let response = client
        .get(GITHUB_LATEST)
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .map_err(|_| "network".to_string())?;
    if !response.status().is_success() {
        return Err("failed".to_string());
    }
    let release: GithubRelease = response.json().await.map_err(|_| "invalid".to_string())?;
    if release.tag_name.trim().is_empty() {
        return Err("invalid".to_string());
    }
    let latest = normalize_version(&release.tag_name);
    let asset = release
        .assets
        .unwrap_or_default()
        .into_iter()
        .find(|item| is_windows_x64_installer(&item.name));
    Ok(UpdateCheck {
        current_version: current.clone(),
        latest_version: latest.clone(),
        notes: release.body.unwrap_or_default().trim().to_string(),
        up_to_date: compare_semver(&current, &latest) >= 0,
        asset_name: asset.as_ref().map(|item| item.name.clone()),
        download_url: asset.map(|item| item.browser_download_url),
    })
}

#[tauri::command]
pub async fn download_installer(app: AppHandle, url: String) -> Result<String, String> {
    if !(url.starts_with("https://github.com/")
        || url.starts_with("https://objects.githubusercontent.com/")
        || url.starts_with("https://release-assets.githubusercontent.com/"))
    {
        return Err("download".to_string());
    }

    let client = reqwest::Client::builder()
        .user_agent(USER_AGENT)
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|_| "download".to_string())?;
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|_| "network".to_string())?;
    if !response.status().is_success() {
        return Err("download".to_string());
    }

    let file_name = response
        .url()
        .path_segments()
        .and_then(|mut segments| segments.next_back())
        .filter(|name| name.to_ascii_lowercase().ends_with(".exe"))
        .map(|name| name.to_string())
        .unwrap_or_else(|| "Forge-setup.exe".to_string());

    let dir = app
        .path()
        .download_dir()
        .map_err(|_| "download".to_string())?;
    std::fs::create_dir_all(&dir).map_err(|_| "download".to_string())?;
    let path = unique_path(dir.join(file_name));
    let bytes = response.bytes().await.map_err(|_| "download".to_string())?;
    let mut file = File::create(&path).map_err(|_| "download".to_string())?;
    file.write_all(&bytes).map_err(|_| "download".to_string())?;
    Ok(path.display().to_string())
}

fn unique_path(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }
    let stem = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("Forge-setup");
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("exe");
    let parent = path
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    for index in 2..1000 {
        let candidate = parent.join(format!("{stem}-{index}.{ext}"));
        if !candidate.exists() {
            return candidate;
        }
    }
    path
}

#[tauri::command]
pub fn open_external(target: String) -> Result<(), String> {
    if target.starts_with("https://github.com/sebastian-wong0412/Forge") {
        return open::that(&target).map_err(|err| format!("open: {err}"));
    }

    let path = PathBuf::from(&target);
    if !path.exists() {
        return Err("open".to_string());
    }

    #[cfg(windows)]
    {
        std::process::Command::new("explorer")
            .arg(format!("/select,{}", path.display()))
            .spawn()
            .map_err(|err| format!("open: {err}"))?;
        Ok(())
    }

    #[cfg(not(windows))]
    {
        let reveal = path.parent().unwrap_or(path.as_path());
        open::that(reveal).map_err(|err| format!("open: {err}"))
    }
}

#[cfg(test)]
mod tests {
    use super::{compare_semver, is_windows_x64_installer, normalize_version};

    #[test]
    fn strips_version_prefix() {
        assert_eq!(normalize_version("v0.3.0"), "0.3.0");
        assert_eq!(normalize_version("0.3.0"), "0.3.0");
    }

    #[test]
    fn compares_semver_parts() {
        assert!(compare_semver("0.3.0", "0.2.1") > 0);
        assert_eq!(compare_semver("0.3.0", "v0.3.0"), 0);
        assert!(compare_semver("0.2.1", "0.3.0") < 0);
    }

    #[test]
    fn accepts_windows_x64_setup_exe() {
        assert!(is_windows_x64_installer("Forge_0.3.0_x64-setup.exe"));
        assert!(!is_windows_x64_installer("Forge_0.3.0.dmg"));
        assert!(!is_windows_x64_installer("Forge_0.3.0_aarch64-setup.exe"));
    }
}
