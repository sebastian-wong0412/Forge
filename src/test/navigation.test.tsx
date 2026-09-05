import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import {
  getCycle,
  getKeyResults,
  getObjective,
  getObjectives,
  getProject,
  getProjects,
  getReviews,
  getTask,
  getTasks,
} from "../api";
import { SettingsProvider } from "../i18n";
import { CycleDetailPage } from "../pages/CycleDetailPage";
import { ObjectiveDetailPage } from "../pages/ObjectiveDetailPage";
import { ProjectDetailPage } from "../pages/ProjectDetailPage";
import { TaskDetailPage } from "../pages/TaskDetailPage";
import { task } from "./fixtures";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getCycle: vi.fn(),
    getObjectives: vi.fn(),
    getReviews: vi.fn(),
    getObjective: vi.fn(),
    getKeyResults: vi.fn(),
    getProjects: vi.fn(),
    getProject: vi.fn(),
    getTasks: vi.fn(),
    getTask: vi.fn(),
  };
});

beforeEach(() => {
  window.history.replaceState({}, "");
});

const stamp = "2026-08-30T09:00:00Z";
const cycle = {
  id: "c1",
  name: "Q3 2026",
  start_on: "2026-07-01",
  end_on: "2026-09-30",
  status: "active" as const,
  created_at: stamp,
  updated_at: stamp,
};
const objective = {
  id: "o1",
  cycle_id: "c1",
  title: "深度",
  description: null,
  status: "active" as const,
  start_on: null,
  end_on: null,
  created_at: stamp,
  updated_at: stamp,
};
const project = {
  id: "p1",
  objective_id: "o1",
  title: "Forge",
  description: null,
  status: "active" as const,
  created_at: stamp,
  updated_at: stamp,
};

function mockHierarchy() {
  vi.mocked(getCycle).mockResolvedValue(cycle);
  vi.mocked(getObjectives).mockResolvedValue([objective]);
  vi.mocked(getReviews).mockResolvedValue([]);
  vi.mocked(getObjective).mockResolvedValue(objective);
  vi.mocked(getKeyResults).mockResolvedValue([]);
  vi.mocked(getProjects).mockResolvedValue([project]);
  vi.mocked(getProject).mockResolvedValue(project);
  vi.mocked(getTasks).mockResolvedValue([]);
  vi.mocked(getTask).mockResolvedValue(task({ id: "t1", project_id: "p1", title: "README" }));
}

function renderEn(ui: React.ReactNode, path: string) {
  localStorage.setItem("forge.preferences", JSON.stringify({ language: "en", theme: "system" }));
  return render(
    <SettingsProvider>
      <MemoryRouter initialEntries={[path]}>{ui}</MemoryRouter>
    </SettingsProvider>,
  );
}

test("Cycle detail back falls back to the cycle list", async () => {
  mockHierarchy();
  render(
    <MemoryRouter initialEntries={["/cycles/c1"]}>
      <Routes>
        <Route path="/cycles" element={<div>Cycles list</div>} />
        <Route path="/cycles/:cycleId" element={<CycleDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  expect(await screen.findByRole("button", { name: "‹ 返回" })).toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "‹ 返回" }));
  expect(screen.getByText("Cycles list")).toBeInTheDocument();
});

test("Objective detail back falls back to the parent cycle", async () => {
  mockHierarchy();
  render(
    <MemoryRouter initialEntries={["/objectives/o1"]}>
      <Routes>
        <Route path="/cycles/:cycleId" element={<div>Cycle page</div>} />
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  fireEvent.click(await screen.findByRole("button", { name: "‹ 返回" }));
  expect(screen.getByText("Cycle page")).toBeInTheDocument();
});

test("Project detail back falls back to the parent objective", async () => {
  mockHierarchy();
  render(
    <MemoryRouter initialEntries={["/projects/p1"]}>
      <Routes>
        <Route path="/objectives/:objectiveId" element={<div>Objective page</div>} />
        <Route path="/projects/:projectId" element={<ProjectDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  fireEvent.click(await screen.findByRole("button", { name: "‹ 返回" }));
  expect(screen.getByText("Objective page")).toBeInTheDocument();
});

test("Task detail back uses history when a previous page exists", async () => {
  mockHierarchy();
  render(
    <MemoryRouter initialEntries={["/today", "/tasks/t1"]}>
      <Routes>
        <Route path="/today" element={<div>Today page</div>} />
        <Route path="/projects/:projectId" element={<div>Project page</div>} />
        <Route path="/tasks/:taskId" element={<TaskDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  window.history.replaceState({ idx: 1 }, "");
  fireEvent.click(await screen.findByRole("button", { name: "‹ 返回" }));
  await waitFor(() => {
    expect(screen.getByText("Today page")).toBeInTheDocument();
  });
  expect(screen.queryByText("Project page")).not.toBeInTheDocument();
});

test("Task detail back falls back to the parent project", async () => {
  mockHierarchy();
  render(
    <MemoryRouter initialEntries={["/tasks/t1"]}>
      <Routes>
        <Route path="/projects/:projectId" element={<div>Project page</div>} />
        <Route path="/tasks/:taskId" element={<TaskDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  fireEvent.click(await screen.findByRole("button", { name: "‹ 返回" }));
  expect(screen.getByText("Project page")).toBeInTheDocument();
});

test("Back button uses English copy", async () => {
  mockHierarchy();
  renderEn(
    <Routes>
      <Route path="/cycles/:cycleId" element={<CycleDetailPage />} />
    </Routes>,
    "/cycles/c1",
  );

  expect(await screen.findByRole("button", { name: "‹ Back" })).toBeInTheDocument();
});
