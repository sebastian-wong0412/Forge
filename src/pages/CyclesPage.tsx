import { FormEvent, useState } from "react";
import { Link } from "react-router-dom";
import { createCycle, getCycles, type Cycle } from "../api";
import { DateField } from "../components/DateField";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { NextStep } from "../components/NextStep";
import { PageHeader } from "../components/PageHeader";
import { StatusBadge } from "../components/StatusBadge";
import { useOptionalExample } from "../example/ExampleProvider";
import { useLoad } from "../hooks/useLoad";
import { useT } from "../i18n";
import { formatDisplayDate } from "../lib/dates";
import { visibleCycles } from "../lib/exampleWorkspace";

export function CyclesPage() {
  const t = useT();
  const example = useOptionalExample();
  const { data, error, loading, reload } = useLoad(getCycles, []);
  const [name, setName] = useState("");
  const [startOn, setStartOn] = useState("");
  const [endOn, setEndOn] = useState("");
  const [formError, setFormError] = useState<string | null>(null);
  const [busy, setBusy] = useState(false);
  const [created, setCreated] = useState<Cycle | null>(null);

  async function onCreate(event: FormEvent) {
    event.preventDefault();
    setBusy(true);
    setFormError(null);
    try {
      const cycle = await createCycle({ name, start_on: startOn, end_on: endOn });
      setName("");
      setStartOn("");
      setEndOn("");
      setCreated(cycle);
      await reload();
    } catch (err) {
      setFormError(err instanceof Error ? err.message : t("error.createFailed"));
    } finally {
      setBusy(false);
    }
  }

  if (loading && !data) {
    return <LoadingState label={t("cycles.loading")} />;
  }
  if (error && !data) {
    return <ErrorState message={error} onRetry={() => void reload()} />;
  }

  const cycles = example ? visibleCycles(data ?? [], example.state) : (data ?? []);

  return (
    <div className="stack">
      <PageHeader kicker={t("cycles.kicker")} title={t("cycles.title")} meta={t("cycles.meta")} />
      {cycles.length === 0 ? (
        <EmptyState title={t("cycles.empty.title")} detail={t("cycles.empty.detail")} />
      ) : null}
      {cycles.length === 0 ? (
        <ul className="muted example-list">
          <li>{t("cycles.example.q3")}</li>
          <li>{t("cycles.example.study")}</li>
          <li>{t("cycles.example.launch")}</li>
        </ul>
      ) : null}
      {created ? (
        <NextStep
          title={t("cycles.created.title")}
          detail={t("cycles.created.detail")}
          action={
            <Link to={`/cycles/${created.id}`} className="btn btn-primary">
              {t("cycles.created.action")}
            </Link>
          }
        />
      ) : null}
      <div className="stack">
        {cycles.map((cycle) => (
          <Link key={cycle.id} to={`/cycles/${cycle.id}`} className="card card-link">
            <div className="row">
              <strong className="card-title">{cycle.name}</strong>
              <StatusBadge status={cycle.status} />
            </div>
            <p className="muted">
              {formatDisplayDate(cycle.start_on)} – {formatDisplayDate(cycle.end_on)}
            </p>
          </Link>
        ))}
      </div>
      <form id="create-cycle" className="panel stack" onSubmit={onCreate}>
        <h2 className="section-title">{t("cycles.form.title")}</h2>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="cycle-name">{t("cycles.form.name")}</label>
            <input
              id="cycle-name"
              value={name}
              onChange={(event) => setName(event.target.value)}
              placeholder={t("cycles.form.name.placeholder")}
              required
            />
          </div>
          <div className="field">
            <label htmlFor="cycle-start">{t("cycles.form.start")}</label>
            <DateField id="cycle-start" value={startOn} onChange={setStartOn} required />
          </div>
          <div className="field">
            <label htmlFor="cycle-end">{t("cycles.form.end")}</label>
            <DateField id="cycle-end" value={endOn} onChange={setEndOn} required />
          </div>
        </div>
        {formError ? <ErrorState message={formError} /> : null}
        <div>
          <button type="submit" className="btn btn-primary" disabled={busy}>
            {t("cycles.form.submit")}
          </button>
        </div>
      </form>
    </div>
  );
}
