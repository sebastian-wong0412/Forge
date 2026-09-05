import type { Cycle, Task, TodayResponse } from "../api/types";

export function cycle(overrides: Partial<Cycle> = {}): Cycle {
  return {
    id: "c1",
    name: "Q3 学习计划",
    start_on: "2026-07-01",
    end_on: "2026-09-30",
    status: "active",
    created_at: "2026-08-30T09:00:00Z",
    updated_at: "2026-08-30T09:00:00Z",
    ...overrides,
  };
}

export function task(overrides: Partial<Task> = {}): Task {
  return {
    id: "task-1",
    project_id: "project-1",
    title: "Finish research",
    description: "Complete the first draft",
    status: "todo",
    scheduled_on: "2026-08-30",
    completed_at: null,
    created_at: "2026-08-30T09:00:00Z",
    updated_at: "2026-08-30T09:00:00Z",
    ...overrides,
  };
}

export function today(overrides: Partial<TodayResponse> = {}): TodayResponse {
  return {
    date: "2026-08-30",
    scheduled: [],
    overdue: [],
    unscheduled_in_progress: [],
    completed: [],
    ...overrides,
  };
}
