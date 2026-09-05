import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { beforeEach, expect, test, vi } from "vitest";
import { ExampleBanner } from "../components/ExampleBanner";
import { SettingsProvider } from "../i18n";

const exit = vi.fn();
const reset = vi.fn();

beforeEach(() => {
  exit.mockClear();
  reset.mockReset();
});

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

test("exit replaces the current example page with Today", () => {
  render(
    <SettingsProvider>
      <MemoryRouter initialEntries={["/cycles/c1"]}>
        <Routes>
          <Route path="/cycles/c1" element={<ExampleBanner />} />
          <Route path="/today" element={<div>Today page</div>} />
        </Routes>
      </MemoryRouter>
    </SettingsProvider>,
  );

  fireEvent.click(screen.getByRole("button", { name: "离开示例" }));
  expect(exit).toHaveBeenCalledTimes(1);
  expect(screen.getByText("Today page")).toBeInTheDocument();
});

test("reset failure is shown as an error, not muted copy", async () => {
  reset.mockRejectedValueOnce(new Error("reset exploded"));
  render(
    <SettingsProvider>
      <MemoryRouter>
        <ExampleBanner />
      </MemoryRouter>
    </SettingsProvider>,
  );

  fireEvent.click(screen.getByRole("button", { name: "重置示例" }));
  expect(await screen.findByRole("alert")).toHaveTextContent("reset exploded");
  await waitFor(() => {
    expect(screen.getByRole("button", { name: "重置示例" })).toBeEnabled();
  });
});
