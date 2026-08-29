import { FormEvent, useState } from "react";
import { createCheckIn, getCheckIns } from "../api";
import { useLoad } from "../hooks/useLoad";
import { localCalendarDate } from "../lib/dates";
import { EmptyState } from "./EmptyState";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";

export function CheckInPanel({
  keyResultId,
  onKeyResultChanged,
}: {
  keyResultId: string;
  onKeyResultChanged: () => Promise<void>;
}) {
  const history = useLoad(() => getCheckIns(keyResultId), [keyResultId]);
  const [value, setValue] = useState("");
  const [note, setNote] = useState("");
  const [checkedOn, setCheckedOn] = useState(localCalendarDate());
  const [error, setError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setError(null);
    try {
      await createCheckIn(keyResultId, {
        value: Number(value),
        note: note || null,
        checked_on: checkedOn,
      });
      setValue("");
      setNote("");
      await history.reload();
      await onKeyResultChanged();
    } catch (err) {
      setError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <div className="stack">
      {history.loading && !history.data ? <LoadingState label="正在加载进展记录…" /> : null}
      {history.error ? <ErrorState message={history.error} /> : null}
      {history.data && history.data.length === 0 ? <EmptyState title="还没有进展记录。" /> : null}
      {history.data && history.data.length > 0 ? (
        <div>
          <h3 className="section-title">进展记录</h3>
          {history.data.map((checkIn) => (
            <div key={checkIn.id} className="check-in">
              <strong>{checkIn.value}</strong>{" "}
              <span className="muted">{checkIn.checked_on}</span>
              {checkIn.note ? <p>{checkIn.note}</p> : null}
            </div>
          ))}
        </div>
      ) : null}
      <form className="stack" onSubmit={onSubmit}>
        <div className="form-grid">
          <div className="field">
            <label htmlFor={`checkin-value-${keyResultId}`}>数值</label>
            <input
              id={`checkin-value-${keyResultId}`}
              type="number"
              step="any"
              value={value}
              onChange={(event) => setValue(event.target.value)}
              required
            />
          </div>
          <div className="field">
            <label htmlFor={`checkin-date-${keyResultId}`}>记录日期</label>
            <input
              id={`checkin-date-${keyResultId}`}
              type="date"
              value={checkedOn}
              onChange={(event) => setCheckedOn(event.target.value)}
              required
            />
          </div>
          <div className="field">
            <label htmlFor={`checkin-note-${keyResultId}`}>备注</label>
            <input
              id={`checkin-note-${keyResultId}`}
              value={note}
              onChange={(event) => setNote(event.target.value)}
            />
          </div>
        </div>
        {error ? <ErrorState message={error} /> : null}
        <div>
          <button type="submit" className="btn">
            添加进展记录
          </button>
        </div>
      </form>
    </div>
  );
}
