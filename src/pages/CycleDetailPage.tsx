import { FormEvent, useState } from "react";
import { Link, useParams } from "react-router-dom";
import {
  activateCycle,
  archiveCycle,
  closeCycle,
  createObjective,
  createReview,
  getCycle,
  getObjectives,
  getProjects,
  getReviews,
  type Objective,
  type Project,
} from "../api";
import { Breadcrumbs } from "../components/Breadcrumbs";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { NextStep } from "../components/NextStep";
import { PageHeader } from "../components/PageHeader";
import { StatusBadge } from "../components/StatusBadge";
import { useLoad } from "../hooks/useLoad";

export function CycleDetailPage() {
  const { cycleId = "" } = useParams();
  const cycle = useLoad(() => getCycle(cycleId), [cycleId]);
  const objectives = useLoad(() => getObjectives(cycleId), [cycleId]);
  const reviews = useLoad(() => getReviews(cycleId), [cycleId]);
  const [error, setError] = useState<string | null>(null);
  const [createdObjective, setCreatedObjective] = useState<Objective | null>(null);

  async function refresh() {
    await Promise.all([cycle.reload(), objectives.reload(), reviews.reload()]);
  }

  async function run(action: () => Promise<unknown>) {
    setError(null);
    try {
      await action();
      await refresh();
    } catch (err) {
      setError(err instanceof Error ? err.message : "无法更新周期，请稍后重试。");
    }
  }

  if (cycle.loading && !cycle.data) {
    return <LoadingState label="正在加载周期…" />;
  }
  if (cycle.error && !cycle.data) {
    return <ErrorState message={cycle.error} onRetry={() => void cycle.reload()} />;
  }
  if (!cycle.data) {
    return <ErrorState message="未找到该周期。" />;
  }

  return (
    <div className="stack">
      <Breadcrumbs
        items={[
          { label: "周期", to: "/cycles" },
          { label: cycle.data.name },
        ]}
      />
      <PageHeader
        kicker="周期"
        title={cycle.data.name}
        meta={
          <>
            {cycle.data.start_on} – {cycle.data.end_on} · <StatusBadge status={cycle.data.status} />
          </>
        }
        actions={
          <>
            {cycle.data.status === "planning" ? (
              <button type="button" className="btn" onClick={() => void run(() => activateCycle(cycleId))}>
                开始
              </button>
            ) : null}
            {cycle.data.status === "active" ? (
              <button type="button" className="btn" onClick={() => void run(() => closeCycle(cycleId))}>
                结束
              </button>
            ) : null}
            {cycle.data.status !== "archived" ? (
              <button type="button" className="btn" onClick={() => void run(() => archiveCycle(cycleId))}>
                归档
              </button>
            ) : null}
          </>
        }
      />
      {error ? <ErrorState message={error} /> : null}
      {createdObjective ? (
        <NextStep
          title="目标已创建"
          detail="下一步：添加关键结果或创建一个项目。"
          action={
            <Link to={`/objectives/${createdObjective.id}`} className="btn btn-primary">
              前往目标
            </Link>
          }
        />
      ) : null}
      <ObjectivesSection
        cycleId={cycleId}
        objectives={objectives.data ?? []}
        loading={objectives.loading}
        error={objectives.error}
        onCreated={async (objective) => {
          setCreatedObjective(objective);
          await refresh();
        }}
      />
      <ReviewsSection
        cycleId={cycleId}
        reviews={reviews.data ?? []}
        loading={reviews.loading}
        error={reviews.error}
        onCreated={refresh}
      />
    </div>
  );
}

function ObjectivesSection({
  cycleId,
  objectives,
  loading,
  error,
  onCreated,
}: {
  cycleId: string;
  objectives: Objective[];
  loading: boolean;
  error: string | null;
  onCreated: (objective: Objective) => Promise<void>;
}) {
  const [title, setTitle] = useState("");
  const [formError, setFormError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setFormError(null);
    try {
      const created = await createObjective(cycleId, { title });
      setTitle("");
      await onCreated(created);
    } catch (err) {
      setFormError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">目标</h2>
      {loading && objectives.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {objectives.length === 0 && !loading ? (
        <EmptyState title="还没有目标" detail="明确这个周期里你最想实现的结果。" />
      ) : null}
      {objectives.map((objective) => (
        <ObjectiveProjects key={objective.id} objective={objective} />
      ))}
      <form className="panel row" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="objective-title">新目标</label>
          <input
            id="objective-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            required
          />
        </div>
        <button type="submit" className="btn btn-primary">
          添加
        </button>
      </form>
      {formError ? <ErrorState message={formError} /> : null}
    </section>
  );
}

function ObjectiveProjects({ objective }: { objective: Objective }) {
  const projects = useLoad(() => getProjects(objective.id), [objective.id]);

  return (
    <Link to={`/objectives/${objective.id}`} className="card card-link">
      <div className="row">
        <strong>{objective.title}</strong>
        <StatusBadge status={objective.status} />
      </div>
      <p className="muted">
        {objective.start_on && objective.end_on
          ? `${objective.start_on} – ${objective.end_on}`
          : "未设置日期"}
      </p>
      <ProjectSummary projects={projects.data ?? []} />
    </Link>
  );
}

function ProjectSummary({ projects }: { projects: Project[] }) {
  if (projects.length === 0) {
    return <p className="muted">还没有项目</p>;
  }
  return <p className="muted">{projects.length} 个项目</p>;
}

function ReviewsSection({
  cycleId,
  reviews,
  loading,
  error,
  onCreated,
}: {
  cycleId: string;
  reviews: Awaited<ReturnType<typeof getReviews>>;
  loading: boolean;
  error: string | null;
  onCreated: () => Promise<void>;
}) {
  const [content, setContent] = useState("");
  const [periodStart, setPeriodStart] = useState("");
  const [periodEnd, setPeriodEnd] = useState("");
  const [formError, setFormError] = useState<string | null>(null);

  async function onSubmit(event: FormEvent) {
    event.preventDefault();
    setFormError(null);
    try {
      await createReview(cycleId, {
        content,
        period_start: periodStart || null,
        period_end: periodEnd || null,
      });
      setContent("");
      setPeriodStart("");
      setPeriodEnd("");
      await onCreated();
    } catch (err) {
      setFormError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">复盘</h2>
      {loading && reviews.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {reviews.length === 0 && !loading ? (
        <EmptyState title="还没有复盘" detail="写下这个周期的收获与调整。" />
      ) : null}
      {reviews.map((review) => (
        <article key={review.id} className="card">
          <p>{review.content}</p>
          <p className="muted">
            {review.period_start || review.period_end
              ? `${review.period_start ?? "…"} – ${review.period_end ?? "…"}`
              : "未设置时段"}
          </p>
        </article>
      ))}
      <form className="panel stack" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="review-content">新复盘</label>
          <textarea
            id="review-content"
            rows={4}
            value={content}
            onChange={(event) => setContent(event.target.value)}
            required
          />
        </div>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="review-start">开始</label>
            <input
              id="review-start"
              type="date"
              value={periodStart}
              onChange={(event) => setPeriodStart(event.target.value)}
            />
          </div>
          <div className="field">
            <label htmlFor="review-end">结束</label>
            <input
              id="review-end"
              type="date"
              value={periodEnd}
              onChange={(event) => setPeriodEnd(event.target.value)}
            />
          </div>
        </div>
        {formError ? <ErrorState message={formError} /> : null}
        <div>
          <button type="submit" className="btn btn-primary">
            添加复盘
          </button>
        </div>
      </form>
    </section>
  );
}
