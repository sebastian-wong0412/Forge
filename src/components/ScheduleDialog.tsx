import { useState, type FormEvent } from "react";
import type { IsoDate, Task } from "../api/types";
import { localCalendarDate } from "../lib/dates";
import { Dialog } from "./Dialog";

export function ScheduleDialog({
  task,
  onClose,
  onSave,
}: {
  task: Task;
  onClose: () => void;
  onSave: (date: IsoDate) => Promise<void>;
}) {
  const [date, setDate] = useState(task.scheduled_on ?? localCalendarDate());
  const [error, setError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);

  async function submit(event: FormEvent) {
    event.preventDefault();
    setBusy(true);
    setError(null);
    try {
      await onSave(date);
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : "无法安排日期，请稍后重试。");
    } finally {
      setBusy(false);
    }
  }

  return (
    <Dialog title={task.scheduled_on ? "改期" : "安排日期"} onClose={onClose}>
      <form className="stack" onSubmit={submit}>
        <div className="field">
          <label htmlFor="scheduled-on">日期</label>
          <input
            id="scheduled-on"
            type="date"
            required
            value={date}
            onChange={(event) => setDate(event.target.value)}
          />
        </div>
        {error ? <p className="state error">{error}</p> : null}
        <div className="row">
          <button type="submit" className="btn btn-primary" disabled={busy}>
            保存
          </button>
          <button type="button" className="btn" onClick={onClose}>
            关闭
          </button>
        </div>
      </form>
    </Dialog>
  );
}
