import { render, screen } from "@testing-library/react";
import { ApiClientError } from "../api/client";
import { ErrorState } from "../components/ErrorState";

test("renders a human-readable API error without raw debug output", () => {
  const error = new ApiClientError(422, "domain", "当前状态不允许此操作。");
  render(<ErrorState message={error.message} onRetry={() => undefined} />);
  expect(screen.getByRole("alert")).toHaveTextContent("当前状态不允许此操作。");
  expect(screen.getByRole("button", { name: "重试" })).toBeInTheDocument();
  expect(screen.queryByText(/DomainError|AppError|stack/i)).not.toBeInTheDocument();
});
