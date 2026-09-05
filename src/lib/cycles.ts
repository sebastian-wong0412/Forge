import type { Cycle } from "../api/types";

const OPEN_CYCLE_LIMIT = 5;

export function openCycleShortcuts(cycles: Cycle[]): Cycle[] {
  return cycles
    .filter((cycle) => cycle.status === "active" || cycle.status === "planning")
    .sort((a, b) => {
      if (a.status !== b.status) {
        return a.status === "active" ? -1 : 1;
      }
      return b.updated_at.localeCompare(a.updated_at);
    })
    .slice(0, OPEN_CYCLE_LIMIT);
}
