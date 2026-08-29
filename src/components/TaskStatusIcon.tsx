import type { TaskStatus } from "../api/types";
import { statusLabel } from "../lib/status";

const MARK: Record<TaskStatus, string> = {
  todo: "[ ]",
  in_progress: "[→]",
  done: "[✓]",
  cancelled: "[×]",
};

export function TaskStatusIcon({ status }: { status: TaskStatus }) {
  return (
    <span className="task-status" aria-label={statusLabel(status)}>
      {MARK[status]}
    </span>
  );
}
