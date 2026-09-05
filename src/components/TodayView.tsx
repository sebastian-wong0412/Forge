import { Link } from "react-router-dom";
import type { Cycle, IsoDate, Project, Task, TodayResponse } from "../api/types";
import { useT } from "../i18n";
import { formatCalendarDate } from "../lib/dates";
import { openCycleShortcuts } from "../lib/cycles";
import { executionDestination } from "../lib/todayProjects";
import { EmptyState } from "./EmptyState";
import { OnboardingCard } from "./OnboardingCard";
import { PageHeader } from "./PageHeader";
import { StatusBadge } from "./StatusBadge";
import { TaskList } from "./TaskList";
import { TodayDateNav } from "./TodayDateNav";

export function TodayView({
  today,
  localToday,
  cycles = [],
  projects = [],
  projectTitles,
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
  cycles?: Cycle[];
  projects?: Project[];
  projectTitles?: Record<string, string>;
  busyId?: string | null;
  onDateChange: (date: IsoDate) => void;
  onStart: (task: Task) => void;
  onComplete: (task: Task) => void;
  onCancel: (task: Task) => void;
  onSchedule: (task: Task) => void;
  onUnschedule: (task: Task) => void;
}) {
  const t = useT();
  const sections = [
    { title: t("today.section.scheduled"), tasks: today.scheduled },
    { title: t("today.section.overdue"), tasks: today.overdue },
    { title: t("today.section.unscheduled"), tasks: today.unscheduled_in_progress },
    { title: t("today.section.completed"), tasks: today.completed },
  ].filter((section) => section.tasks.length > 0);
  const shortcuts = openCycleShortcuts(cycles);
  const destination = executionDestination(cycles, projects);

  return (
    <div className="stack">
      <PageHeader
        kicker={t("today.kicker")}
        title={formatCalendarDate(today.date)}
        actions={
          <TodayDateNav date={today.date} localToday={localToday} onChange={onDateChange} />
        }
      />
      {cycles.length === 0 ? (
        <OnboardingCard />
      ) : sections.length === 0 ? (
        <div className="stack">
          <EmptyState
            title={t("today.empty.title")}
            detail={t("today.empty.detail")}
            action={
              <Link to={destination} className="btn btn-primary">
                {t("today.empty.action")}
              </Link>
            }
          />
          {projects.length > 0 ? (
            <section className="stack" aria-label={t("today.projects.title")}>
              <h2 className="section-title">{t("today.projects.title")}</h2>
              {projects.map((project) => (
                <Link key={project.id} to={`/projects/${project.id}`} className="card card-link">
                  <div className="row">
                    <strong>{project.title}</strong>
                    <StatusBadge status={project.status} />
                  </div>
                </Link>
              ))}
            </section>
          ) : shortcuts.length > 0 ? (
            <section className="stack" aria-label={t("today.continue.title")}>
              <h2 className="section-title">{t("today.continue.title")}</h2>
              {shortcuts.map((cycle) => (
                <Link key={cycle.id} to={`/cycles/${cycle.id}`} className="card card-link">
                  <div className="row">
                    <strong>{cycle.name}</strong>
                    <StatusBadge status={cycle.status} />
                  </div>
                </Link>
              ))}
            </section>
          ) : (
            <Link to="/cycles" className="btn">
              {t("today.continue.browse")}
            </Link>
          )}
        </div>
      ) : (
        <div className="stack">
          {sections.map((section) => (
            <TaskList
              key={section.title}
              title={section.title}
              tasks={section.tasks}
              empty=""
              projectTitles={projectTitles}
              busyId={busyId}
              onStart={onStart}
              onComplete={onComplete}
              onCancel={onCancel}
              onSchedule={onSchedule}
              onUnschedule={onUnschedule}
            />
          ))}
          <div>
            <Link to={destination} className="btn">
              {t("today.choose.projects")}
            </Link>
          </div>
        </div>
      )}
    </div>
  );
}
