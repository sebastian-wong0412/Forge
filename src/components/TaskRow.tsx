import { useNavigate } from "react-router-dom";
import type { Task } from "../api/types";
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

  return (
    <div className="task-row">
      <TaskStatusIcon status={task.status} />
      <button
        type="button"
        className="btn"
        style={{ border: 0, background: "transparent", padding: 0, textAlign: "left" }}
        onClick={() => navigate(`/tasks/${task.id}`)}
      >
        <div className="task-title">{task.title}</div>
        <div className="task-meta">
          {projectTitle ? `${projectTitle} · ` : ""}
          {task.scheduled_on ? `安排于 ${task.scheduled_on}` : "未排期"}
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
