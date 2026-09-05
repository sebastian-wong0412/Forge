import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { beforeEach, expect, test, vi } from "vitest";
import { ExampleProvider } from "../example/ExampleProvider";
import { SettingsProvider } from "../i18n";
import { markOnboardingCompleted } from "../lib/onboarding";
import { WelcomePage } from "../pages/WelcomePage";
import { translate } from "../i18n";

const enter = vi.fn();

vi.mock("../example/ExampleProvider", async () => {
  const actual = await vi.importActual<typeof import("../example/ExampleProvider")>(
    "../example/ExampleProvider",
  );
  return {
    ...actual,
    useExample: () => ({
      active: false,
      current: null,
      state: { active: false, current: null, retiredCycleIds: [], retiredProjectIds: [] },
      enter,
      exit: vi.fn(),
      reset: vi.fn(),
    }),
  };
});

function renderWelcome() {
  return render(
    <SettingsProvider>
      <MemoryRouter initialEntries={["/welcome"]}>
        <ExampleProvider>
          <Routes>
            <Route path="/welcome" element={<WelcomePage />} />
            <Route path="/today" element={<div>normal-today</div>} />
            <Route path="/cycles/:cycleId" element={<div>example-cycle</div>} />
          </Routes>
        </ExampleProvider>
      </MemoryRouter>
    </SettingsProvider>,
  );
}

beforeEach(() => {
  enter.mockReset();
});

test("fresh user sees Welcome copy", () => {
  renderWelcome();
  expect(screen.getByRole("heading", { name: "欢迎来到 Forge" })).toBeInTheDocument();
  expect(screen.getByText("看看一个意图，如何一步步成为行动。")).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "探索示例" })).toBeInTheDocument();
  expect(screen.getByRole("button", { name: "从零开始" })).toBeInTheDocument();
});

test("Start from scratch completes onboarding and leaves Welcome", async () => {
  renderWelcome();
  fireEvent.click(screen.getByRole("button", { name: "从零开始" }));
  expect(screen.getByText("normal-today")).toBeInTheDocument();
  expect(localStorage.getItem("forge.onboarding")).toContain("true");
});

test("Explore the example completes onboarding and opens the example cycle", async () => {
  enter.mockResolvedValue({
    cycleId: "c-example",
    objectiveId: "o",
    keyResultId: "k",
    projectId: "p",
    taskIds: [],
  });
  renderWelcome();
  fireEvent.click(screen.getByRole("button", { name: "探索示例" }));
  await waitFor(() => {
    expect(screen.getByText("example-cycle")).toBeInTheDocument();
  });
  expect(enter).toHaveBeenCalledTimes(1);
});

test("completed onboarding helper stays completed after Welcome scratch", () => {
  markOnboardingCompleted();
  expect(localStorage.getItem("forge.onboarding")).toContain("true");
});

test("welcome keys exist in English and Chinese", () => {
  expect(translate("en", "welcome.title")).toBe("Welcome to Forge");
  expect(translate("zh", "welcome.title")).toBe("欢迎来到 Forge");
  expect(translate("en", "welcome.explore")).toBe("Explore the example");
  expect(translate("zh", "welcome.explore")).toBe("探索示例");
  expect(translate("en", "example.banner.title")).toBe("Example Workspace");
  expect(translate("zh", "example.banner.title")).toBe("示例空间");
  expect(translate("en", "tour.vision.note")).toContain("not a page");
  expect(translate("zh", "tour.vision.note")).toContain("没有独立页面");
});
