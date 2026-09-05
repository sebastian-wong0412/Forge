import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { createTask, getCycle, getObjective, getProject, getTasks, scheduleTask } from "../api";
import { localCalendarDate } from "../lib/dates";
import { ProjectDetailPage } from "../pages/ProjectDetailPage";
import { task } from "./fixtures";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getProject: vi.fn(),
    getObjective: vi.fn(),
    getCycle: vi.fn(),
    getTasks: vi.fn(),
    createTask: vi.fn(),
    scheduleTask: vi.fn(),
  };
});

const stamp = "2026-08-30T09:00:00Z";

function mockProjectTree() {
  vi.mocked(getProject).mockResolvedValue({
    id: "p1",
    objective_id: "o1",
    title: "Machine Learning Course Project",
    description: null,
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getObjective).mockResolvedValue({
    id: "o1",
    cycle_id: "c1",
    title: "Build strong foundations in machine learning",
    description: null,
    status: "active",
    start_on: null,
    end_on: null,
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 Learning Plan",
    start_on: "2026-07-01",
    end_on: "2026-09-30",
    status: "active",
    created_at: stamp,
    updated_at: stamp,
  });
}

test("creating a task offers scheduling it for today", async () => {
  mockProjectTree();
  const created = task({
    id: "t-new",
    title: "Review gradient descent notes",
    scheduled_on: null,
  });
  vi.mocked(getTasks).mockResolvedValue([]);
  vi.mocked(createTask).mockResolvedValue(created);
  vi.mocked(scheduleTask).mockImplementation(async (_id, date) => ({
    ...created,
    scheduled_on: date,
  }));

  render(
    <MemoryRouter initialEntries={["/projects/p1"]}>
      <Routes>
        <Route path="/projects/:projectId" element={<ProjectDetailPage />} />
        <Route path="/today" element={<div>Today page</div>} />
      </Routes>
    </MemoryRouter>,
  );

  fireEvent.change(await screen.findByLabelText("标题"), {
    target: { value: "Review gradient descent notes" },
  });
  fireEvent.click(screen.getByRole("button", { name: "创建任务" }));

  expect(await screen.findByText("任务已创建")).toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "安排到今天" }));

  await waitFor(() => {
    expect(scheduleTask).toHaveBeenCalledWith("t-new", localCalendarDate());
    expect(screen.getByRole("link", { name: "查看今日" })).toHaveAttribute("href", "/today");
  });
});

test("draft projects can still add tasks", async () => {
  vi.mocked(getProject).mockResolvedValue({
    id: "p1",
    objective_id: "o1",
    title: "Machine Learning Course Project",
    description: null,
    status: "draft",
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getObjective).mockResolvedValue({
    id: "o1",
    cycle_id: "c1",
    title: "Build strong foundations",
    description: null,
    status: "active",
    start_on: null,
    end_on: null,
    created_at: stamp,
    updated_at: stamp,
  });
  vi.mocked(getCycle).mockResolvedValue({
    id: "c1",
    name: "Q3 Learning Plan",
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

  expect(await screen.findByText("现在就可以添加任务。准备开始推进时，再开始这个项目。")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "创建任务" })).toBeEnabled();
});
