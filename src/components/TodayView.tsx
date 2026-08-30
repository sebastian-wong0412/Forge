import { Link } from "react-router-dom";
import type { IsoDate, Task, TodayResponse } from "../api/types";
import { useSettings } from "../i18n";
import { formatCalendarDate } from "../lib/dates";
import { EmptyState } from "./EmptyState";
import { OnboardingCard } from "./OnboardingCard";
import { PageHeader } from "./PageHeader";
import { TaskList } from "./TaskList";
import { TodayDateNav } from "./TodayDateNav";

export function TodayView({
  today,
  localToday,
  hasCycles = false,
  busyId,
  onDateChange,
  onStart,
  onComplete,
  onCancel,
  onSchedule,
  onUnschedule,
}: {
  today: TodayResponse;
  localToday: IsoDate;
  hasCycles?: boolean;
  busyId?: string | null;
  onDateChange: (date: IsoDate) => void;
  onStart: (task: Task) => void;
  onComplete: (task: Task) => void;
  onCancel: (task: Task) => void;
  onSchedule: (task: Task) => void;
  onUnschedule: (task: Task) => void;
}) {
  const { t, locale } = useSettings();
  const sections = [
    { title: t("today.section.scheduled"), tasks: today.scheduled },
    { title: t("today.section.overdue"), tasks: today.overdue },
    { title: t("today.section.unscheduled"), tasks: today.unscheduled_in_progress },
    { title: t("today.section.completed"), tasks: today.completed },
  ].filter((section) => section.tasks.length > 0);

  return (
    <div className="stack">
      <PageHeader
        kicker={t("today.kicker")}
        title={formatCalendarDate(today.date, locale)}
        actions={
          <TodayDateNav date={today.date} localToday={localToday} onChange={onDateChange} />
        }
      />
      {!hasCycles ? (
        <OnboardingCard />
      ) : sections.length === 0 ? (
        <EmptyState
          title={t("today.empty.title")}
          detail={t("today.empty.detail")}
          action={
            <Link to="/cycles" className="btn btn-primary">
              {t("today.empty.action")}
            </Link>
          }
        />
      ) : (
        <div className="stack">
          {sections.map((section) => (
            <TaskList
              key={section.title}
              title={section.title}
              tasks={section.tasks}
              empty=""
              busyId={busyId}
              onStart={onStart}
              onComplete={onComplete}
              onCancel={onCancel}
              onSchedule={onSchedule}
              onUnschedule={onUnschedule}
            />
          ))}
        </div>
      )}
    </div>
  );
}
