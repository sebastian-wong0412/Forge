export function ErrorState({
  message,
  onRetry,
}: {
  message: string;
  onRetry?: () => void;
}) {
  return (
    <div className="state error" role="alert">
      <p>{message}</p>
      {onRetry ? (
        <p style={{ marginTop: 12 }}>
          <button type="button" className="btn" onClick={onRetry}>
            重试
          </button>
        </p>
      ) : null}
    </div>
  );
}
