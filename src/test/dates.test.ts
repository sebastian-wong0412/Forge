import { localCalendarDate } from "../lib/dates";

test("uses the browser local calendar date, not UTC", () => {
  const lateLocal = new Date(2026, 7, 30, 23, 30, 0);
  expect(localCalendarDate(lateLocal)).toBe("2026-08-30");
});
