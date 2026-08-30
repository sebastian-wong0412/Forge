import { formatCalendarDate, localCalendarDate } from "../lib/dates";

test("uses the browser local calendar date, not UTC", () => {
  const lateLocal = new Date(2026, 7, 30, 23, 30, 0);
  expect(localCalendarDate(lateLocal)).toBe("2026-08-30");
});

test("formats calendar dates for the active locale", () => {
  expect(formatCalendarDate("2026-08-30", "zh")).toBe("2026年8月30日");
  expect(formatCalendarDate("2026-08-30", "en")).toBe("August 30, 2026");
});
