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
import { formatTimestamp } from "../lib/dates";

export function TaskDetailPage() {
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
    return <LoadingState label="正在加载任务…" />;
  }
  if (task.error && !task.data) {
    return <ErrorState message={task.error} onRetry={() => void task.reload()} />;
  }
  const current = task.data;
  if (!current) {
    return <ErrorState message="未找到该任务。" />;
  }

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: "周期", to: "/cycles" },
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
        kicker="任务"
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
        {current.description ? <p>{current.description}</p> : <p className="muted">暂无说明</p>}
        <p className="muted">
          安排于 {current.scheduled_on ?? "—"} · 创建于 {formatTimestamp(current.created_at)}
          {current.completed_at ? ` · 完成于 ${formatTimestamp(current.completed_at)}` : ""}
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
