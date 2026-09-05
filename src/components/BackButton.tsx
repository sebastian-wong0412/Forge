import { useNavigate } from "react-router-dom";
import { useT } from "../i18n";

function historyCanGoBack(): boolean {
  const index = window.history.state?.idx;
  return typeof index === "number" && index > 0;
}

export function BackButton({ fallback }: { fallback: string }) {
  const navigate = useNavigate();
  const t = useT();

  return (
    <button
      type="button"
      className="btn back-btn"
      onClick={() => {
        if (historyCanGoBack()) {
          navigate(-1);
          return;
        }
        navigate(fallback);
      }}
    >
      ‹ {t("common.back")}
    </button>
  );
}
