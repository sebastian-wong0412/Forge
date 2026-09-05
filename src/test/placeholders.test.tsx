import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { getCycle, getCycles, getKeyResults, getObjective, getProjects } from "../api";
import { SettingsProvider } from "../i18n";
import { CyclesPage } from "../pages/CyclesPage";
import { ObjectiveDetailPage } from "../pages/ObjectiveDetailPage";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getCycles: vi.fn(),
    getCycle: vi.fn(),
    getObjective: vi.fn(),
    getKeyResults: vi.fn(),
    getProjects: vi.fn(),
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
  expect(title).toHaveAttribute("placeholder", "示例：阅读 20 篇论文");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "percentage" } });
  expect(title).toHaveAttribute("placeholder", "示例：完成度达到 80%");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "milestone" } });
  expect(title).toHaveAttribute("placeholder", "示例：完成原型");

  fireEvent.change(screen.getByLabelText("类型"), { target: { value: "qualitative" } });
  expect(title).toHaveAttribute("placeholder", "示例：建立更稳健的工作流");
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
  expect(title).toHaveAttribute("placeholder", "e.g. Read 20 papers");
  fireEvent.change(screen.getByLabelText("Type"), { target: { value: "percentage" } });
  expect(title).toHaveAttribute("placeholder", "e.g. Reach 80% completion");
});
