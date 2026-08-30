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
import { useT, type TranslateFn } from "../i18n";

export function CycleDetailPage() {
  const t = useT();
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
      setError(err instanceof Error ? err.message : t("error.cycleUpdateFailed"));
    }
  }

  if (cycle.loading && !cycle.data) {
    return <LoadingState label={t("cycles.loadingOne")} />;
  }
  if (cycle.error && !cycle.data) {
    return <ErrorState message={cycle.error} onRetry={() => void cycle.reload()} />;
  }
  if (!cycle.data) {
    return <ErrorState message={t("error.cycleNotFound")} />;
  }

  return (
    <div className="stack">
      <Breadcrumbs
        items={[{ label: t("cycles.breadcrumb"), to: "/cycles" }, { label: cycle.data.name }]}
      />
      <PageHeader
        kicker={t("cycles.kickerOne")}
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
                {t("common.start")}
              </button>
            ) : null}
            {cycle.data.status === "active" ? (
              <button type="button" className="btn" onClick={() => void run(() => closeCycle(cycleId))}>
                {t("common.end")}
              </button>
            ) : null}
            {cycle.data.status !== "archived" ? (
              <button type="button" className="btn" onClick={() => void run(() => archiveCycle(cycleId))}>
                {t("common.archive")}
              </button>
            ) : null}
          </>
        }
      />
      {error ? <ErrorState message={error} /> : null}
      {createdObjective ? (
        <NextStep
          title={t("objectives.created.title")}
          detail={t("objectives.created.detail")}
          action={
            <Link to={`/objectives/${createdObjective.id}`} className="btn btn-primary">
              {t("objectives.created.action")}
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
  const t = useT();
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
      setFormError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">{t("objectives.section")}</h2>
      {loading && objectives.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {objectives.length === 0 && !loading ? (
        <EmptyState title={t("objectives.empty.title")} detail={t("objectives.empty.detail")} />
      ) : null}
      {objectives.map((objective) => (
        <ObjectiveProjects key={objective.id} objective={objective} />
      ))}
      <form className="panel row" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="objective-title">{t("objectives.form.label")}</label>
          <input
            id="objective-title"
            value={title}
            onChange={(event) => setTitle(event.target.value)}
            required
          />
        </div>
        <button type="submit" className="btn btn-primary">
          {t("common.add")}
        </button>
      </form>
      {formError ? <ErrorState message={formError} /> : null}
    </section>
  );
}

function ObjectiveProjects({ objective }: { objective: Objective }) {
  const t = useT();
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
          : t("common.dateUnset")}
      </p>
      <ProjectSummary projects={projects.data ?? []} t={t} />
    </Link>
  );
}

function ProjectSummary({ projects, t }: { projects: Project[]; t: TranslateFn }) {
  if (projects.length === 0) {
    return <p className="muted">{t("objectives.noProjects")}</p>;
  }
  return <p className="muted">{t("objectives.projectCount", { count: projects.length })}</p>;
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
  const t = useT();
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
      setFormError(err instanceof Error ? err.message : t("error.createFailed"));
    }
  }

  return (
    <section className="stack">
      <h2 className="section-title">{t("reviews.section")}</h2>
      {loading && reviews.length === 0 ? <LoadingState /> : null}
      {error ? <ErrorState message={error} /> : null}
      {reviews.length === 0 && !loading ? (
        <EmptyState title={t("reviews.empty.title")} detail={t("reviews.empty.detail")} />
      ) : null}
      {reviews.map((review) => (
        <article key={review.id} className="card">
          <p>{review.content}</p>
          <p className="muted">
            {review.period_start || review.period_end
              ? `${review.period_start ?? t("common.ellipsis")} – ${review.period_end ?? t("common.ellipsis")}`
              : t("common.periodUnset")}
          </p>
        </article>
      ))}
      <form className="panel stack" onSubmit={onSubmit}>
        <div className="field">
          <label htmlFor="review-content">{t("reviews.form.content")}</label>
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
            <label htmlFor="review-start">{t("reviews.form.start")}</label>
            <input
              id="review-start"
              type="date"
              value={periodStart}
              onChange={(event) => setPeriodStart(event.target.value)}
            />
          </div>
          <div className="field">
            <label htmlFor="review-end">{t("reviews.form.end")}</label>
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
            {t("reviews.form.submit")}
          </button>
        </div>
      </form>
    </section>
  );
}
