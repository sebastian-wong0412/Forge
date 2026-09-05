import { fireEvent, render, screen } from "@testing-library/react";
import { DateField } from "../components/DateField";
import { SettingsProvider } from "../i18n";

function renderField(
  props: Partial<Parameters<typeof DateField>[0]> = {},
  language: "zh" | "en" = "zh",
) {
  const onChange = props.onChange ?? vi.fn();
  if (language === "en") {
    localStorage.setItem("forge.preferences", JSON.stringify({ language: "en", theme: "system" }));
  }
  const view = render(
    <SettingsProvider>
      <label htmlFor="due">
        {language === "en" ? "Date" : "日期"}
        <DateField id="due" value={props.value ?? ""} onChange={onChange} required={props.required} />
      </label>
    </SettingsProvider>,
  );
  return { onChange, ...view };
}

test("displays API dates as YYYY/MM/DD", () => {
  renderField({ value: "2026-09-05" });
  expect(screen.getByLabelText("日期")).toHaveValue("2026/09/05");
  expect(document.getElementById("due-picker")).toHaveValue("2026-09-05");
});

test("picker selection keeps the display format and emits ISO", () => {
  const { onChange } = renderField({ value: "" });
  fireEvent.change(document.getElementById("due-picker") as HTMLInputElement, {
    target: { value: "2026-09-05" },
  });
  expect(onChange).toHaveBeenCalledWith("2026-09-05");
  expect(screen.getByLabelText("日期")).toHaveValue("2026/09/05");
});

test("typed display dates emit ISO values", () => {
  const { onChange } = renderField({ value: "" });
  fireEvent.change(screen.getByLabelText("日期"), { target: { value: "2026/09/05" } });
  expect(onChange).toHaveBeenCalledWith("2026-09-05");
});

test("invalid dates are rejected", () => {
  const { onChange } = renderField({ value: "" });
  const input = screen.getByLabelText("日期") as HTMLInputElement;
  fireEvent.change(input, { target: { value: "2026/13/40" } });
  expect(onChange).not.toHaveBeenCalled();
  expect(input.validationMessage).toBe("请输入 YYYY/MM/DD");
});

test("empty dates clear the ISO value", () => {
  const { onChange } = renderField({ value: "2026-09-05" });
  fireEvent.change(screen.getByLabelText("日期"), { target: { value: "" } });
  expect(onChange).toHaveBeenCalledWith("");
});

test("English UI uses the same YYYY/MM/DD display", () => {
  renderField({ value: "2026-09-05" }, "en");
  const input = screen.getByLabelText("Date");
  expect(input).toHaveValue("2026/09/05");
  expect((input as HTMLInputElement).value).not.toMatch(/日/);
  expect(screen.getByRole("button", { name: "Choose date" })).toBeInTheDocument();
});
