import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ErrorState } from "../components/ErrorState";
import { useExample } from "../example/ExampleProvider";
import { useT } from "../i18n";
import { markOnboardingCompleted } from "../lib/onboarding";

export function WelcomePage() {
  const t = useT();
  const navigate = useNavigate();
  const example = useExample();
  const [busy, setBusy] = useState<"explore" | "scratch" | null>(null);
  const [error, setError] = useState<string | null>(null);

  async function onExplore() {
    setBusy("explore");
    setError(null);
    try {
      const tree = await example.enter();
      navigate(`/cycles/${tree.cycleId}`, { replace: true });
    } catch (err) {
      setError(err instanceof Error ? err.message : t("welcome.error"));
      setBusy(null);
    }
  }

  function onScratch() {
    setBusy("scratch");
    markOnboardingCompleted();
    navigate("/today", { replace: true });
  }

  return (
    <div className="welcome-shell">
      <section className="welcome" aria-labelledby="welcome-title">
        <p className="welcome-kicker">Forge</p>
        <h1 id="welcome-title">{t("welcome.title")}</h1>
        <p className="welcome-lead">{t("welcome.lead")}</p>
        {error ? <ErrorState message={error} /> : null}
        <div className="welcome-actions">
          <button
            type="button"
            className="btn btn-primary"
            disabled={busy !== null}
            onClick={() => void onExplore()}
          >
            {busy === "explore" ? t("welcome.exploring") : t("welcome.explore")}
          </button>
          <button
            type="button"
            className="btn"
            disabled={busy !== null}
            onClick={onScratch}
          >
            {t("welcome.scratch")}
          </button>
        </div>
      </section>
    </div>
  );
}
