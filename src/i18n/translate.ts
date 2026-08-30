import { en } from "./en";
import { zh, type MessageKey } from "./zh";

export type Locale = "zh" | "en";
export type LanguagePreference = "system" | "zh" | "en";
export type ThemePreference = "system" | "dark";

export type Vars = Record<string, string | number>;

const catalogs: Record<Locale, Record<MessageKey, string>> = { zh, en };

export function systemLocale(language = navigator.language): Locale {
  return language.toLowerCase().startsWith("zh") ? "zh" : "en";
}

export function resolveLocale(preference: LanguagePreference): Locale {
  return preference === "system" ? systemLocale() : preference;
}

export function translate(locale: Locale, key: MessageKey, vars?: Vars): string {
  let template = catalogs[locale][key] ?? catalogs.zh[key] ?? key;
  if (!vars) {
    return template;
  }
  return template.replace(/\{(\w+)\}/g, (_, name: string) =>
    vars[name] === undefined ? `{${name}}` : String(vars[name]),
  );
}

export function isMessageKey(value: string): value is MessageKey {
  return value in catalogs.zh;
}

export function statusMessageKey(status: string): MessageKey | null {
  const key = `status.${status}`;
  return isMessageKey(key) ? key : null;
}

export type { MessageKey };
export { zh, en };
