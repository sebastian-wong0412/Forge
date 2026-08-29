import { FormEvent, useState } from "react";
import { useParams } from "react-router-dom";
import {
  activateProject,
  archiveProject,
  completeProject,
  createTask,
  getCycle,
  getObjective,
  getProject,
  getTasks,
} from "../api";
import { Breadcrumbs } from "../components/Breadcrumbs";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { PageHeader } from "../components/PageHeader";
import { ScheduleDialog } from "../components/ScheduleDialog";
import { StatusBadge } from "../components/StatusBadge";
import { TaskList } from "../components/TaskList";
import { useLoad } from "../hooks/useLoad";
import { useTaskMutations } from "../hooks/useTaskMutations";

export function ProjectDetailPage() {
  const { projectId = "" } = useParams();
  const project = useLoad(() => getProject(projectId), [projectId]);
  const objective = useLoad(
    () => (project.data ? getObjective(project.data.objective_id) : Promise.resolve(null)),
    [project.data?.objective_id],
  );
  const cycle = useLoad(
    () => (objective.data ? getCycle(objective.data.cycle_id) : Promise.resolve(null)),
    [objective.data?.cycle_id],
  );
  const tasks = useLoad(() => getTasks(projectId), [projectId]);
  const [error, setError] = useState<string | null>(null);
  const mutations = useTaskMutations(tasks.reload);

  async function run(action: () => Promise<unknown>) {
    setError(null);
    try {
      await action();
      await project.reload();
    } catch (err) {
      setError(err instanceof Error ? err.message : "无法更新项目，请稍后重试。");
    }
  }

  if (project.loading && !project.data) {
    return <LoadingState label="正在加载项目…" />;
  }
  if (project.error && !project.data) {
    return <ErrorState message={project.error} onRetry={() => void project.reload()} />;
  }
  if (!project.data) {
    return <ErrorState message="未找到该项目。" />;
  }

  const canCreateTask = project.data.status === "active";

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: "周期", to: "/cycles" },
          ...(cycle.data ? [{ label: cycle.data.name, to: `/cycles/${cycle.data.id}` }] : []),
          ...(objective.data
            ? [{ label: objective.data.title, to: `/objectives/${objective.data.id}` }]
            : []),
          { label: project.data.title },
        ]}
      />
      <PageHeader
        kicker="项目"
        title={project.data.title}
        meta={
          <>
            <StatusBadge status={project.data.status} />
            {project.data.description ? ` · ${project.data.description}` : null}
          </>
        }
        actions={
          <>
            {project.data.status === "draft" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => activateProject(projectId))}
              >
                激活
              </button>
            ) : null}
            {project.data.status === "active" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => completeProject(projectId))}
              >
                完成
              </button>
            ) : null}
            {project.data.status !== "archived" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => archiveProject(projectId))}
              >
                归档
              </button>
            ) : null}
          </>
        }
      />
      {error ? <ErrorState message={error} /> : null}
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      {tasks.loading && !tasks.data ? <LoadingState label="正在加载任务…" /> : null}
      {tasks.error && !tasks.data ? <ErrorState message={tasks.error} /> : null}
      {tasks.data && tasks.data.length === 0 ? (
        <EmptyState title="还没有任务" detail="任务是可以直接开始执行的具体工作。" />
      ) : null}
      {tasks.data && tasks.data.length > 0 ? (
        <TaskList
          title="任务"
          tasks={tasks.data}
          empty="还没有任务"
          projectTitle={project.data.title}
          busyId={mutations.busyId}
          onStart={mutations.start}
          onComplete={mutations.complete}
          onCancel={mutations.cancel}
          onSchedule={mutations.setScheduling}
          onUnschedule={mutations.unschedule}
        />
      ) : null}
      <CreateTaskForm
        projectId={projectId}
        disabled={!canCreateTask}
        onCreated={tasks.reload}
      />
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

function CreateTaskForm({
  projectId,
  disabled,
  onCreated,
}: {
  projectId: string;
  disabled: boolean;
  onCreated: () => Promise<void>;
}) {
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [scheduledOn, setScheduledOn] = useState("");
  const [error, setError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await createTask(projectId, {
        title,
        description: description || null,
        scheduled_on: scheduledOn || null,
      });
      setTitle("");
      setDescription("");
      setScheduledOn("");
      await onCreated();
    } catch (err) {
      setError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <form className="panel stack" onSubmit={onSubmit}>
      <h2 className="section-title">创建任务</h2>
      <div className="form-grid">
        <div className="field">
          <label htmlFor="task-title">标题</label>
          <input
            id="task-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            required
            disabled={disabled}
          />
        </div>
        <div className="field">
          <label htmlFor="task-description">说明</label>
          <input
            id="task-description"
            value={description}
            onChange={(event) => setDescription(event.target.value)}
            disabled={disabled}
          />
        </div>
        <div className="field">
          <label htmlFor="task-scheduled">安排日期</label>
          <input
            id="task-scheduled"
            type="date"
            value={scheduledOn}
            onChange={(event) => setScheduledOn(event.target.value)}
            disabled={disabled}
          />
        </div>
      </div>
      {disabled ? (
        <p className="muted">请先激活项目，然后才能添加任务。</p>
      ) : null}
      {error ? <ErrorState message={error} /> : null}
      <div>
        <button type="submit" className="btn btn-primary" disabled={disabled}>
          创建任务
        </button>
      </div>
    </form>
  );
}
