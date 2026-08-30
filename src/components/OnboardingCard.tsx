import { Link } from "react-router-dom";
import { useT } from "../i18n";

export function OnboardingCard() {
  const t = useT();

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
      <Link to="/cycles" className="btn btn-primary">
        {t("onboarding.cta")}
      </Link>
    </section>
  );
}
