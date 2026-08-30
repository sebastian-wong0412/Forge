import type { IsoDate } from "../api/types";
import { useT } from "../i18n";
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
  const t = useT();

  return (
    <div className="row date-nav" role="group" aria-label={t("today.dateNav")}>
      <button type="button" className="btn" onClick={() => onChange(shiftCalendarDate(date, -1))}>
        {t("today.prevDay")}
      </button>
      <span className="date-nav-current">{date}</span>
      <button type="button" className="btn" onClick={() => onChange(localToday)}>
        {t("today.today")}
      </button>
      <button type="button" className="btn" onClick={() => onChange(shiftCalendarDate(date, 1))}>
        {t("today.nextDay")}
      </button>
    </div>
  );
}
