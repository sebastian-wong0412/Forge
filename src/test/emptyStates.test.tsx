import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import {
  getCycle,
  getCycles,
  getKeyResults,
  getObjective,
  getProject,
  getProjects,
  getTasks,
} from "../api";
import { CyclesPage } from "../pages/CyclesPage";
import { ObjectiveDetailPage } from "../pages/ObjectiveDetailPage";
import { ProjectDetailPage } from "../pages/ProjectDetailPage";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getCycles: vi.fn(),
    getCycle: vi.fn(),
    getObjective: vi.fn(),
    getKeyResults: vi.fn(),
    getProjects: vi.fn(),
    getProject: vi.fn(),
    getTasks: vi.fn(),
  };
});

const stamp = "2026-08-30T09:00:00Z";

test("Cycle empty state is Chinese", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  render(
    <MemoryRouter>
      <CyclesPage />
    </MemoryRouter>,
  );
  expect(await screen.findByText("还没有周期")).toBeInTheDocument();
  expect(screen.queryByRole("link", { name: "创建第一个周期" })).not.toBeInTheDocument();
  expect(screen.queryByText("创建第一个周期")).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "创建周期" })).toBeInTheDocument();
});

test("Objective and Project empty states are Chinese", async () => {
  vi.mocked(getObjective).mockResolvedValue({
    id: "o1",
    cycle_id: "c1",
    title: "深度",
    description: null,
    status: "active",
    start_on: null,
    end_on: null,
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 2026",
    start_on: "2026-07-01",
    end_on: "2026-09-30",
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getKeyResults).mockResolvedValue([]);
  vi.mocked(getProjects).mockResolvedValue([]);

  render(
    <MemoryRouter initialEntries={["/objectives/o1"]}>
      <Routes>
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  await waitFor(() => {
    expect(screen.getByText("还没有关键结果")).toBeInTheDocument();
    expect(screen.getByText("还没有项目")).toBeInTheDocument();
  });
});

test("Task empty state is Chinese", async () => {
  vi.mocked(getProject).mockResolvedValue({
    id: "p1",
    objective_id: "o1",
    title: "Forge",
    description: null,
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getObjective).mockResolvedValue({
    id: "o1",
    cycle_id: "c1",
    title: "深度",
    description: null,
    status: "active",
    start_on: null,
    end_on: null,
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 2026",
    start_on: "2026-07-01",
    end_on: "2026-09-30",
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getTasks).mockResolvedValue([]);

  render(
    <MemoryRouter initialEntries={["/projects/p1"]}>
      <Routes>
        <Route path="/projects/:projectId" element={<ProjectDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  expect(await screen.findByText("还没有任务")).toBeInTheDocument();
});
