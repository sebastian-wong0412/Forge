import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter } from "react-router-dom";
import { expect, test, vi } from "vitest";
import { ExampleBanner } from "../components/ExampleBanner";
import { SettingsProvider } from "../i18n";

const exit = vi.fn();
const reset = vi.fn();

vi.mock("../example/ExampleProvider", () => ({
  useExample: () => ({
    active: true,
    current: { cycleId: "c1", objectiveId: "o1", keyResultId: "k1", projectId: "p1", taskIds: [] },
    state: { active: true, current: null, retiredCycleIds: [], retiredProjectIds: [] },
    enter: vi.fn(),
    exit,
    reset,
  }),
}));

test("example banner can exit and open the concept tour", () => {
  render(
    <SettingsProvider>
      <MemoryRouter>
        <ExampleBanner />
      </MemoryRouter>
    </SettingsProvider>,
  );

  expect(screen.getByText("示例空间")).toBeInTheDocument();
  expect(screen.getByText("看看 Forge 如何把意图一步步转化为行动。")).toBeInTheDocument();
  fireEvent.click(screen.getByRole("button", { name: "离开示例" }));
  expect(exit).toHaveBeenCalledTimes(1);
  fireEvent.click(screen.getByRole("button", { name: "Forge 如何工作" }));
  expect(screen.getByRole("heading", { name: "Forge 如何工作" })).toBeInTheDocument();
  expect(screen.getByText("Vision — 你想走向哪里")).toBeInTheDocument();
  expect(screen.getByText(/当前版本还没有独立页面/)).toBeInTheDocument();
});
