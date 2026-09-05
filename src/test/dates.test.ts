import {
  formatCalendarDate,
  formatDisplayDate,
  formatTimestamp,
  localCalendarDate,
  parseDisplayDate,
} from "../lib/dates";

test("uses the browser local calendar date, not UTC", () => {
  const lateLocal = new Date(2026, 7, 30, 23, 30, 0);
  expect(localCalendarDate(lateLocal)).toBe("2026-08-30");
});

test("formats calendar dates as YYYY/MM/DD regardless of locale", () => {
  expect(formatDisplayDate("2026-09-05")).toBe("2026/09/05");
  expect(formatCalendarDate("2026-09-05")).toBe("2026/09/05");
  expect(formatDisplayDate("2026-09-05")).not.toMatch(/日/);
  expect(formatCalendarDate("2026-09-05")).not.toMatch(/日/);
});

test("parses display dates back to API ISO dates", () => {
  expect(parseDisplayDate("2026/09/05")).toBe("2026-09-05");
  expect(parseDisplayDate("2026-09-05")).toBe("2026-09-05");
  expect(parseDisplayDate("2026/9/5")).toBe("2026-09-05");
  expect(parseDisplayDate("not-a-date")).toBeNull();
});

test("timestamp presentation does not use a Chinese day suffix", () => {
  const formatted = formatTimestamp("2026-09-05T09:00:00Z");
  expect(formatted).not.toMatch(/日/);
  expect(formatted).toMatch(/\d{4}\/\d{2}\/\d{2} \d{2}:\d{2}/);
});
