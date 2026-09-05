import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { TodayView } from "../components/TodayView";
import { cycle, today } from "./fixtures";

const noop = () => undefined;

function renderToday(cycles: ReturnType<typeof cycle>[]) {
  return render(
    <MemoryRouter>
      <TodayView
        today={today()}
        localToday="2026-08-30"
        cycles={cycles}
        onDateChange={noop}
        onStart={noop}
        onComplete={noop}
        onCancel={noop}
        onSchedule={noop}
        onUnschedule={noop}
      />
    </MemoryRouter>,
  );
}

test("shows first-use onboarding when there are no cycles", () => {
  renderToday([]);
  expect(screen.getByText("开始使用 Forge")).toBeInTheDocument();
  expect(screen.getByRole("link", { name: "创建我的第一个周期" })).toHaveAttribute(
    "href",
    "/cycles",
  );
  expect(screen.queryByText("今天还没有安排行动。")).not.toBeInTheDocument();
});

test("hides onboarding after a cycle exists", () => {
  renderToday([cycle()]);
  expect(screen.queryByText("开始使用 Forge")).not.toBeInTheDocument();
  expect(screen.getByText("今天还没有安排行动。")).toBeInTheDocument();
});
