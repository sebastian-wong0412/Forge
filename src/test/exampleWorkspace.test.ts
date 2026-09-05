import { beforeEach, expect, test, vi } from "vitest";
import type { Cycle, TodayResponse } from "../api/types";
import { cycle, task, today } from "./fixtures";
import {
  EXAMPLE_STORAGE_KEY,
  belongsToExample,
  ensureExampleWorkspace,
  filterToday,
  knownExampleCycleIds,
  loadExampleState,
  quarterContaining,
  resetExampleWorkspace,
  saveExampleState,
  visibleCycles,
} from "../lib/exampleWorkspace";

const mocks = vi.hoisted(() => ({
  getCycle: vi.fn(),
  getObjective: vi.fn(),
  getProject: vi.fn(),
  createCycle: vi.fn(),
  activateCycle: vi.fn(),
  createObjective: vi.fn(),
  activateObjective: vi.fn(),
  createKeyResult: vi.fn(),
  activateKeyResult: vi.fn(),
  createCheckIn: vi.fn(),
  createProject: vi.fn(),
  activateProject: vi.fn(),
  createTask: vi.fn(),
  startTask: vi.fn(),
  completeTask: vi.fn(),
  archiveCycle: vi.fn(),
}));

vi.mock("../api", () => mocks);

function tree(id = "1") {
  return {
    cycleId: `c${id}`,
    objectiveId: `o${id}`,
    keyResultId: `k${id}`,
    projectId: `p${id}`,
    taskIds: [`t${id}a`, `t${id}b`, `t${id}c`],
  };
}

function stubCreate(id: string) {
  mocks.createCycle.mockResolvedValue({ id: `c${id}` });
  mocks.activateCycle.mockResolvedValue({});
  mocks.createObjective.mockResolvedValue({ id: `o${id}` });
  mocks.activateObjective.mockResolvedValue({});
  mocks.createKeyResult.mockResolvedValue({ id: `k${id}` });
  mocks.activateKeyResult.mockResolvedValue({});
  mocks.createCheckIn.mockResolvedValue({});
  mocks.createProject.mockResolvedValue({ id: `p${id}` });
  mocks.activateProject.mockResolvedValue({});
  mocks.createTask
    .mockResolvedValueOnce({ id: `t${id}a` })
    .mockResolvedValueOnce({ id: `t${id}b` })
    .mockResolvedValueOnce({ id: `t${id}c` });
  mocks.startTask.mockResolvedValue({});
  mocks.completeTask.mockResolvedValue({});
}

beforeEach(() => {
  vi.clearAllMocks();
  localStorage.removeItem(EXAMPLE_STORAGE_KEY);
});

test("quarterContaining keeps today inside the example cycle", () => {
  expect(quarterContaining("2026-09-06")).toEqual({ start: "2026-07-01", end: "2026-09-30" });
});

test("visibleCycles hides example data in the normal workspace", () => {
  const state = {
    active: false,
    current: tree(),
    retiredCycleIds: ["old"],
    retiredProjectIds: ["old-p"],
  };
  const cycles: Cycle[] = [cycle({ id: "c1", name: "Example" }), cycle({ id: "mine" })];
  expect(visibleCycles(cycles, state).map((item) => item.id)).toEqual(["mine"]);
});

test("visibleCycles shows only the example cycle while exploring", () => {
  const state = {
    active: true,
    current: tree(),
    retiredCycleIds: ["old"],
    retiredProjectIds: [],
  };
  const cycles: Cycle[] = [cycle({ id: "c1" }), cycle({ id: "mine" })];
  expect(visibleCycles(cycles, state).map((item) => item.id)).toEqual(["c1"]);
});

test("filterToday keeps example tasks out of the user Today list", () => {
  const state = {
    active: false,
    current: tree(),
    retiredCycleIds: [],
    retiredProjectIds: ["p1"],
  };
  const data: TodayResponse = today({
    scheduled: [task({ id: "user", project_id: "real" }), task({ id: "demo", project_id: "p1" })],
  });
  expect(filterToday(data, state).scheduled.map((item) => item.id)).toEqual(["user"]);
});

test("belongsToExample uses retired project ids after reset", () => {
  const state = {
    active: false,
    current: null,
    retiredCycleIds: [],
    retiredProjectIds: ["old-p"],
  };
  expect(belongsToExample(task({ project_id: "old-p" }), state)).toBe(true);
  expect(knownExampleCycleIds(state).size).toBe(0);
});

test("ensureExampleWorkspace reuses a live example instead of duplicating", async () => {
  const existing = tree("1");
  saveExampleState({
    active: true,
    current: existing,
    retiredCycleIds: [],
    retiredProjectIds: [],
  });
  mocks.getCycle.mockResolvedValue({ status: "active" });
  mocks.getObjective.mockResolvedValue({});
  mocks.getProject.mockResolvedValue({});

  const result = await ensureExampleWorkspace();
  expect(result).toEqual(existing);
  expect(mocks.createCycle).not.toHaveBeenCalled();
});

test("resetExampleWorkspace archives the old example and creates a new tree", async () => {
  saveExampleState({
    active: true,
    current: tree("1"),
    retiredCycleIds: [],
    retiredProjectIds: [],
  });
  mocks.archiveCycle.mockResolvedValue({});
  stubCreate("2");

  const result = await resetExampleWorkspace();
  expect(result.cycleId).toBe("c2");
  expect(mocks.archiveCycle).toHaveBeenCalledWith("c1");
  expect(mocks.createCycle).toHaveBeenCalledTimes(1);
  const saved = loadExampleState();
  expect(saved.retiredCycleIds).toContain("c1");
  expect(saved.retiredProjectIds).toContain("p1");
  expect(saved.current?.cycleId).toBe("c2");
  expect(saved.active).toBe(true);
});
