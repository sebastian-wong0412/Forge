import { useT } from "../i18n";

export function LoadingState({ label }: { label?: string }) {
  const t = useT();

  return (
    <div className="state" role="status">
      {label ?? t("common.loading")}
    </div>
  );
}
