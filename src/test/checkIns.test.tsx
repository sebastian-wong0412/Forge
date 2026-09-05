import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { createCheckIn, getCheckIns } from "../api";
import { CheckInPanel } from "../components/CheckInPanel";

vi.mock("../api", async () => {
  const actual = await vi.importActual<typeof import("../api")>("../api");
  return {
    ...actual,
    getCheckIns: vi.fn(),
    createCheckIn: vi.fn(),
  };
});

const getCheckInsMock = vi.mocked(getCheckIns);
const createCheckInMock = vi.mocked(createCheckIn);

test("reloads check-in history after create", async () => {
  getCheckInsMock
    .mockResolvedValueOnce([])
    .mockResolvedValueOnce([
      {
        id: "c1",
        key_result_id: "kr-1",
        value: 1,
        state: null,
        note: "First pass",
        checked_on: "2026-08-30",
        created_at: "2026-08-30T12:00:00Z",
        updated_at: "2026-08-30T12:00:00Z",
      },
    ]);
  createCheckInMock.mockResolvedValue({
    id: "c1",
    key_result_id: "kr-1",
    value: 1,
    state: null,
    note: "First pass",
    checked_on: "2026-08-30",
    created_at: "2026-08-30T12:00:00Z",
    updated_at: "2026-08-30T12:00:00Z",
  });
  const onKeyResultChanged = vi.fn().mockResolvedValue(undefined);

  render(
    <CheckInPanel
      keyResultId="kr-1"
      progressKind="numeric"
      onKeyResultChanged={onKeyResultChanged}
    />,
  );

  expect(await screen.findByText("记录关键结果进展")).toBeInTheDocument();
  expect(screen.getByText("更新你目前距离这个关键结果还有多远。")).toBeInTheDocument();
  expect(screen.getByText("还没有进展记录。")).toBeInTheDocument();
  expect(screen.queryByRole("button", { name: "标记完成" })).not.toBeInTheDocument();
  fireEvent.change(screen.getByLabelText("数值"), { target: { value: "1" } });
  fireEvent.change(screen.getByLabelText("备注"), { target: { value: "First pass" } });
  fireEvent.click(screen.getByRole("button", { name: "记录进展" }));

  await waitFor(() => {
    expect(createCheckInMock).toHaveBeenCalled();
    expect(getCheckInsMock).toHaveBeenCalledTimes(2);
    expect(onKeyResultChanged).toHaveBeenCalled();
    expect(screen.getByText("1")).toBeInTheDocument();
    expect(screen.getByText("First pass")).toBeInTheDocument();
  });
});

test("qualitative check-in requires a note", async () => {
  getCheckInsMock.mockResolvedValue([]);
  render(
    <CheckInPanel
      keyResultId="kr-2"
      progressKind="qualitative"
      onKeyResultChanged={vi.fn().mockResolvedValue(undefined)}
    />,
  );

  await screen.findByText("还没有进展记录。");
  expect(screen.getByLabelText("说明")).toBeRequired();
  expect(screen.queryByLabelText("数值")).not.toBeInTheDocument();
});
