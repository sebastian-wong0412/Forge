import { isTauriShell } from "../config";
import type { LanguagePreference, ThemePreference } from "./translate";

export interface Preferences {
  language: LanguagePreference;
  theme: ThemePreference;
}

export const DEFAULT_PREFERENCES: Preferences = {
  language: "system",
  theme: "system",
};

const STORAGE_KEY = "forge.preferences";

function parsePreferences(value: unknown): Preferences {
  if (!value || typeof value !== "object") {
    return DEFAULT_PREFERENCES;
  }
  const record = value as Record<string, unknown>;
  const language =
    record.language === "zh" || record.language === "en" || record.language === "system"
      ? record.language
      : DEFAULT_PREFERENCES.language;
  const theme = record.theme === "dark" || record.theme === "system" ? record.theme : DEFAULT_PREFERENCES.theme;
  return { language, theme };
}

export function loadPreferencesSync(): Preferences {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    return raw ? parsePreferences(JSON.parse(raw)) : DEFAULT_PREFERENCES;
  } catch {
    return DEFAULT_PREFERENCES;
  }
}

export async function loadPreferences(): Promise<Preferences> {
  if (isTauriShell()) {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const parsed = parsePreferences(await invoke("load_preferences"));
      try {
        localStorage.setItem(STORAGE_KEY, JSON.stringify(parsed));
      } catch {
        // ignore quota / private-mode failures
      }
      return parsed;
    } catch {
      return loadPreferencesSync();
    }
  }
  return loadPreferencesSync();
}

export async function savePreferences(preferences: Preferences): Promise<void> {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(preferences));
  } catch {
    // ignore quota / private-mode failures
  }
  if (!isTauriShell()) {
    return;
  }
  try {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("save_preferences", { preferences });
  } catch {
    // keep the in-memory / localStorage copy
  }
}

export function applyTheme(theme: ThemePreference): void {
  document.documentElement.dataset.theme = theme;
}

export const GITHUB_REPO_URL = "https://github.com/sebastian-wong0412/Forge";
export const GITHUB_REPO = "sebastian-wong0412/Forge";
