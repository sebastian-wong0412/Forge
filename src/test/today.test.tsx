import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TodayView } from "../components/TodayView";
import type { Cycle } from "../api/types";
import { cycle, task, today } from "./fixtures";

const noop = () => undefined;

function renderToday(
  data = today(),
  extras: {
    localToday?: string;
    onDateChange?: (date: string) => void;
    cycles?: Cycle[];
  } = {},
) {
  return render(
    <MemoryRouter>
      <TodayView
        today={data}
        localToday={extras.localToday ?? "2026-08-30"}
        cycles={extras.cycles}
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
  expect(screen.queryByText("今天还没有安排任务")).not.toBeInTheDocument();
});

test("with cycles and no tasks, Today does not ask the user to create a cycle", () => {
  renderToday(today(), { cycles: [cycle({ name: "Q3 学习计划" })] });
  expect(screen.getByText("今天还没有安排任务")).toBeInTheDocument();
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
  expect(screen.getByText("继续规划")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: /Q3 学习计划/ })).toHaveAttribute(
    "href",
    "/cycles/c-active",
  );
  expect(screen.queryByRole("link", { name: /已结束的周期/ })).not.toBeInTheDocument();
});
