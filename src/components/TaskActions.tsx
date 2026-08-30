import type { Task } from "../api/types";

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
  return (
    <div className="row" onClick={(event) => event.stopPropagation()}>
      {task.status === "todo" ? (
        <button type="button" className="btn btn-primary" disabled={busy} onClick={onStart}>
          开始
        </button>
      ) : null}
      {task.status === "in_progress" ? (
        <button type="button" className="btn btn-primary" disabled={busy} onClick={onComplete}>
          完成
        </button>
      ) : null}
      {task.status === "todo" || task.status === "in_progress" ? (
        <button type="button" className="btn btn-danger" disabled={busy} onClick={onCancel}>
          取消
        </button>
      ) : null}
      {task.status === "todo" || task.status === "in_progress" ? (
        <>
          <button type="button" className="btn" disabled={busy} onClick={onSchedule}>
            {task.scheduled_on ? "改期" : "安排日期"}
          </button>
          {task.scheduled_on ? (
            <button type="button" className="btn" disabled={busy} onClick={onUnschedule}>
              取消安排
            </button>
          ) : null}
        </>
      ) : null}
    </div>
  );
}
