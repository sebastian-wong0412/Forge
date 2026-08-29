import { fireEvent, render, screen } from "@testing-library/react";
import { TodayDateNav } from "../components/TodayDateNav";
import { localCalendarDate, shiftCalendarDate } from "../lib/dates";

test("defaults to the browser local calendar date", () => {
  const lateLocal = new Date(2026, 7, 30, 23, 30, 0);
  expect(localCalendarDate(lateLocal)).toBe("2026-08-30");
});

test("Previous day changes the query date", () => {
  const onChange = vi.fn();
  render(<TodayDateNav date="2026-08-30" localToday="2026-08-30" onChange={onChange} />);
  fireEvent.click(screen.getByRole("button", { name: "‹ 前一天" }));
  expect(onChange).toHaveBeenCalledWith("2026-08-29");
});

test("Next day changes the query date", () => {
  const onChange = vi.fn();
  render(<TodayDateNav date="2026-08-30" localToday="2026-08-30" onChange={onChange} />);
  fireEvent.click(screen.getByRole("button", { name: "后一天 ›" }));
  expect(onChange).toHaveBeenCalledWith("2026-08-31");
});

test("Today button returns to the local today", () => {
  const onChange = vi.fn();
  render(<TodayDateNav date="2026-08-29" localToday="2026-08-30" onChange={onChange} />);
  fireEvent.click(screen.getByRole("button", { name: "今天" }));
  expect(onChange).toHaveBeenCalledWith("2026-08-30");
});

test("calendar date arithmetic does not use toISOString", () => {
  expect(shiftCalendarDate("2026-08-30", -1)).toBe("2026-08-29");
  expect(shiftCalendarDate("2026-08-30", 1)).toBe("2026-08-31");
});
