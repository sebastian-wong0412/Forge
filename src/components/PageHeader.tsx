import type { ReactNode } from "react";

export function PageHeader({
  kicker,
  title,
  meta,
  actions,
}: {
  kicker?: string;
  title: string;
  meta?: ReactNode;
  actions?: ReactNode;
}) {
  return (
    <header className="page-header">
      <div>
        {kicker ? <div className="page-kicker">{kicker}</div> : null}
        <h1 className="page-title">{title}</h1>
        {meta ? <div className="page-meta">{meta}</div> : null}
      </div>
      {actions ? <div className="row">{actions}</div> : null}
    </header>
  );
}
