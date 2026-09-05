import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TodayView } from "../components/TodayView";
import type { Cycle, Project } from "../api/types";
import { SettingsProvider } from "../i18n";
import { cycle, project, task, today } from "./fixtures";

const noop = () => undefined;

function renderToday(
  data = today(),
  extras: {
    localToday?: string;
    onDateChange?: (date: string) => void;
    cycles?: Cycle[];
    projects?: Project[];
    projectTitles?: Record<string, string>;
  } = {},
) {
  return render(
    <MemoryRouter>
      <TodayView
        today={data}
        localToday={extras.localToday ?? "2026-08-30"}
        cycles={extras.cycles}
        projects={extras.projects}
        projectTitles={extras.projectTitles}
        onDateChange={extras.onDateChange ?? noop}
        onStart={noop}
        onComplete={noop}
        onCancel={noop}
        onSchedule={noop}
        onUnschedule={noop}
      />
    </MemoryRouter>,
  );
}

test("renders the requested calendar date and Today heading", () => {
  renderToday(today({ date: "2026-08-30" }), { cycles: [cycle()] });
  expect(screen.getByText("今日", { selector: ".page-kicker" })).toBeInTheDocument();
  expect(screen.getByRole("heading", { name: "2026/08/30" })).toBeInTheDocument();
  expect(screen.getByText("2026/08/30", { selector: ".date-nav-current" })).toBeInTheDocument();
});

test("renders backend buckets without recalculating membership", () => {
  renderToday(
    today({
      scheduled: [task({ id: "s1", title: "Scheduled work" })],
      overdue: [task({ id: "o1", title: "Overdue work", scheduled_on: "2026-08-29" })],
      unscheduled_in_progress: [
        task({
          id: "p1",
          title: "Started work",
          status: "in_progress",
          scheduled_on: null,
        }),
      ],
      completed: [
        task({
          id: "c1",
          title: "Finished work",
          status: "done",
          completed_at: "2026-08-30T15:00:00Z",
        }),
      ],
    }),
    { cycles: [cycle()] },
  );

  expect(screen.getByText("今日计划")).toBeInTheDocument();
  expect(screen.getByText("已逾期")).toBeInTheDocument();
  expect(screen.getByText("进行中（未安排）")).toBeInTheDocument();
  expect(screen.getByText("今日完成")).toBeInTheDocument();
  expect(screen.getByText("Scheduled work")).toBeInTheDocument();
  expect(screen.getByText("Overdue work")).toBeInTheDocument();
  expect(screen.getByText("Started work")).toBeInTheDocument();
  expect(screen.getByText("Finished work")).toBeInTheDocument();
});

test("hides empty Today sections", () => {
  renderToday(
    today({
      scheduled: [task({ title: "Only scheduled" })],
    }),
    { cycles: [cycle()] },
  );
  expect(screen.getByText("今日计划")).toBeInTheDocument();
  expect(screen.queryByText("已逾期")).not.toBeInTheDocument();
  expect(screen.queryByText("进行中（未安排）")).not.toBeInTheDocument();
  expect(screen.queryByText("今日完成")).not.toBeInTheDocument();
});

test("renders scheduled_on as a calendar date string", () => {
  renderToday(
    today({
      scheduled: [task({ title: "Dated task", scheduled_on: "2026-08-30" })],
    }),
    { cycles: [cycle()] },
  );
  expect(screen.getByText("安排到 2026/08/30")).toBeInTheDocument();
});

test("without cycles, Today shows first-cycle onboarding", () => {
  renderToday(today(), { cycles: [] });
  expect(screen.getByRole("link", { name: "创建我的第一个周期" })).toHaveAttribute(
    "href",
    "/cycles",
  );
  expect(screen.queryByText("今天还没有安排行动。")).not.toBeInTheDocument();
});

test("with cycles and no tasks, Today does not ask the user to create a cycle", () => {
  renderToday(today(), { cycles: [cycle({ name: "Q3 学习计划" })] });
  expect(screen.getByText("今天还没有安排行动。")).toBeInTheDocument();
  expect(screen.queryByText("去创建周期")).not.toBeInTheDocument();
  expect(screen.queryByRole("link", { name: "创建我的第一个周期" })).not.toBeInTheDocument();
  expect(screen.queryByRole("link", { name: "创建周期" })).not.toBeInTheDocument();
});

test("with cycles and no tasks, Today links existing open cycles", () => {
  renderToday(today(), {
    cycles: [
      cycle({ id: "c-active", name: "Q3 学习计划", status: "active" }),
      cycle({
        id: "c-closed",
        name: "已结束的周期",
        status: "closed",
        updated_at: "2026-08-31T09:00:00Z",
      }),
    ],
  });
  expect(screen.getByText("打开一个周期")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: /Q3 学习计划/ })).toHaveAttribute(
    "href",
    "/cycles/c-active",
  );
  expect(screen.getByRole("link", { name: "查看进行中的项目" })).toHaveAttribute(
    "href",
    "/cycles/c-active",
  );
  expect(screen.queryByRole("link", { name: /已结束的周期/ })).not.toBeInTheDocument();
});

test("Today empty CTA goes to a workable project when one exists", () => {
  renderToday(today(), {
    cycles: [cycle({ id: "c-active", name: "Q3 学习计划", status: "active" })],
    projects: [project({ id: "p-ml", title: "Machine Learning Course Project" })],
  });
  expect(screen.getByRole("link", { name: "查看进行中的项目" })).toHaveAttribute(
    "href",
    "/projects/p-ml",
  );
  expect(screen.getByRole("link", { name: /Machine Learning Course Project/ })).toHaveAttribute(
    "href",
    "/projects/p-ml",
  );
});

test("Today task rows show project context under the title", () => {
  renderToday(
    today({
      scheduled: [
        task({
          title: "Review gradient descent notes",
          project_id: "p-ml",
        }),
      ],
      completed: [
        task({
          id: "c1",
          title: "Complete Chapter 2 exercises",
          status: "done",
          project_id: "p-ml",
          completed_at: "2026-08-30T15:00:00Z",
        }),
      ],
    }),
    {
      cycles: [cycle()],
      projectTitles: { "p-ml": "Machine Learning Course Project" },
    },
  );
  expect(screen.getByText("Review gradient descent notes")).toBeInTheDocument();
  expect(screen.getByText("Complete Chapter 2 exercises")).toBeInTheDocument();
  expect(screen.getAllByText("Machine Learning Course Project")).toHaveLength(2);
  expect(screen.getByRole("link", { name: "从项目中选择" })).toBeInTheDocument();
});

test("English Today empty state uses catalog copy", () => {
  localStorage.setItem("forge.preferences", JSON.stringify({ language: "en", theme: "system" }));
  render(
    <MemoryRouter>
      <SettingsProvider>
        <TodayView
          today={today()}
          localToday="2026-08-30"
          cycles={[cycle()]}
          onDateChange={noop}
          onStart={noop}
          onComplete={noop}
          onCancel={noop}
          onSchedule={noop}
          onUnschedule={noop}
        />
      </SettingsProvider>
    </MemoryRouter>,
  );
  expect(screen.getByText("Nothing planned for today.")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: "View active projects" })).toHaveAttribute(
    "href",
    "/cycles/c1",
  );
});
