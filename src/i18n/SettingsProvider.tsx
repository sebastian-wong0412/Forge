import {
  createContext,
  createElement,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { isTauriShell } from "../config";
import {
  applyTheme,
  DEFAULT_PREFERENCES,
  loadPreferences,
  loadPreferencesSync,
  savePreferences,
  type Preferences,
} from "./preferences";
import {
  resolveLocale,
  translate,
  type LanguagePreference,
  type Locale,
  type MessageKey,
  type ThemePreference,
  type Vars,
} from "./translate";

export type TranslateFn = (key: MessageKey, vars?: Vars) => string;

interface SettingsContextValue {
  preferences: Preferences;
  locale: Locale;
  t: TranslateFn;
  setLanguage: (language: LanguagePreference) => void;
  setTheme: (theme: ThemePreference) => void;
}

const SettingsContext = createContext<SettingsContextValue | null>(null);

let currentTranslate: TranslateFn = (key, vars) =>
  translate(resolveLocale(loadPreferencesSync().language), key, vars);

export function tCurrent(key: MessageKey, vars?: Vars): string {
  return currentTranslate(key, vars);
}

export function SettingsProvider({ children }: { children: ReactNode }) {
  const [preferences, setPreferences] = useState<Preferences>(loadPreferencesSync);
  const dirty = useRef(false);

  useEffect(() => {
    if (!isTauriShell()) {
      return;
    }
    let cancelled = false;
    void loadPreferences().then((loaded) => {
      if (cancelled || dirty.current) {
        return;
      }
      setPreferences(loaded);
      applyTheme(loaded.theme);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => {
    applyTheme(preferences.theme);
  }, [preferences.theme]);

  const locale = useMemo(() => resolveLocale(preferences.language), [preferences.language]);
  const t = useCallback<TranslateFn>((key, vars) => translate(locale, key, vars), [locale]);

  useEffect(() => {
    currentTranslate = t;
    document.documentElement.lang = locale === "zh" ? "zh-CN" : "en";
    return () => {
      currentTranslate = (key, vars) =>
        translate(resolveLocale(loadPreferencesSync().language), key, vars);
    };
  }, [locale, t]);

  const setLanguage = useCallback((language: LanguagePreference) => {
    dirty.current = true;
    setPreferences((current) => {
      const next = { ...current, language };
      void savePreferences(next);
      return next;
    });
  }, []);

  const setTheme = useCallback((theme: ThemePreference) => {
    dirty.current = true;
    setPreferences((current) => {
      const next = { ...current, theme };
      void savePreferences(next);
      return next;
    });
  }, []);

  const value = useMemo(
    () => ({ preferences, locale, t, setLanguage, setTheme }),
    [preferences, locale, t, setLanguage, setTheme],
  );

  return createElement(SettingsContext.Provider, { value }, children);
}

export function useSettings(): SettingsContextValue {
  const value = useContext(SettingsContext);
  if (!value) {
    const locale = resolveLocale("system");
    return {
      preferences: DEFAULT_PREFERENCES,
      locale,
      t: (key, vars) => translate(locale, key, vars),
      setLanguage: () => undefined,
      setTheme: () => undefined,
    };
  }
  return value;
}

export function useT(): TranslateFn {
  return useSettings().t;
}
