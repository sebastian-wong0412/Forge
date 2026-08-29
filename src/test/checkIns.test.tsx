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
    note: "First pass",
    checked_on: "2026-08-30",
    created_at: "2026-08-30T12:00:00Z",
    updated_at: "2026-08-30T12:00:00Z",
  });
  const onKeyResultChanged = vi.fn().mockResolvedValue(undefined);

  render(<CheckInPanel keyResultId="kr-1" onKeyResultChanged={onKeyResultChanged} />);

  await screen.findByText("还没有进展记录。");
  fireEvent.change(screen.getByLabelText("数值"), { target: { value: "1" } });
  fireEvent.change(screen.getByLabelText("备注"), { target: { value: "First pass" } });
  fireEvent.click(screen.getByRole("button", { name: "添加进展记录" }));

  await waitFor(() => {
    expect(createCheckInMock).toHaveBeenCalled();
    expect(getCheckInsMock).toHaveBeenCalledTimes(2);
    expect(onKeyResultChanged).toHaveBeenCalled();
    expect(screen.getByText("1")).toBeInTheDocument();
    expect(screen.getByText("First pass")).toBeInTheDocument();
  });
});
