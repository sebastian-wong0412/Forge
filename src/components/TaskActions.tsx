import type { Task } from "../api/types";
import { useT } from "../i18n";

export function TaskActions({
  task,
  busy = false,
  onStart,
  onComplete,
  onCancel,
  onSchedule,
  onUnschedule,
}: {
  task: Task;
  busy?: boolean;
  onStart: () => void;
  onComplete: () => void;
  onCancel: () => void;
  onSchedule: () => void;
  onUnschedule: () => void;
}) {
  const t = useT();

  return (
    <div className="row" onClick={(event) => event.stopPropagation()}>
      {task.status === "todo" ? (
        <button type="button" className="btn btn-primary" disabled={busy} onClick={onStart}>
          {t("common.start")}
        </button>
      ) : null}
      {task.status === "in_progress" ? (
        <button type="button" className="btn btn-primary" disabled={busy} onClick={onComplete}>
          {t("common.complete")}
        </button>
      ) : null}
      {task.status === "todo" || task.status === "in_progress" ? (
        <button type="button" className="btn btn-danger" disabled={busy} onClick={onCancel}>
          {t("common.cancel")}
        </button>
      ) : null}
      {task.status === "todo" || task.status === "in_progress" ? (
        <>
          <button type="button" className="btn" disabled={busy} onClick={onSchedule}>
            {task.scheduled_on ? t("tasks.reschedule") : t("tasks.schedule")}
          </button>
          {task.scheduled_on ? (
            <button type="button" className="btn" disabled={busy} onClick={onUnschedule}>
              {t("tasks.unschedule")}
            </button>
          ) : null}
        </>
      ) : null}
    </div>
  );
}
