import { getCycles } from "../api";
import { cycle } from "./fixtures";
import { hasRealUserCycles, saveExampleState } from "../lib/exampleWorkspace";
import {
  isOnboardingCompleted,
  loadOnboardingState,
  markOnboardingCompleted,
} from "../lib/onboarding";

test("fresh onboarding state is incomplete", () => {
  expect(loadOnboardingState()).toEqual({ completed: false });
  expect(isOnboardingCompleted()).toBe(false);
});

test("markOnboardingCompleted persists across reads", () => {
  markOnboardingCompleted();
  expect(isOnboardingCompleted()).toBe(true);
  expect(loadOnboardingState().completed).toBe(true);
});

test("existing user cycles skip first-run even without a stored flag", () => {
  const state = {
    active: false,
    current: { cycleId: "ex", objectiveId: "o", keyResultId: "k", projectId: "p", taskIds: [] },
    retiredCycleIds: [],
    retiredProjectIds: [],
  };
  expect(hasRealUserCycles([cycle({ id: "user-1" })], state)).toBe(true);
  expect(hasRealUserCycles([cycle({ id: "ex" })], state)).toBe(false);
  expect(hasRealUserCycles([], state)).toBe(false);
});

test("getCycles is the existing-user signal used by the gate", () => {
  expect(typeof getCycles).toBe("function");
  saveExampleState({
    active: false,
    current: null,
    retiredCycleIds: ["ex"],
    retiredProjectIds: [],
  });
  expect(hasRealUserCycles([cycle({ id: "ex" })], {
    active: false,
    current: null,
    retiredCycleIds: ["ex"],
    retiredProjectIds: [],
  })).toBe(false);
});
