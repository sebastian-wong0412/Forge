import { resolveLocale, systemLocale, translate } from "../i18n";

test("system locale treats zh* as Chinese and everything else as English", () => {
  expect(systemLocale("zh-CN")).toBe("zh");
  expect(systemLocale("zh-TW")).toBe("zh");
  expect(systemLocale("en-US")).toBe("en");
  expect(systemLocale("ja-JP")).toBe("en");
});

test("resolveLocale follows the explicit preference or the system", () => {
  expect(resolveLocale("zh")).toBe("zh");
  expect(resolveLocale("en")).toBe("en");
  expect(resolveLocale("system")).toBe("zh");
});

test("translate interpolates variables and falls back by key", () => {
  expect(translate("zh", "nav.today")).toBe("今日");
  expect(translate("en", "nav.today")).toBe("Today");
  expect(translate("en", "settings.update.current", { version: "0.3.0" })).toBe(
    "Current version 0.3.0",
  );
});
