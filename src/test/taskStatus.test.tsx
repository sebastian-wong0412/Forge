import { render, screen } from "@testing-library/react";
import { TaskStatusIcon } from "../components/TaskStatusIcon";

test("renders visible status marks that are not color-only", () => {
  const { rerender } = render(<TaskStatusIcon status="todo" />);
  expect(screen.getByLabelText("待开始")).toHaveTextContent("[ ]");

  rerender(<TaskStatusIcon status="in_progress" />);
  expect(screen.getByLabelText("进行中")).toHaveTextContent("[→]");

  rerender(<TaskStatusIcon status="done" />);
  expect(screen.getByLabelText("已完成")).toHaveTextContent("[✓]");
});
