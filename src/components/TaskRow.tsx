import { useNavigate } from "react-router-dom";
import type { Task } from "../api/types";
import { useT } from "../i18n";
import { TaskActions } from "./TaskActions";
import { TaskStatusIcon } from "./TaskStatusIcon";

export function TaskRow({
  task,
  projectTitle,
  busy,
  onStart,
  onComplete,
  onCancel,
  onSchedule,
  onUnschedule,
}: {
  task: Task;
  projectTitle?: string;
  busy?: boolean;
  onStart: () => void;
  onComplete: () => void;
  onCancel: () => void;
  onSchedule: () => void;
  onUnschedule: () => void;
}) {
  const navigate = useNavigate();
  const t = useT();

  return (
    <div className="task-row">
      <TaskStatusIcon status={task.status} />
      <button
        type="button"
        className="task-row-open"
        onClick={() => navigate(`/tasks/${task.id}`)}
      >
        <div className="task-title">{task.title}</div>
        <div className="task-meta">
          {projectTitle ? `${projectTitle} · ` : ""}
          {task.scheduled_on
            ? t("tasks.scheduledOn", { date: task.scheduled_on })
            : t("tasks.unscheduled")}
        </div>
      </button>
      <TaskActions
        task={task}
        busy={busy}
        onStart={onStart}
        onComplete={onComplete}
        onCancel={onCancel}
        onSchedule={onSchedule}
        onUnschedule={onUnschedule}
      />
    </div>
  );
}
