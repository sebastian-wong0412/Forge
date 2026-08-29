import type { Task, TodayResponse } from "../api/types";

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
