import { render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { Breadcrumbs } from "../components/Breadcrumbs";

test("breadcrumb links navigate to parent routes", () => {
  render(
    <MemoryRouter>
      <Breadcrumbs
        items={[
          { label: "周期", to: "/cycles" },
          { label: "Q3 2026", to: "/cycles/c1" },
          { label: "Improve technical depth", to: "/objectives/o1" },
          { label: "Forge", to: "/projects/p1" },
          { label: "Implement Today view" },
        ]}
      />
    </MemoryRouter>,
  );

  expect(screen.getByRole("link", { name: "周期" })).toHaveAttribute("href", "/cycles");
  expect(screen.getByRole("link", { name: "Q3 2026" })).toHaveAttribute("href", "/cycles/c1");
  expect(screen.getByRole("link", { name: "Improve technical depth" })).toHaveAttribute(
    "href",
    "/objectives/o1",
  );
  expect(screen.getByRole("link", { name: "Forge" })).toHaveAttribute("href", "/projects/p1");
  expect(screen.getByText("Implement Today view")).toBeInTheDocument();
});
