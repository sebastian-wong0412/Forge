import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import {
  getCycle,
  getCycles,
  getKeyResults,
  getObjective,
  getObjectives,
  getProject,
  getProjects,
  getReviews,
  getTasks,
} from "../api";
import { CycleDetailPage } from "../pages/CycleDetailPage";
import { CyclesPage } from "../pages/CyclesPage";
import { ObjectiveDetailPage } from "../pages/ObjectiveDetailPage";
import { ProjectDetailPage } from "../pages/ProjectDetailPage";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getCycles: vi.fn(),
    getCycle: vi.fn(),
    getObjectives: vi.fn(),
    getObjective: vi.fn(),
    getKeyResults: vi.fn(),
    getProjects: vi.fn(),
    getProject: vi.fn(),
    getTasks: vi.fn(),
    getReviews: vi.fn(),
  };
});

const stamp = "2026-08-30T09:00:00Z";

test("Cycle list empty state explains the next action", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  render(
    <MemoryRouter>
      <CyclesPage />
    </MemoryRouter>,
  );
  expect(await screen.findByText("还没有周期")).toBeInTheDocument();
  expect(screen.queryByRole("link", { name: "创建第一个周期" })).not.toBeInTheDocument();
  expect(screen.queryByText("创建第一个周期")).not.toBeInTheDocument();
  expect(screen.queryByRole("link", { name: "创建周期" })).not.toBeInTheDocument();
  expect(screen.getByRole("button", { name: "创建周期" })).toBeInTheDocument();
});

test("Cycle without objectives shows an execution-focused empty state", async () => {
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 2026",
    start_on: "2026-07-01",
    end_on: "2026-09-30",
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getObjectives).mockResolvedValue([]);
  vi.mocked(getReviews).mockResolvedValue([]);

  render(
    <MemoryRouter initialEntries={["/cycles/c1"]}>
      <Routes>
        <Route path="/cycles/:cycleId" element={<CycleDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  expect(await screen.findByText("这个周期，你想推进什么？")).toBeInTheDocument();
  expect(screen.getByText("先定义一个目标，让这个周期有清晰的方向。")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: "添加目标" })).toHaveAttribute(
    "href",
    "#create-objective",
  );
});

test("Objective empty states explain KR measurement and project path", async () => {
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
    expect(screen.getByText("让进展变得可衡量。")).toBeInTheDocument();
    expect(screen.getByText("把这个目标变成可以推进的事情。")).toBeInTheDocument();
  });
  expect(screen.getByRole("link", { name: "添加关键结果" })).toHaveAttribute(
    "href",
    "#create-key-result",
  );
  expect(screen.getByRole("link", { name: "创建项目" })).toHaveAttribute("href", "#create-project");
  expect(screen.queryByText("还没有任务")).not.toBeInTheDocument();
});

test("Project empty state asks for a concrete task", async () => {
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

  expect(await screen.findByText("这个项目还没有具体行动。")).toBeInTheDocument();
  expect(screen.getByText("把项目拆成一件下一步可以真正执行的任务。")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: "添加任务" })).toHaveAttribute("href", "#create-task");
});
