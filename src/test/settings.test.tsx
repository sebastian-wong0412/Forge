import { fireEvent, render, screen } from "@testing-library/react";
import { MemoryRouter, Route, Routes } from "react-router-dom";
import { Layout } from "../components/Layout";
import { SettingsProvider } from "../i18n";
import { SettingsPage } from "../pages/SettingsPage";

function renderSettings() {
  return render(
    <SettingsProvider>
      <MemoryRouter initialEntries={["/settings"]}>
        <Routes>
          <Route element={<Layout />}>
            <Route path="/settings" element={<SettingsPage />} />
          </Route>
        </Routes>
      </MemoryRouter>
    </SettingsProvider>,
  );
}

test("renders Settings general and about sections", async () => {
  renderSettings();
  expect(screen.getByRole("heading", { name: "设置" })).toBeInTheDocument();
  expect(screen.getByText("通用")).toBeInTheDocument();
  expect(screen.getByText("关于 Forge")).toBeInTheDocument();
  expect(screen.getByLabelText("语言")).toHaveValue("system");
  expect(screen.getByLabelText("主题")).toHaveValue("system");
  expect(await screen.findByText(/0\.3\.1/)).toBeInTheDocument();
  expect(screen.getByText(/MIT License/)).toBeInTheDocument();
});

test("switching language updates navigation immediately", async () => {
  renderSettings();
  await screen.findByText(/0\.3\.1/);
  fireEvent.change(screen.getByLabelText("语言"), { target: { value: "en" } });
  expect(screen.getByRole("link", { name: "Today" })).toHaveAttribute("href", "/today");
  expect(screen.getByRole("link", { name: "Cycles" })).toHaveAttribute("href", "/cycles");
  expect(screen.getByRole("link", { name: "Settings" })).toHaveAttribute("href", "/settings");
  expect(screen.getByRole("heading", { name: "Settings" })).toBeInTheDocument();
});

test("switching theme sets data-theme immediately", async () => {
  renderSettings();
  await screen.findByText(/0\.3\.1/);
  fireEvent.change(screen.getByLabelText("主题"), { target: { value: "dark" } });
  expect(document.documentElement.dataset.theme).toBe("dark");
});
