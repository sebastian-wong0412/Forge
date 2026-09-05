import { useState, type FormEvent } from "react";
import type { IsoDate, Task } from "../api/types";
import { useT } from "../i18n";
import { localCalendarDate } from "../lib/dates";
import { DateField } from "./DateField";
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
  const t = useT();
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
      setError(err instanceof Error ? err.message : t("error.scheduleFailed"));
    } finally {
      setBusy(false);
    }
  }

  return (
    <Dialog title={task.scheduled_on ? t("tasks.reschedule") : t("tasks.schedule")} onClose={onClose}>
      <form className="stack" onSubmit={submit}>
        <div className="field">
          <label htmlFor="scheduled-on">{t("tasks.date")}</label>
          <DateField id="scheduled-on" value={date} onChange={setDate} required />
        </div>
        {error ? <p className="state error">{error}</p> : null}
        <div className="row">
          <button type="submit" className="btn btn-primary" disabled={busy}>
            {t("common.save")}
          </button>
          <button type="button" className="btn" onClick={onClose}>
            {t("common.close")}
          </button>
        </div>
      </form>
    </Dialog>
  );
}
