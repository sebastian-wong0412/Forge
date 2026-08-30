import { Link } from "react-router-dom";
import type { IsoDate, Task, TodayResponse } from "../api/types";
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
  const sections = [
    { title: "今日计划", tasks: today.scheduled },
    { title: "已逾期", tasks: today.overdue },
    { title: "进行中（未安排）", tasks: today.unscheduled_in_progress },
    { title: "今日完成", tasks: today.completed },
  ].filter((section) => section.tasks.length > 0);

  return (
    <div className="stack">
      <PageHeader
        kicker="今日"
        title={formatCalendarDate(today.date)}
        actions={
          <TodayDateNav date={today.date} localToday={localToday} onChange={onDateChange} />
        }
      />
      {!hasCycles ? (
        <OnboardingCard />
      ) : sections.length === 0 ? (
        <EmptyState
          title="今天没有安排任务"
          detail="先创建一个周期，然后建立项目和任务。"
          action={
            <Link to="/cycles" className="btn btn-primary">
              去创建周期
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
