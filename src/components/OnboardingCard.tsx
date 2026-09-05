import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { ErrorState } from "./ErrorState";
import { useOptionalExample } from "../example/ExampleProvider";
import { useT } from "../i18n";

export function OnboardingCard() {
  const t = useT();
  const navigate = useNavigate();
  const example = useOptionalExample();
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function onExplore() {
    if (!example) {
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const tree = await example.enter();
      navigate(`/cycles/${tree.cycleId}`);
    } catch (err) {
      setError(err instanceof Error ? err.message : t("example.enterFailed"));
    } finally {
      setBusy(false);
    }
  }

  return (
    <section className="onboarding" aria-labelledby="onboarding-title">
      <h2 id="onboarding-title">{t("onboarding.title")}</h2>
      <p>{t("onboarding.lead1")}</p>
      <p>{t("onboarding.lead2")}</p>
      <ol className="onboarding-steps">
        <li>
          <strong>{t("onboarding.step1.title")}</strong>
          <span>{t("onboarding.step1.detail")}</span>
        </li>
        <li>
          <strong>{t("onboarding.step2.title")}</strong>
          <span>{t("onboarding.step2.detail")}</span>
        </li>
        <li>
          <strong>{t("onboarding.step3.title")}</strong>
          <span>{t("onboarding.step3.detail")}</span>
        </li>
        <li>
          <strong>{t("onboarding.step4.title")}</strong>
          <span>{t("onboarding.step4.detail")}</span>
        </li>
      </ol>
      {error ? <ErrorState message={error} /> : null}
      <div className="row">
        <Link to="/cycles" className="btn btn-primary">
          {t("onboarding.cta")}
        </Link>
        {example ? (
          <button type="button" className="btn" disabled={busy} onClick={() => void onExplore()}>
            {t("onboarding.explore")}
          </button>
        ) : null}
      </div>
    </section>
  );
}
