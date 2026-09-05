import { useEffect, useState, type ReactNode } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import { getCycles } from "../api";
import { useExample } from "../example/ExampleProvider";
import { useT } from "../i18n";
import { hasRealUserCycles } from "../lib/exampleWorkspace";
import { isOnboardingCompleted, markOnboardingCompleted } from "../lib/onboarding";
import { ErrorState } from "./ErrorState";
import { LoadingState } from "./LoadingState";

export function OnboardingGate({ children }: { children: ReactNode }) {
  const t = useT();
  const location = useLocation();
  const navigate = useNavigate();
  const example = useExample();
  const [ready, setReady] = useState(isOnboardingCompleted());
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isOnboardingCompleted()) {
      setReady(true);
      return;
    }

    let cancelled = false;
    void getCycles()
      .then((cycles) => {
        if (cancelled) {
          return;
        }
        if (hasRealUserCycles(cycles, example.state)) {
          markOnboardingCompleted();
        }
        setReady(true);
      })
      .catch((err: unknown) => {
        if (!cancelled) {
          setError(err instanceof Error ? err.message : t("welcome.error"));
          setReady(true);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [example.state, t]);

  useEffect(() => {
    if (!ready) {
      return;
    }
    const completed = isOnboardingCompleted();
    if (!completed && location.pathname !== "/welcome") {
      navigate("/welcome", { replace: true });
    }
    if (completed && location.pathname === "/welcome") {
      navigate("/today", { replace: true });
    }
  }, [location.pathname, navigate, ready]);

  if (!ready) {
    return <LoadingState label={t("welcome.loading")} />;
  }
  if (error && location.pathname === "/welcome") {
    return <ErrorState message={error} />;
  }
  return children;
}
