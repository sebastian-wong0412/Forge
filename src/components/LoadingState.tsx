export function LoadingState({ label = "加载中…" }: { label?: string }) {
  return (
    <div className="state" role="status">
      {label}
    </div>
  );
}
