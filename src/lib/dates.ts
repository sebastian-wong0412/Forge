import type { IsoDate, Rfc3339 } from "../api/types";
import { resolveLocale, type Locale } from "../i18n";

export function shiftCalendarDate(date: IsoDate, days: number): IsoDate {
  const [year, month, day] = date.split("-").map(Number);
  if (!year || !month || !day) {
    return date;
  }
  const shifted = new Date(Date.UTC(year, month - 1, day + days));
  const nextYear = shifted.getUTCFullYear();
  const nextMonth = String(shifted.getUTCMonth() + 1).padStart(2, "0");
  const nextDay = String(shifted.getUTCDate()).padStart(2, "0");
  return `${nextYear}-${nextMonth}-${nextDay}`;
}

export function localCalendarDate(now = new Date()): IsoDate {
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  const day = String(now.getDate()).padStart(2, "0");
  return `${year}-${month}-${day}`;
}

export function formatCalendarDate(
  date: IsoDate,
  locale: Locale = resolveLocale("system"),
): string {
  const [year, month, day] = date.split("-");
  const monthIndex = Number(month) - 1;
  if (!year || monthIndex < 0 || monthIndex > 11 || !day) {
    return date;
  }
  if (locale === "en") {
    return new Date(Date.UTC(Number(year), monthIndex, Number(day))).toLocaleDateString("en-US", {
      year: "numeric",
      month: "long",
      day: "numeric",
      timeZone: "UTC",
    });
  }
  return `${year}年${Number(month)}月${Number(day)}日`;
}

export function formatTimestamp(
  value: Rfc3339,
  locale: Locale = resolveLocale("system"),
): string {
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return value;
  }
  return parsed.toLocaleString(locale === "zh" ? "zh-CN" : "en-US", {
    year: "numeric",
    month: "numeric",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function dateRange(start: IsoDate | null, end: IsoDate | null): string | null {
  if (!start && !end) {
    return null;
  }
  if (start && end) {
    return `${start} – ${end}`;
  }
  return start ?? end;
}
