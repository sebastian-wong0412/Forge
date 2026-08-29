import { FormEvent, useState } from "react";
import { Link } from "react-router-dom";
import { createCycle, getCycles, type Cycle } from "../api";
import { EmptyState } from "../components/EmptyState";
import { ErrorState } from "../components/ErrorState";
import { LoadingState } from "../components/LoadingState";
import { NextStep } from "../components/NextStep";
import { PageHeader } from "../components/PageHeader";
import { StatusBadge } from "../components/StatusBadge";
import { useLoad } from "../hooks/useLoad";

export function CyclesPage() {
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
      setFormError(err instanceof Error ? err.message : "创建失败，请稍后重试。");
    } finally {
      setBusy(false);
    }
  }

  if (loading && !data) {
    return <LoadingState label="正在加载周期…" />;
  }
  if (error && !data) {
    return <ErrorState message={error} onRetry={() => void reload()} />;
  }

  const cycles = data ?? [];

  return (
    <div className="stack">
      <PageHeader kicker="规划" title="周期" meta="一段时间内工作的起点。" />
      {cycles.length === 0 ? (
        <EmptyState
          title="还没有周期"
          detail="周期是 Forge 中组织一段时间工作的起点。你可以把它理解为一个阶段、季度、项目周期或个人计划。"
          action={
            <a href="#create-cycle" className="btn btn-primary">
              创建第一个周期
            </a>
          }
        />
      ) : null}
      {cycles.length === 0 ? (
        <ul className="muted example-list">
          <li>Q3 2026</li>
          <li>秋季学习计划</li>
          <li>产品上线阶段</li>
        </ul>
      ) : null}
      {created ? (
        <NextStep
          title="周期已创建"
          detail="下一步：创建一个目标，明确这个周期最重要的成果。"
          action={
            <Link to={`/cycles/${created.id}`} className="btn btn-primary">
              创建目标
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
              {cycle.start_on} – {cycle.end_on}
            </p>
          </Link>
        ))}
      </div>
      <form id="create-cycle" className="panel stack" onSubmit={onCreate}>
        <h2 className="section-title">新周期</h2>
        <div className="form-grid">
          <div className="field">
            <label htmlFor="cycle-name">名称</label>
            <input
              id="cycle-name"
              value={name}
              onChange={(event) => setName(event.target.value)}
              required
            />
          </div>
          <div className="field">
            <label htmlFor="cycle-start">开始</label>
            <input
              id="cycle-start"
              type="date"
              value={startOn}
              onChange={(event) => setStartOn(event.target.value)}
              required
            />
          </div>
          <div className="field">
            <label htmlFor="cycle-end">结束</label>
            <input
              id="cycle-end"
              type="date"
              value={endOn}
              onChange={(event) => setEndOn(event.target.value)}
              required
            />
          </div>
        </div>
        {formError ? <ErrorState message={formError} /> : null}
        <div>
          <button type="submit" className="btn btn-primary" disabled={busy}>
            创建周期
          </button>
        </div>
      </form>
    </div>
  );
}
