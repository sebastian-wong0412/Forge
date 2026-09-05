import type { IsoDate, Rfc3339 } from "../api/types";

const ISO_DATE = /^(\d{4})-(\d{2})-(\d{2})$/;
const DISPLAY_DATE = /^(\d{4})[/.-](\d{1,2})[/.-](\d{1,2})$/;

function pad2(value: number): string {
  return String(value).padStart(2, "0");
}

export function shiftCalendarDate(date: IsoDate, days: number): IsoDate {
  const [year, month, day] = date.split("-").map(Number);
  if (!year || !month || !day) {
    return date;
  }
  const shifted = new Date(Date.UTC(year, month - 1, day + days));
  const nextYear = shifted.getUTCFullYear();
  const nextMonth = pad2(shifted.getUTCMonth() + 1);
  const nextDay = pad2(shifted.getUTCDate());
  return `${nextYear}-${nextMonth}-${nextDay}`;
}

export function localCalendarDate(now = new Date()): IsoDate {
  const year = now.getFullYear();
  const month = pad2(now.getMonth() + 1);
  const day = pad2(now.getDate());
  return `${year}-${month}-${day}`;
}

function isRealUtcDate(year: number, month: number, day: number): boolean {
  const utc = new Date(Date.UTC(year, month - 1, day));
  return (
    utc.getUTCFullYear() === year && utc.getUTCMonth() === month - 1 && utc.getUTCDate() === day
  );
}

export function parseDisplayDate(value: string): IsoDate | null {
  const trimmed = value.trim();
  const match = trimmed.match(ISO_DATE) ?? trimmed.match(DISPLAY_DATE);
  if (!match) {
    return null;
  }
  const year = Number(match[1]);
  const month = Number(match[2]);
  const day = Number(match[3]);
  if (!isRealUtcDate(year, month, day)) {
    return null;
  }
  return `${match[1]}-${pad2(month)}-${pad2(day)}`;
}

export function formatDisplayDate(date: string): string {
  const parsed = parseDisplayDate(date);
  if (!parsed) {
    return date;
  }
  const [year, month, day] = parsed.split("-");
  return `${year}/${month}/${day}`;
}

export function formatCalendarDate(date: IsoDate): string {
  const [year, month, day] = date.split("-");
  const monthIndex = Number(month) - 1;
  if (!year || monthIndex < 0 || monthIndex > 11 || !day) {
    return date;
  }
  return formatDisplayDate(date);
}

export function formatTimestamp(value: Rfc3339): string {
  const parsed = new Date(value);
  if (Number.isNaN(parsed.getTime())) {
    return value;
  }
  return `${parsed.getFullYear()}/${pad2(parsed.getMonth() + 1)}/${pad2(parsed.getDate())} ${pad2(parsed.getHours())}:${pad2(parsed.getMinutes())}`;
}

export function dateRange(start: IsoDate | null, end: IsoDate | null): string | null {
  if (!start && !end) {
    return null;
  }
  if (start && end) {
    return `${formatDisplayDate(start)} – ${formatDisplayDate(end)}`;
  }
  return formatDisplayDate(start ?? end ?? "");
}
