import { render, screen, waitFor } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { beforeEach, expect, test, vi } from "vitest";
import { getCycles } from "../api";
import { OnboardingGate } from "../components/OnboardingGate";
import { ExampleProvider } from "../example/ExampleProvider";
import { SettingsProvider } from "../i18n";
import { markOnboardingCompleted } from "../lib/onboarding";
import { cycle } from "./fixtures";

vi.mock("../api", () => ({
  getCycles: vi.fn(),
}));

function renderGate(path: string) {
  return render(
    <SettingsProvider>
      <MemoryRouter initialEntries={[path]}>
        <ExampleProvider>
          <OnboardingGate>
            <Routes>
              <Route path="/welcome" element={<div>welcome-screen</div>} />
              <Route path="/today" element={<div>today-screen</div>} />
            </Routes>
          </OnboardingGate>
        </ExampleProvider>
      </MemoryRouter>
    </SettingsProvider>,
  );
}

beforeEach(() => {
  vi.mocked(getCycles).mockReset();
});

test("fresh user with an empty database is sent to Welcome", async () => {
  vi.mocked(getCycles).mockResolvedValue([]);
  renderGate("/today");
  await waitFor(() => {
    expect(screen.getByText("welcome-screen")).toBeInTheDocument();
  });
});

test("existing user with real cycles skips Welcome", async () => {
  vi.mocked(getCycles).mockResolvedValue([cycle({ id: "user-cycle" })]);
  renderGate("/today");
  await waitFor(() => {
    expect(screen.getByText("today-screen")).toBeInTheDocument();
  });
  expect(localStorage.getItem("forge.onboarding")).toContain("true");
  expect(screen.queryByText("welcome-screen")).not.toBeInTheDocument();
});

test("completed onboarding does not show Welcome again", async () => {
  markOnboardingCompleted();
  renderGate("/today");
  await waitFor(() => {
    expect(screen.getByText("today-screen")).toBeInTheDocument();
  });
  expect(getCycles).not.toHaveBeenCalled();
});
