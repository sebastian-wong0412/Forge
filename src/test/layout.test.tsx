import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Layout } from "../components/Layout";

test("sidebar links to Today, Cycles, and Settings", () => {
  render(
    <MemoryRouter>
      <Layout />
    </MemoryRouter>,
  );

  expect(screen.getByRole("link", { name: "今日" })).toHaveAttribute("href", "/today");
  expect(screen.getByRole("link", { name: "周期" })).toHaveAttribute("href", "/cycles");
  expect(screen.getByRole("link", { name: "设置" })).toHaveAttribute("href", "/settings");
  expect(screen.queryByText("即将推出")).not.toBeInTheDocument();
});
