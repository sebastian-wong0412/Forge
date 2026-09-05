import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useExample } from "../example/ExampleProvider";
import { useT } from "../i18n";
import { ConceptTour } from "./ConceptTour";

export function ExampleBanner() {
  const t = useT();
  const navigate = useNavigate();
  const example = useExample();
  const [tourOpen, setTourOpen] = useState(false);
  const [resetting, setResetting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  function onExit() {
    example.exit();
    navigate("/today");
  }

  async function onReset() {
    setResetting(true);
    setError(null);
    try {
      const tree = await example.reset();
      navigate(`/cycles/${tree.cycleId}`, { replace: true });
    } catch (err) {
      setError(err instanceof Error ? err.message : t("example.resetFailed"));
    } finally {
      setResetting(false);
    }
  }

  return (
    <section className="example-banner" aria-label={t("example.banner.title")}>
      <div className="example-banner-copy">
        <p>
          <strong>{t("example.banner.title")}</strong>
        </p>
        <p className="muted">{t("example.banner.detail")}</p>
        {error ? <p className="muted">{error}</p> : null}
      </div>
      <div className="example-banner-actions">
        <button type="button" className="btn" onClick={() => setTourOpen(true)}>
          {t("example.how")}
        </button>
        <button type="button" className="btn" disabled={resetting} onClick={() => void onReset()}>
          {resetting ? t("example.resetting") : t("example.reset")}
        </button>
        <button type="button" className="btn" onClick={onExit}>
          {t("example.exit")}
        </button>
      </div>
      {tourOpen ? <ConceptTour onClose={() => setTourOpen(false)} /> : null}
    </section>
  );
}
