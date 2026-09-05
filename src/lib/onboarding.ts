const STORAGE_KEY = "forge.onboarding";

export interface OnboardingState {
  completed: boolean;
}

export function loadOnboardingState(): OnboardingState {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) {
      return { completed: false };
    }
    const parsed = JSON.parse(raw) as { completed?: unknown };
    return { completed: parsed.completed === true };
  } catch {
    return { completed: false };
  }
}

export function isOnboardingCompleted(): boolean {
  return loadOnboardingState().completed;
}

export function markOnboardingCompleted(): void {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify({ completed: true }));
  } catch {
    // ignore quota / private-mode failures
  }
}
