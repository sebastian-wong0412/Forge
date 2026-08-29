import type { IsoDate } from "../api/types";
import { shiftCalendarDate } from "../lib/dates";

export function TodayDateNav({
  date,
  localToday,
  onChange,
}: {
  date: IsoDate;
  localToday: IsoDate;
  onChange: (date: IsoDate) => void;
}) {
  return (
    <div className="row date-nav" role="group" aria-label="选择日期">
      <button type="button" className="btn" onClick={() => onChange(shiftCalendarDate(date, -1))}>
        ‹ 前一天
      </button>
      <span className="date-nav-current">{date}</span>
      <button type="button" className="btn" onClick={() => onChange(localToday)}>
        今天
      </button>
      <button type="button" className="btn" onClick={() => onChange(shiftCalendarDate(date, 1))}>
        后一天 ›
      </button>
    </div>
  );
}
