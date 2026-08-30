import { FormEvent, useState } from "react";
import { createCheckIn, getCheckIns } from "../api";
import type { MilestoneState, ProgressKind } from "../api/types";
import { useLoad } from "../hooks/useLoad";
import { useT } from "../i18n";
import { localCalendarDate } from "../lib/dates";
import { statusLabel } from "../lib/status";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";

const MILESTONE_STATES: MilestoneState[] = ["not_started", "in_progress", "achieved"];

export function CheckInPanel({
  keyResultId,
  progressKind,
  onKeyResultChanged,
}: {
  keyResultId: string;
  progressKind: ProgressKind;
  onKeyResultChanged: () => Promise<void>;
}) {
  const t = useT();
  const history = useLoad(() => getCheckIns(keyResultId), [keyResultId]);
  const [value, setValue] = useState("");
  const [state, setState] = useState<MilestoneState>("in_progress");
  const [note, setNote] = useState("");
  const [checkedOn, setCheckedOn] = useState(localCalendarDate());
  const [error, setError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await createCheckIn(keyResultId, {
        value:
          progressKind === "numeric" || progressKind === "percentage" ? Number(value) : null,
        state: progressKind === "milestone" ? state : null,
        note: note || null,
        checked_on: checkedOn,
      });
      setValue("");
      setNote("");
      await history.reload();
      await onKeyResultChanged();
    } catch (err) {
      setError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <div className="stack">
      {history.loading && !history.data ? <LoadingState label={t("checkIn.loading")} /> : null}
      {history.error ? <ErrorState message={history.error} /> : null}
      {history.data && history.data.length === 0 ? <EmptyState title={t("checkIn.empty")} /> : null}
      {history.data && history.data.length > 0 ? (
        <div>
          <h3 className="section-title">{t("checkIn.section")}</h3>
          {history.data.map((checkIn) => (
            <div key={checkIn.id} className="check-in">
              <strong>
                {checkIn.state
                  ? statusLabel(checkIn.state)
                  : checkIn.value !== null
                    ? checkIn.value
                    : checkIn.note}
              </strong>{" "}
              <span className="muted">{checkIn.checked_on}</span>
              {checkIn.note && (checkIn.value !== null || checkIn.state) ? (
                <p>{checkIn.note}</p>
              ) : null}
            </div>
          ))}
        </div>
      ) : null}
      <form className="stack" onSubmit={onSubmit}>
        <div className="form-grid">
          {progressKind === "numeric" || progressKind === "percentage" ? (
            <div className="field">
              <label htmlFor={`checkin-value-${keyResultId}`}>
                {progressKind === "percentage" ? t("checkIn.percentage") : t("checkIn.value")}
              </label>
              <input
                id={`checkin-value-${keyResultId}`}
                type="number"
                step="any"
                value={value}
                onChange={(event) => setValue(event.target.value)}
                required
              />
            </div>
          ) : null}
          {progressKind === "milestone" ? (
            <div className="field">
              <label htmlFor={`checkin-state-${keyResultId}`}>{t("checkIn.state")}</label>
              <select
                id={`checkin-state-${keyResultId}`}
                value={state}
                onChange={(event) => setState(event.target.value as MilestoneState)}
              >
                {MILESTONE_STATES.map((item) => (
                  <option key={item} value={item}>
                    {statusLabel(item)}
                  </option>
                ))}
              </select>
            </div>
          ) : null}
          <div className="field">
            <label htmlFor={`checkin-date-${keyResultId}`}>{t("checkIn.date")}</label>
            <input
              id={`checkin-date-${keyResultId}`}
              type="date"
              value={checkedOn}
              onChange={(event) => setCheckedOn(event.target.value)}
              required
            />
          </div>
          <div className="field">
            <label htmlFor={`checkin-note-${keyResultId}`}>
              {progressKind === "qualitative" ? t("checkIn.noteRequired") : t("checkIn.note")}
            </label>
            <input
              id={`checkin-note-${keyResultId}`}
              value={note}
              onChange={(event) => setNote(event.target.value)}
              required={progressKind === "qualitative"}
            />
          </div>
        </div>
        {error ? <ErrorState message={error} /> : null}
        <div>
          <button type="submit" className="btn">
            {t("checkIn.submit")}
          </button>
        </div>
      </form>
    </div>
  );
}
