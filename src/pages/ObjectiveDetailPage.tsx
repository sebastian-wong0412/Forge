import { FormEvent, useState } from "react";
import { Link, useParams } from "react-router-dom";
import {
  activateKeyResult,
  activateObjective,
  archiveObjective,
  completeKeyResult,
  completeObjective,
  activateProject,
  createKeyResult,
  createProject,
  getCycle,
  getKeyResults,
  getObjective,
  getProjects,
  type KeyResult,
  type ProgressKind,
  type Project,
} from "../api";
import { BackButton } from "../components/BackButton";
import { Breadcrumbs } from "../components/Breadcrumbs";
import { CheckInPanel } from "../components/CheckInPanel";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { PageHeader } from "../components/PageHeader";
import { StatusBadge } from "../components/StatusBadge";
import { useLoad } from "../hooks/useLoad";
import { useT, type TranslateFn, type MessageKey } from "../i18n";
import { dateRange } from "../lib/dates";
import { statusLabel } from "../lib/status";

const KR_TITLE_PLACEHOLDER: Record<ProgressKind, MessageKey> = {
  numeric: "keyResults.form.title.placeholder.numeric",
  percentage: "keyResults.form.title.placeholder.percentage",
  milestone: "keyResults.form.title.placeholder.milestone",
  qualitative: "keyResults.form.title.placeholder.qualitative",
};

export function ObjectiveDetailPage() {
  const t = useT();
  const { objectiveId = "" } = useParams();
  const objective = useLoad(() => getObjective(objectiveId), [objectiveId]);
  const cycle = useLoad(
    () => (objective.data ? getCycle(objective.data.cycle_id) : Promise.resolve(null)),
    [objective.data?.cycle_id],
  );
  const keyResults = useLoad(() => getKeyResults(objectiveId), [objectiveId]);
  const projects = useLoad(() => getProjects(objectiveId), [objectiveId]);
  const [error, setError] = useState<string | null>(null);

  async function refresh() {
    await Promise.all([objective.reload(), keyResults.reload(), projects.reload()]);
  }

  async function run(action: () => Promise<unknown>) {
    setError(null);
    try {
      await action();
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : t("error.objectiveUpdateFailed"));
    }
  }

  if (objective.loading && !objective.data) {
    return <LoadingState label={t("objectives.loading")} />;
  }
  if (objective.error && !objective.data) {
    return <ErrorState message={objective.error} onRetry={() => void objective.reload()} />;
  }
  if (!objective.data) {
    return <ErrorState message={t("error.objectiveNotFound")} />;
  }

  const objectiveDates = dateRange(objective.data.start_on, objective.data.end_on);

  return (
    <div className="stack">
      <BackButton fallback={`/cycles/${objective.data.cycle_id}`} />
      <Breadcrumbs
        items={[
          { label: t("cycles.breadcrumb"), to: "/cycles" },
          ...(cycle.data ? [{ label: cycle.data.name, to: `/cycles/${cycle.data.id}` }] : []),
          { label: objective.data.title },
        ]}
      />
      <PageHeader
        kicker={t("objectives.kicker")}
        title={objective.data.title}
        meta={
          <>
            {objectiveDates ? `${objectiveDates} · ` : null}
            <StatusBadge status={objective.data.status} />
            {objective.data.description ? ` · ${objective.data.description}` : null}
          </>
        }
        actions={
          <>
            {objective.data.status === "draft" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => activateObjective(objectiveId))}
              >
                {t("common.start")}
              </button>
            ) : null}
            {objective.data.status === "active" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => completeObjective(objectiveId))}
              >
                {t("common.complete")}
              </button>
            ) : null}
            {objective.data.status !== "archived" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => archiveObjective(objectiveId))}
              >
                {t("common.archive")}
              </button>
            ) : null}
          </>
        }
      />
      {error ? <ErrorState message={error} /> : null}
      <KeyResultsSection
        objectiveId={objectiveId}
        keyResults={keyResults.data ?? []}
        loading={keyResults.loading}
        error={keyResults.error}
        onChanged={refresh}
      />
      <ProjectsSection
        objectiveId={objectiveId}
        projects={projects.data ?? []}
        loading={projects.loading}
        error={projects.error}
        onCreated={refresh}
      />
    </div>
  );
}

function keyResultSummary(keyResult: KeyResult, percent: string | null, t: TranslateFn): string {
  if (keyResult.progress_kind === "milestone") {
    return `${statusLabel(keyResult.current_state ?? "not_started")}${percent ? t("keyResults.summary.percent", { percent }) : ""}`;
  }
  if (keyResult.progress_kind === "qualitative") {
    return keyResult.latest_note ?? t("keyResults.noProgress");
  }
  const current = keyResult.current_value ?? t("common.emDash");
  const unit = keyResult.unit ? ` ${keyResult.unit}` : "";
  const start = keyResult.start_value ?? t("common.emDash");
  const target =
    keyResult.target_value !== null
      ? t("keyResults.summary.target", { target: keyResult.target_value })
      : "";
  return t("keyResults.summary.current", {
    current,
    unit,
    start,
    target,
    percent: percent ? t("keyResults.summary.percent", { percent }) : "",
  });
}

function KeyResultsSection({
  objectiveId,
  keyResults,
  loading,
  error,
  onChanged,
}: {
  objectiveId: string;
  keyResults: KeyResult[];
  loading: boolean;
  error: string | null;
  onChanged: () => Promise<void>;
}) {
  const t = useT();
  const [title, setTitle] = useState("");
  const [progressKind, setProgressKind] = useState<ProgressKind>("numeric");
  const [startValue, setStartValue] = useState("0");
  const [targetValue, setTargetValue] = useState("");
  const [unit, setUnit] = useState("");
  const [formError, setFormError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setFormError(null);
    try {
      await createKeyResult(objectiveId, {
        title,
        progress_kind: progressKind,
        start_value:
          progressKind === "numeric" || progressKind === "percentage" ? Number(startValue) : null,
        target_value:
          progressKind === "numeric" || progressKind === "percentage"
            ? targetValue === ""
              ? null
              : Number(targetValue)
            : null,
        unit: progressKind === "numeric" ? unit || null : null,
      });
      setTitle("");
      setStartValue("0");
      setTargetValue("");
      setUnit("");
      await onChanged();
    } catch (err) {
      setFormError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">{t("keyResults.section")}</h2>
      {loading && keyResults.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {keyResults.length === 0 && !loading ? (
        <EmptyState
          title={t("keyResults.empty.title")}
          detail={t("keyResults.empty.detail")}
          action={
            <a href="#create-key-result" className="btn btn-primary">
              {t("keyResults.empty.action")}
            </a>
          }
        />
      ) : null}
      {keyResults.map((keyResult) => (
        <KeyResultCard key={keyResult.id} keyResult={keyResult} onChanged={onChanged} />
      ))}
      <form id="create-key-result" className="panel stack" onSubmit={onSubmit}>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="kr-title">{t("keyResults.form.title")}</label>
            <input
              id="kr-title"
              value={title}
              onChange={(event) => setTitle(event.target.value)}
              placeholder={t(KR_TITLE_PLACEHOLDER[progressKind])}
              required
            />
          </div>
          <div className="field">
            <label htmlFor="kr-kind">{t("keyResults.form.kind")}</label>
            <select
              id="kr-kind"
              value={progressKind}
              onChange={(event) => setProgressKind(event.target.value as ProgressKind)}
            >
              <option value="numeric">{t("keyResults.kind.numeric")}</option>
              <option value="percentage">{t("keyResults.kind.percentage")}</option>
              <option value="milestone">{t("keyResults.kind.milestone")}</option>
              <option value="qualitative">{t("keyResults.kind.qualitative")}</option>
            </select>
          </div>
          {progressKind === "numeric" || progressKind === "percentage" ? (
            <>
              <div className="field">
                <label htmlFor="kr-start">{t("keyResults.form.start")}</label>
                <input
                  id="kr-start"
                  type="number"
                  step="any"
                  value={startValue}
                  onChange={(event) => setStartValue(event.target.value)}
                  required
                />
              </div>
              <div className="field">
                <label htmlFor="kr-target">{t("keyResults.form.target")}</label>
                <input
                  id="kr-target"
                  type="number"
                  step="any"
                  value={targetValue}
                  onChange={(event) => setTargetValue(event.target.value)}
                  required={progressKind === "percentage"}
                />
              </div>
            </>
          ) : null}
          {progressKind === "numeric" ? (
            <div className="field">
              <label htmlFor="kr-unit">{t("keyResults.form.unit")}</label>
              <input id="kr-unit" value={unit} onChange={(event) => setUnit(event.target.value)} />
            </div>
          ) : null}
        </div>
        {formError ? <ErrorState message={formError} /> : null}
        <div>
          <button type="submit" className="btn btn-primary">
            {t("keyResults.form.submit")}
          </button>
        </div>
      </form>
    </section>
  );
}

function KeyResultCard({
  keyResult,
  onChanged,
}: {
  keyResult: KeyResult;
  onChanged: () => Promise<void>;
}) {
  const t = useT();
  const percent =
    keyResult.progress === null ? null : `${Math.round(keyResult.progress * 100)}%`;

  return (
    <article className="card stack">
      <div className="row">
        <strong>{keyResult.title}</strong>
        <StatusBadge status={keyResult.status} />
      </div>
      {keyResult.description ? <p>{keyResult.description}</p> : null}
      <p className="muted">{keyResultSummary(keyResult, percent, t)}</p>
      {keyResult.progress !== null ? (
        <div
          className="progress"
          role="progressbar"
          aria-label={t("keyResults.progressLabel")}
          aria-valuemin={0}
          aria-valuemax={100}
          aria-valuenow={Math.round(keyResult.progress * 100)}
        >
          <span style={{ width: `${Math.min(100, Math.max(0, keyResult.progress * 100))}%` }} />
        </div>
      ) : null}
      <div className="row">
        {keyResult.status === "draft" ? (
          <button
            type="button"
            className="btn"
            onClick={() => void activateKeyResult(keyResult.id).then(onChanged)}
          >
            {t("common.start")}
          </button>
        ) : null}
        {keyResult.status === "active" ? (
          <button
            type="button"
            className="btn"
            onClick={() => void completeKeyResult(keyResult.id).then(onChanged)}
          >
            {t("keyResults.complete")}
          </button>
        ) : null}
      </div>
      <CheckInPanel
        keyResultId={keyResult.id}
        progressKind={keyResult.progress_kind}
        onKeyResultChanged={onChanged}
      />
    </article>
  );
}

function ProjectsSection({
  objectiveId,
  projects,
  loading,
  error,
  onCreated,
}: {
  objectiveId: string;
  projects: Awaited<ReturnType<typeof getProjects>>;
  loading: boolean;
  error: string | null;
  onCreated: () => Promise<void>;
}) {
  const t = useT();
  const [title, setTitle] = useState("");
  const [formError, setFormError] = useState<string | null>(null);
  const [draftCreated, setDraftCreated] = useState<Project | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setFormError(null);
    try {
      const created = await createProject(objectiveId, { title });
      setTitle("");
      setDraftCreated(created);
      await onCreated();
    } catch (err) {
      setFormError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">{t("projects.section")}</h2>
      {loading && projects.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {projects.length === 0 && !loading ? (
        <EmptyState
          title={t("projects.empty.title")}
          detail={t("projects.empty.detail")}
          action={
            <a href="#create-project" className="btn btn-primary">
              {t("projects.empty.action")}
            </a>
          }
        />
      ) : null}
      {projects.map((project) => (
        <Link key={project.id} to={`/projects/${project.id}`} className="card card-link">
          <div className="row">
            <strong>{project.title}</strong>
            <StatusBadge status={project.status} />
          </div>
          {project.description ? <p className="muted">{project.description}</p> : null}
        </Link>
      ))}
      <form id="create-project" className="panel row" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="project-title">{t("projects.form.label")}</label>
          <input
            id="project-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            placeholder={t("projects.form.placeholder")}
            required
          />
        </div>
        <button type="submit" className="btn btn-primary">
          {t("common.add")}
        </button>
      </form>
      {draftCreated && draftCreated.status === "draft" ? (
        <div className="panel next-step">
          <p>
            <strong>{t("projects.created.title")}</strong>
          </p>
          <p>{t("projects.created.detail")}</p>
          <div className="row">
            <button
              type="button"
              className="btn btn-primary"
              onClick={() =>
                void activateProject(draftCreated.id)
                  .then(() => {
                    setDraftCreated(null);
                    return onCreated();
                  })
                  .catch((err: unknown) => {
                    setFormError(
                      err instanceof Error ? err.message : t("error.projectStartFailed"),
                    );
                  })
              }
            >
              {t("projects.created.start")}
            </button>
          </div>
        </div>
      ) : null}
      {formError ? <ErrorState message={formError} /> : null}
    </section>
  );
}
