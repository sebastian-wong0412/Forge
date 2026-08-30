import { useParams } from "react-router-dom";
import { getCycle, getObjective, getProject, getTask } from "../api";
import { Breadcrumbs } from "../components/Breadcrumbs";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { PageHeader } from "../components/PageHeader";
import { ScheduleDialog } from "../components/ScheduleDialog";
import { StatusBadge } from "../components/StatusBadge";
import { TaskActions } from "../components/TaskActions";
import { TaskStatusIcon } from "../components/TaskStatusIcon";
import { useLoad } from "../hooks/useLoad";
import { useTaskMutations } from "../hooks/useTaskMutations";
import { useSettings } from "../i18n";
import { formatTimestamp } from "../lib/dates";

export function TaskDetailPage() {
  const { t, locale } = useSettings();
  const { taskId = "" } = useParams();
  const task = useLoad(() => getTask(taskId), [taskId]);
  const project = useLoad(
    () => (task.data ? getProject(task.data.project_id) : Promise.resolve(null)),
    [task.data?.project_id],
  );
  const objective = useLoad(
    () => (project.data ? getObjective(project.data.objective_id) : Promise.resolve(null)),
    [project.data?.objective_id],
  );
  const cycle = useLoad(
    () => (objective.data ? getCycle(objective.data.cycle_id) : Promise.resolve(null)),
    [objective.data?.cycle_id],
  );
  const mutations = useTaskMutations(task.reload);

  if (task.loading && !task.data) {
    return <LoadingState label={t("tasks.loading")} />;
  }
  if (task.error && !task.data) {
    return <ErrorState message={task.error} onRetry={() => void task.reload()} />;
  }
  const current = task.data;
  if (!current) {
    return <ErrorState message={t("error.taskNotFound")} />;
  }

  const completed = current.completed_at
    ? ` · ${t("tasks.completedAt", { time: formatTimestamp(current.completed_at, locale) })}`
    : "";

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: t("cycles.breadcrumb"), to: "/cycles" },
          ...(cycle.data ? [{ label: cycle.data.name, to: `/cycles/${cycle.data.id}` }] : []),
          ...(objective.data
            ? [{ label: objective.data.title, to: `/objectives/${objective.data.id}` }]
            : []),
          ...(project.data
            ? [{ label: project.data.title, to: `/projects/${project.data.id}` }]
            : []),
          { label: current.title },
        ]}
      />
      <PageHeader
        kicker={t("tasks.kicker")}
        title={current.title}
        meta={
          <span className="row">
            <TaskStatusIcon status={current.status} />
            <StatusBadge status={current.status} />
          </span>
        }
      />
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      <section className="card stack">
        {current.description ? <p>{current.description}</p> : <p className="muted">{t("common.noDescription")}</p>}
        <p className="muted">
          {t("tasks.meta", {
            scheduled: current.scheduled_on ?? t("common.emDash"),
            created: formatTimestamp(current.created_at, locale),
            completed,
          })}
        </p>
        <TaskActions
          task={current}
          busy={mutations.busyId === current.id}
          onStart={() => void mutations.start(current)}
          onComplete={() => void mutations.complete(current)}
          onCancel={() => void mutations.cancel(current)}
          onSchedule={() => mutations.setScheduling(current)}
          onUnschedule={() => void mutations.unschedule(current)}
        />
      </section>
      {mutations.scheduling ? (
        <ScheduleDialog
          task={mutations.scheduling}
          onClose={() => mutations.setScheduling(null)}
          onSave={mutations.saveSchedule}
        />
      ) : null}
    </div>
  );
}
