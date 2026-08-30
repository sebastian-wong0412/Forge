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
import { useT } from "../i18n";

export function ProjectDetailPage() {
  const t = useT();
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
      setError(err instanceof Error ? err.message : t("error.projectUpdateFailed"));
    }
  }

  if (project.loading && !project.data) {
    return <LoadingState label={t("projects.loading")} />;
  }
  if (project.error && !project.data) {
    return <ErrorState message={project.error} onRetry={() => void project.reload()} />;
  }
  if (!project.data) {
    return <ErrorState message={t("error.projectNotFound")} />;
  }

  const canCreateTask = project.data.status === "draft" || project.data.status === "active";

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: t("cycles.breadcrumb"), to: "/cycles" },
          ...(cycle.data ? [{ label: cycle.data.name, to: `/cycles/${cycle.data.id}` }] : []),
          ...(objective.data
            ? [{ label: objective.data.title, to: `/objectives/${objective.data.id}` }]
            : []),
          { label: project.data.title },
        ]}
      />
      <PageHeader
        kicker={t("projects.kicker")}
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
                {t("common.start")}
              </button>
            ) : null}
            {project.data.status === "active" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => completeProject(projectId))}
              >
                {t("common.complete")}
              </button>
            ) : null}
            {project.data.status !== "archived" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => archiveProject(projectId))}
              >
                {t("common.archive")}
              </button>
            ) : null}
          </>
        }
      />
      {error ? <ErrorState message={error} /> : null}
      {mutations.error ? <ErrorState message={mutations.error} /> : null}
      {tasks.loading && !tasks.data ? <LoadingState label={t("tasks.loading")} /> : null}
      {tasks.error && !tasks.data ? <ErrorState message={tasks.error} /> : null}
      {tasks.data && tasks.data.length === 0 ? (
        <EmptyState title={t("tasks.empty.title")} detail={t("tasks.empty.detail")} />
      ) : null}
      {tasks.data && tasks.data.length > 0 ? (
        <TaskList
          title={t("tasks.section")}
          tasks={tasks.data}
          empty={t("tasks.empty.title")}
          projectTitle={project.data.title}
          busyId={mutations.busyId}
          onStart={mutations.start}
          onComplete={mutations.complete}
          onCancel={mutations.cancel}
          onSchedule={mutations.setScheduling}
          onUnschedule={mutations.unschedule}
        />
      ) : null}
      <CreateTaskForm projectId={projectId} disabled={!canCreateTask} onCreated={tasks.reload} />
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
  const t = useT();
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
      setError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <form className="panel stack" onSubmit={onSubmit}>
      <h2 className="section-title">{t("tasks.form.title")}</h2>
      <div className="form-grid">
        <div className="field">
          <label htmlFor="task-title">{t("tasks.form.name")}</label>
          <input
            id="task-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            required
            disabled={disabled}
          />
        </div>
        <div className="field">
          <label htmlFor="task-description">{t("tasks.form.description")}</label>
          <input
            id="task-description"
            value={description}
            onChange={(event) => setDescription(event.target.value)}
            disabled={disabled}
          />
        </div>
        <div className="field">
          <label htmlFor="task-scheduled">{t("tasks.form.scheduled")}</label>
          <input
            id="task-scheduled"
            type="date"
            value={scheduledOn}
            onChange={(event) => setScheduledOn(event.target.value)}
            disabled={disabled}
          />
        </div>
      </div>
      {disabled ? <p className="muted">{t("tasks.form.disabled")}</p> : null}
      {error ? <ErrorState message={error} /> : null}
      <div>
        <button type="submit" className="btn btn-primary" disabled={disabled}>
          {t("tasks.form.submit")}
        </button>
      </div>
    </form>
  );
}
