import {
  activateCycle,
  activateKeyResult,
  activateObjective,
  activateProject,
  archiveCycle,
  completeTask,
  createCheckIn,
  createCycle,
  createKeyResult,
  createObjective,
  createProject,
  createTask,
  getCycle,
  getObjective,
  getProject,
  startTask,
} from "../api";
import type { Cycle, IsoDate, Task, TodayResponse } from "../api/types";
import { localCalendarDate, shiftCalendarDate } from "./dates";

export const EXAMPLE_STORAGE_KEY = "forge.example";

export const EXAMPLE_CYCLE_NAME = "Q3 Learning Plan";
export const EXAMPLE_OBJECTIVE_TITLE = "Build strong foundations in machine learning";
export const EXAMPLE_KEY_RESULT_TITLE = "Complete 3 machine learning projects";
export const EXAMPLE_PROJECT_TITLE = "Machine Learning Course Project";
export const EXAMPLE_TASK_TITLES = [
  "Complete Chapter 2 exercises",
  "Review gradient descent notes",
  "Implement a basic linear regression model",
] as const;

export interface ExampleTree {
  cycleId: string;
  objectiveId: string;
  keyResultId: string;
  projectId: string;
  taskIds: string[];
}

export interface ExampleState {
  active: boolean;
  current: ExampleTree | null;
  retiredCycleIds: string[];
  retiredProjectIds: string[];
}

const EMPTY_STATE: ExampleState = {
  active: false,
  current: null,
  retiredCycleIds: [],
  retiredProjectIds: [],
};

function pad2(value: number): string {
  return String(value).padStart(2, "0");
}

export function quarterContaining(date: IsoDate): { start: IsoDate; end: IsoDate } {
  const [year, month] = date.split("-").map(Number);
  if (!year || !month) {
    return { start: date, end: date };
  }
  const startMonth = Math.floor((month - 1) / 3) * 3 + 1;
  const endMonth = startMonth + 2;
  const lastDay = new Date(Date.UTC(year, endMonth, 0)).getUTCDate();
  return {
    start: `${year}-${pad2(startMonth)}-01`,
    end: `${year}-${pad2(endMonth)}-${pad2(lastDay)}`,
  };
}

export function loadExampleState(): ExampleState {
  try {
    const raw = localStorage.getItem(EXAMPLE_STORAGE_KEY);
    if (!raw) {
      return EMPTY_STATE;
    }
    const parsed = JSON.parse(raw) as Partial<ExampleState>;
    return {
      active: parsed.active === true,
      current: isTree(parsed.current) ? parsed.current : null,
      retiredCycleIds: Array.isArray(parsed.retiredCycleIds)
        ? parsed.retiredCycleIds.filter((id): id is string => typeof id === "string")
        : [],
      retiredProjectIds: Array.isArray(parsed.retiredProjectIds)
        ? parsed.retiredProjectIds.filter((id): id is string => typeof id === "string")
        : [],
    };
  } catch {
    return EMPTY_STATE;
  }
}

export function saveExampleState(state: ExampleState): void {
  try {
    localStorage.setItem(EXAMPLE_STORAGE_KEY, JSON.stringify(state));
  } catch {
    // ignore quota / private-mode failures
  }
}

function isTree(value: unknown): value is ExampleTree {
  if (!value || typeof value !== "object") {
    return false;
  }
  const tree = value as ExampleTree;
  return (
    typeof tree.cycleId === "string" &&
    typeof tree.objectiveId === "string" &&
    typeof tree.keyResultId === "string" &&
    typeof tree.projectId === "string" &&
    Array.isArray(tree.taskIds)
  );
}

export function knownExampleCycleIds(state: ExampleState): Set<string> {
  const ids = [...state.retiredCycleIds];
  if (state.current?.cycleId) {
    ids.push(state.current.cycleId);
  }
  return new Set(ids);
}

export function knownExampleProjectIds(state: ExampleState): Set<string> {
  const ids = [...state.retiredProjectIds];
  if (state.current?.projectId) {
    ids.push(state.current.projectId);
  }
  return new Set(ids);
}

export function visibleCycles(cycles: Cycle[], state: ExampleState): Cycle[] {
  const hidden = knownExampleCycleIds(state);
  if (state.active && state.current) {
    return cycles.filter((cycle) => cycle.id === state.current?.cycleId);
  }
  return cycles.filter((cycle) => !hidden.has(cycle.id));
}

export function belongsToExample(task: Task, state: ExampleState): boolean {
  return knownExampleProjectIds(state).has(task.project_id);
}

export function filterToday(today: TodayResponse, state: ExampleState): TodayResponse {
  const keep = (task: Task) =>
    state.active ? belongsToExample(task, state) : !belongsToExample(task, state);
  return {
    ...today,
    scheduled: today.scheduled.filter(keep),
    overdue: today.overdue.filter(keep),
    unscheduled_in_progress: today.unscheduled_in_progress.filter(keep),
    completed: today.completed.filter(keep),
  };
}

export function hasRealUserCycles(cycles: Cycle[], state: ExampleState): boolean {
  return visibleCycles(cycles, { ...state, active: false }).length > 0;
}

async function treeIsUsable(tree: ExampleTree): Promise<boolean> {
  try {
    const cycle = await getCycle(tree.cycleId);
    if (cycle.status === "archived") {
      return false;
    }
    await getObjective(tree.objectiveId);
    await getProject(tree.projectId);
    return true;
  } catch {
    return false;
  }
}

function retire(state: ExampleState, tree: ExampleTree | null): ExampleState {
  if (!tree) {
    return state;
  }
  return {
    ...state,
    current: null,
    retiredCycleIds: unique([...state.retiredCycleIds, tree.cycleId]),
    retiredProjectIds: unique([...state.retiredProjectIds, tree.projectId]),
  };
}

function unique(ids: string[]): string[] {
  return [...new Set(ids)];
}

export async function createExampleWorkspace(today = localCalendarDate()): Promise<ExampleTree> {
  const dates = quarterContaining(today);
  const earlier = shiftCalendarDate(today, -14);
  const recent = shiftCalendarDate(today, -7);

  const cycle = await createCycle({
    name: EXAMPLE_CYCLE_NAME,
    start_on: dates.start,
    end_on: dates.end,
  });
  await activateCycle(cycle.id);

  const objective = await createObjective(cycle.id, {
    title: EXAMPLE_OBJECTIVE_TITLE,
    description: "Build Your Data Science Foundation",
    start_on: dates.start,
    end_on: dates.end,
  });
  await activateObjective(objective.id);

  const keyResult = await createKeyResult(objective.id, {
    title: EXAMPLE_KEY_RESULT_TITLE,
    progress_kind: "numeric",
    start_value: 0,
    target_value: 3,
    unit: "projects",
  });
  await activateKeyResult(keyResult.id);
  await createCheckIn(keyResult.id, {
    value: 1,
    note: "1 / 3 projects completed",
    checked_on: earlier < dates.start ? dates.start : earlier,
  });
  await createCheckIn(keyResult.id, {
    value: 2,
    note: "2 / 3 projects completed",
    checked_on: recent < dates.start ? dates.start : recent,
  });

  const project = await createProject(objective.id, {
    title: EXAMPLE_PROJECT_TITLE,
  });
  await activateProject(project.id);

  const chapter = await createTask(project.id, {
    title: EXAMPLE_TASK_TITLES[0],
  });
  await startTask(chapter.id);
  await completeTask(chapter.id);

  const review = await createTask(project.id, {
    title: EXAMPLE_TASK_TITLES[1],
    scheduled_on: today,
  });

  const implement = await createTask(project.id, {
    title: EXAMPLE_TASK_TITLES[2],
  });

  return {
    cycleId: cycle.id,
    objectiveId: objective.id,
    keyResultId: keyResult.id,
    projectId: project.id,
    taskIds: [chapter.id, review.id, implement.id],
  };
}

export async function ensureExampleWorkspace(): Promise<ExampleTree> {
  const state = loadExampleState();
  if (state.current && (await treeIsUsable(state.current))) {
    return state.current;
  }

  const next = retire(state, state.current);
  const tree = await createExampleWorkspace();
  saveExampleState({ ...next, current: tree });
  return tree;
}

export async function resetExampleWorkspace(): Promise<ExampleTree> {
  const state = loadExampleState();
  if (state.current?.cycleId) {
    try {
      await archiveCycle(state.current.cycleId);
    } catch {
      // keep going — a missing or already-archived cycle should not block reset
    }
  }
  const retired = retire(state, state.current);
  const tree = await createExampleWorkspace();
  saveExampleState({ ...retired, active: true, current: tree });
  return tree;
}
