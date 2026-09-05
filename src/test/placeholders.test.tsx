import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { getCycle, getCycles, getKeyResults, getObjective, getProject, getProjects, getTasks } from "../api";
import { SettingsProvider } from "../i18n";
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

function mockObjective() {
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
}

function renderEn(ui: React.ReactNode) {
  localStorage.setItem("forge.preferences", JSON.stringify({ language: "en", theme: "system" }));
  return render(<SettingsProvider>{ui}</SettingsProvider>);
}

test("Chinese create-form placeholders use catalog examples", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  mockObjective();
  render(
    <MemoryRouter initialEntries={["/cycles", "/objectives/o1"]} initialIndex={0}>
      <Routes>
        <Route path="/cycles" element={<CyclesPage />} />
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  expect(await screen.findByPlaceholderText("示例：Q3 学习计划")).toBeInTheDocument();
});

test("English create-form placeholders use catalog examples", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  renderEn(
    <MemoryRouter>
      <CyclesPage />
    </MemoryRouter>,
  );

  expect(await screen.findByPlaceholderText("e.g. Q3 Learning Plan")).toBeInTheDocument();
  expect(screen.getAllByPlaceholderText("YYYY/MM/DD").length).toBeGreaterThan(0);
});

test("Key Result title placeholder changes with progress kind", async () => {
  mockObjective();
  render(
    <MemoryRouter initialEntries={["/objectives/o1"]}>
      <Routes>
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  const title = await screen.findByLabelText("标题");
  expect(title).toHaveAttribute("placeholder", "示例：完成 10 个项目");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "percentage" } });
  expect(title).toHaveAttribute("placeholder", "示例：将课程完成度提升至 80%");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "milestone" } });
  expect(title).toHaveAttribute("placeholder", "示例：完成毕业项目");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "qualitative" } });
  expect(title).toHaveAttribute("placeholder", "示例：形成完整的学习方法论");
});

test("English Key Result placeholders change with progress kind", async () => {
  mockObjective();
  renderEn(
    <MemoryRouter initialEntries={["/objectives/o1"]}>
      <Routes>
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  const title = await screen.findByLabelText("Title");
  expect(title).toHaveAttribute("placeholder", "e.g. Complete 10 projects");
  fireEvent.change(screen.getByLabelText("Type"), { target: { value: "percentage" } });
  expect(title).toHaveAttribute("placeholder", "e.g. Raise course completion to 80%");
});

test("create-form placeholders stay in one learning scenario and avoid Forge-specific copy", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  mockObjective();
  render(
    <MemoryRouter initialEntries={["/objectives/o1"]}>
      <Routes>
        <Route path="/objectives/:objectiveId" element={<ObjectiveDetailPage />} />
      </Routes>
    </MemoryRouter>,
  );

  expect(await screen.findByPlaceholderText("示例：完成 10 个项目")).toBeInTheDocument();
  expect(screen.getByPlaceholderText("示例：机器学习课程项目")).toBeInTheDocument();
  expect(screen.queryByPlaceholderText(/Forge/i)).not.toBeInTheDocument();
});

test("task placeholder stays in the same learning scenario", async () => {
  vi.mocked(getProject).mockResolvedValue({
    id: "p1",
    objective_id: "o1",
    title: "机器学习课程项目",
    description: null,
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getObjective).mockResolvedValue({
    id: "o1",
    cycle_id: "c1",
    title: "提升数据科学能力",
    description: null,
    status: "active",
    start_on: null,
    end_on: null,
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 学习计划",
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

  expect(await screen.findByPlaceholderText("示例：完成第 3 章习题")).toBeInTheDocument();
  expect(screen.queryByPlaceholderText(/Forge/i)).not.toBeInTheDocument();
});
