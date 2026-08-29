import { render, screen } from "@testing-library/react";
import { TaskActions } from "../components/TaskActions";
import { task } from "./fixtures";

const noop = () => undefined;

function renderActions(status: "todo" | "in_progress" | "done" | "cancelled") {
  return render(
    <TaskActions
      task={task({ status })}
      onStart={noop}
      onComplete={noop}
      onCancel={noop}
      onSchedule={noop}
      onUnschedule={noop}
    />,
  );
}

test("todo shows Cancel", () => {
  renderActions("todo");
  expect(screen.getByRole("button", { name: "取消" })).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "开始" })).toBeInTheDocument();
});

test("done and cancelled do not show Cancel", () => {
  const { rerender } = renderActions("done");
  expect(screen.queryByRole("button", { name: "取消" })).not.toBeInTheDocument();

  rerender(
    <TaskActions
      task={task({ status: "cancelled" })}
      onStart={noop}
      onComplete={noop}
      onCancel={noop}
      onSchedule={noop}
      onUnschedule={noop}
    />,
  );
  expect(screen.queryByRole("button", { name: "取消" })).not.toBeInTheDocument();
});
