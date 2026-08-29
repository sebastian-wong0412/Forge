import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TodayView } from "../components/TodayView";
import { task, today } from "./fixtures";

const noop = () => undefined;

function renderToday(
  data = today(),
  extras: { localToday?: string; onDateChange?: (date: string) => void; hasCycles?: boolean } = {},
) {
  return render(
    <MemoryRouter>
      <TodayView
        today={data}
        localToday={extras.localToday ?? "2026-08-30"}
        hasCycles={extras.hasCycles ?? true}
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
  renderToday(today({ date: "2026-08-30" }));
  expect(screen.getByText("今日", { selector: ".page-kicker" })).toBeInTheDocument();
  expect(screen.getByText("2026年8月30日")).toBeInTheDocument();
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
  );

  expect(screen.getByText("今日计划")).toBeInTheDocument();
  expect(screen.getByText("已逾期")).toBeInTheDocument();
  expect(screen.getByText("未排期进行中")).toBeInTheDocument();
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
  );
  expect(screen.getByText("今日计划")).toBeInTheDocument();
  expect(screen.queryByText("已逾期")).not.toBeInTheDocument();
  expect(screen.queryByText("未排期进行中")).not.toBeInTheDocument();
  expect(screen.queryByText("今日完成")).not.toBeInTheDocument();
});

test("renders scheduled_on as a calendar date string", () => {
  renderToday(
    today({
      scheduled: [task({ title: "Dated task", scheduled_on: "2026-08-30" })],
    }),
  );
  expect(screen.getByText("安排于 2026-08-30")).toBeInTheDocument();
});

test("shows an empty Today state with a Cycles CTA", () => {
  renderToday(today(), { hasCycles: true });
  expect(screen.getByText("今天没有安排任务")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: "去创建周期" })).toHaveAttribute("href", "/cycles");
});
