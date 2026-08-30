import { Link } from "react-router-dom";
import { useT } from "../i18n";

export interface BreadcrumbItem {
  label: string;
  to?: string;
}

export function Breadcrumbs({ items }: { items: BreadcrumbItem[] }) {
  const t = useT();

  return (
    <nav className="breadcrumbs" aria-label={t("nav.breadcrumbs")}>
      {items.map((item, index) => (
        <span key={`${item.label}-${index}`} className="breadcrumb-item">
          {index > 0 ? <span className="breadcrumb-sep">→</span> : null}
          {item.to ? <Link to={item.to}>{item.label}</Link> : <span>{item.label}</span>}
        </span>
      ))}
    </nav>
  );
}
