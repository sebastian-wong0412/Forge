export type { LanguagePreference, Locale, MessageKey, ThemePreference, Vars } from "./translate";
export { resolveLocale, systemLocale, translate, statusMessageKey } from "./translate";
export { SettingsProvider, useSettings, useT, tCurrent, type TranslateFn } from "./SettingsProvider";
export {
  DEFAULT_PREFERENCES,
  GITHUB_REPO,
  GITHUB_REPO_URL,
  applyTheme,
  loadPreferences,
  loadPreferencesSync,
  savePreferences,
  type Preferences,
} from "./preferences";
