import type { ReactNode } from "react";

export function EmptyState({
  title,
  detail,
  action,
}: {
  title: string;
  detail?: string;
  action?: ReactNode;
}) {
  return (
    <div className="state">
      <p>{title}</p>
      {detail ? <p className="muted">{detail}</p> : null}
      {action ? <div className="row" style={{ marginTop: 12 }}>{action}</div> : null}
    </div>
  );
}
