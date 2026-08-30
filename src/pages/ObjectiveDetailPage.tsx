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
import { Breadcrumbs } from "../components/Breadcrumbs";
import { CheckInPanel } from "../components/CheckInPanel";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { PageHeader } from "../components/PageHeader";
import { StatusBadge } from "../components/StatusBadge";
import { useLoad } from "../hooks/useLoad";
import { statusLabel } from "../lib/status";

export function ObjectiveDetailPage() {
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
      setError(err instanceof Error ? err.message : "无法更新目标，请稍后重试。");
    }
  }

  if (objective.loading && !objective.data) {
    return <LoadingState label="正在加载目标…" />;
  }
  if (objective.error && !objective.data) {
    return <ErrorState message={objective.error} onRetry={() => void objective.reload()} />;
  }
  if (!objective.data) {
    return <ErrorState message="未找到该目标。" />;
  }

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: "周期", to: "/cycles" },
          ...(cycle.data
            ? [{ label: cycle.data.name, to: `/cycles/${cycle.data.id}` }]
            : []),
          { label: objective.data.title },
        ]}
      />
      <PageHeader
        kicker="目标"
        title={objective.data.title}
        meta={
          <>
            {objective.data.start_on && objective.data.end_on
              ? `${objective.data.start_on} – ${objective.data.end_on} · `
              : null}
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
                开始
              </button>
            ) : null}
            {objective.data.status === "active" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => completeObjective(objectiveId))}
              >
                完成
              </button>
            ) : null}
            {objective.data.status !== "archived" ? (
              <button
                type="button"
                className="btn"
                onClick={() => void run(() => archiveObjective(objectiveId))}
              >
                归档
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

function keyResultSummary(keyResult: KeyResult, percent: string | null): string {
  if (keyResult.progress_kind === "milestone") {
    return `${statusLabel(keyResult.current_state ?? "not_started")}${percent ? ` · ${percent}` : ""}`;
  }
  if (keyResult.progress_kind === "qualitative") {
    return keyResult.latest_note ?? "还没有进展";
  }
  const current = keyResult.current_value ?? "—";
  const unit = keyResult.unit ? ` ${keyResult.unit}` : "";
  const start = keyResult.start_value ?? "—";
  const target = keyResult.target_value !== null ? ` · ${keyResult.target_value} 目标` : "";
  return `${current}${unit} 当前 · ${start} 起点${target}${percent ? ` · ${percent}` : ""}`;
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
          progressKind === "numeric" || progressKind === "percentage"
            ? Number(startValue)
            : null,
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
      setFormError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">关键结果</h2>
      {loading && keyResults.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {keyResults.length === 0 && !loading ? (
        <EmptyState title="还没有关键结果" detail="写出你希望看到的结果，不一定是数字。" />
      ) : null}
      {keyResults.map((keyResult) => (
        <KeyResultCard key={keyResult.id} keyResult={keyResult} onChanged={onChanged} />
      ))}
      <form className="panel stack" onSubmit={onSubmit}>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="kr-title">标题</label>
            <input id="kr-title" value={title} onChange={(event) => setTitle(event.target.value)} required />
          </div>
          <div className="field">
            <label htmlFor="kr-kind">类型</label>
            <select
              id="kr-kind"
              value={progressKind}
              onChange={(event) => setProgressKind(event.target.value as ProgressKind)}
            >
              <option value="numeric">数值</option>
              <option value="percentage">百分比</option>
              <option value="milestone">里程碑</option>
              <option value="qualitative">定性</option>
            </select>
          </div>
          {progressKind === "numeric" || progressKind === "percentage" ? (
            <>
              <div className="field">
                <label htmlFor="kr-start">起始值</label>
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
                <label htmlFor="kr-target">目标值</label>
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
              <label htmlFor="kr-unit">单位</label>
              <input id="kr-unit" value={unit} onChange={(event) => setUnit(event.target.value)} />
            </div>
          ) : null}
        </div>
        {formError ? <ErrorState message={formError} /> : null}
        <div>
          <button type="submit" className="btn btn-primary">
            添加关键结果
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
  const percent =
    keyResult.progress === null ? null : `${Math.round(keyResult.progress * 100)}%`;

  return (
    <article className="card stack">
      <div className="row">
        <strong>{keyResult.title}</strong>
        <StatusBadge status={keyResult.status} />
      </div>
      {keyResult.description ? <p>{keyResult.description}</p> : null}
      <p className="muted">{keyResultSummary(keyResult, percent)}</p>
      {keyResult.progress !== null ? (
        <div className="progress" aria-hidden="true">
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
            开始
          </button>
        ) : null}
        {keyResult.status === "active" ? (
          <button
            type="button"
            className="btn"
            onClick={() => void completeKeyResult(keyResult.id).then(onChanged)}
          >
            完成
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
      setFormError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">项目</h2>
      {loading && projects.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {projects.length === 0 && !loading ? (
        <EmptyState title="还没有项目" detail="把目标拆成可以持续推进的一组工作。" />
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
      <form className="panel row" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="project-title">新项目</label>
          <input
            id="project-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            required
          />
        </div>
        <button type="submit" className="btn btn-primary">
          添加
        </button>
      </form>
      {draftCreated && draftCreated.status === "draft" ? (
        <div className="panel next-step">
          <p>
            <strong>项目已创建。</strong>
          </p>
          <p>可以直接添加任务。开始项目后，父级周期和目标也会进入进行中。</p>
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
                    err instanceof Error ? err.message : "无法开始项目，请稍后重试。",
                  );
                })
            }
          >
            开始项目
          </button>
          </div>
        </div>
      ) : null}
      {formError ? <ErrorState message={formError} /> : null}
    </section>
  );
}
