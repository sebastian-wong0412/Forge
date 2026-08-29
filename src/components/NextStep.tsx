import type { ReactNode } from "react";

export function NextStep({
  title,
  detail,
  action,
}: {
  title: string;
  detail: string;
  action?: ReactNode;
}) {
  return (
    <div className="panel next-step">
      <p>
        <strong>{title}</strong>
      </p>
      <p>{detail}</p>
      {action ? <div className="row">{action}</div> : null}
    </div>
  );
}
