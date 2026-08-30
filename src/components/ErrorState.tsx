import { useT } from "../i18n";

export function ErrorState({
  message,
  onRetry,
}: {
  message: string;
  onRetry?: () => void;
}) {
  const t = useT();

  return (
    <div className="state error" role="alert">
      <p>{message}</p>
      {onRetry ? (
        <p style={{ marginTop: 12 }}>
          <button type="button" className="btn" onClick={onRetry}>
            {t("common.retry")}
          </button>
        </p>
      ) : null}
    </div>
  );
}
