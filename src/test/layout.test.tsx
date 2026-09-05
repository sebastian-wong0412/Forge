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

test("sidebar and main are sibling overflow regions inside the shell", () => {
  const { container } = render(
    <MemoryRouter>
      <Layout />
    </MemoryRouter>,
  );

  const shell = container.querySelector(".app-shell");
  const sidebar = container.querySelector(".sidebar");
  const main = container.querySelector("main.main");
  expect(shell).toBeTruthy();
  expect(sidebar?.parentElement).toBe(shell);
  expect(main?.parentElement).toBe(shell);
});
