import { useState } from "react";
import { useT, type MessageKey } from "../i18n";
import { Dialog } from "./Dialog";

const STEPS: { title: MessageKey; detail: MessageKey; note?: MessageKey }[] = [
  { title: "tour.vision.title", detail: "tour.vision.detail", note: "tour.vision.note" },
  { title: "tour.objective.title", detail: "tour.objective.detail" },
  { title: "tour.keyResult.title", detail: "tour.keyResult.detail" },
  { title: "tour.project.title", detail: "tour.project.detail" },
  { title: "tour.task.title", detail: "tour.task.detail" },
  { title: "tour.today.title", detail: "tour.today.detail" },
];

export function ConceptTour({ onClose }: { onClose: () => void }) {
  const t = useT();
  const [index, setIndex] = useState(0);
  const step = STEPS[index];
  const last = index === STEPS.length - 1;

  return (
    <Dialog title={t("tour.title")} onClose={onClose}>
      <div className="stack">
        <p className="muted">
          {index + 1} / {STEPS.length}
        </p>
        <p>
          <strong>{t(step.title)}</strong>
        </p>
        <p className="muted">{t(step.detail)}</p>
        {step.note ? <p className="muted">{t(step.note)}</p> : null}
        <div className="row">
          {index > 0 ? (
            <button type="button" className="btn" onClick={() => setIndex(index - 1)}>
              {t("tour.back")}
            </button>
          ) : null}
          {last ? (
            <button type="button" className="btn btn-primary" onClick={onClose}>
              {t("tour.done")}
            </button>
          ) : (
            <button type="button" className="btn btn-primary" onClick={() => setIndex(index + 1)}>
              {t("tour.next")}
            </button>
          )}
        </div>
      </div>
    </Dialog>
  );
}
