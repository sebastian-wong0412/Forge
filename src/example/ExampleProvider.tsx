import {
  createContext,
  useCallback,
  useContext,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import {
  ensureExampleWorkspace,
  loadExampleState,
  resetExampleWorkspace,
  saveExampleState,
  type ExampleState,
  type ExampleTree,
} from "../lib/exampleWorkspace";
import { markOnboardingCompleted } from "../lib/onboarding";

interface ExampleContextValue {
  active: boolean;
  current: ExampleTree | null;
  state: ExampleState;
  enter: () => Promise<ExampleTree>;
  exit: () => void;
  reset: () => Promise<ExampleTree>;
}

const ExampleContext = createContext<ExampleContextValue | null>(null);

export function ExampleProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<ExampleState>(() => loadExampleState());

  const persist = useCallback((next: ExampleState) => {
    saveExampleState(next);
    setState(next);
  }, []);

  const enter = useCallback(async () => {
    markOnboardingCompleted();
    const tree = await ensureExampleWorkspace();
    persist({ ...loadExampleState(), active: true, current: tree });
    return tree;
  }, [persist]);

  const exit = useCallback(() => {
    persist({ ...loadExampleState(), active: false });
  }, [persist]);

  const reset = useCallback(async () => {
    const tree = await resetExampleWorkspace();
    persist(loadExampleState());
    return tree;
  }, [persist]);

  const value = useMemo<ExampleContextValue>(
    () => ({
      active: state.active,
      current: state.current,
      state,
      enter,
      exit,
      reset,
    }),
    [enter, exit, reset, state],
  );

  return <ExampleContext.Provider value={value}>{children}</ExampleContext.Provider>;
}

export function useExample(): ExampleContextValue {
  const value = useContext(ExampleContext);
  if (!value) {
    throw new Error("useExample must be used within ExampleProvider");
  }
  return value;
}

export function useOptionalExample(): ExampleContextValue | null {
  return useContext(ExampleContext);
}
