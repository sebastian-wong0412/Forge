import { openCycleShortcuts } from "../lib/cycles";
import { cycle } from "./fixtures";

test("prefers active cycles and omits closed or archived ones", () => {
  const selected = openCycleShortcuts([
    cycle({ id: "closed", name: "Closed", status: "closed", updated_at: "2026-09-05T10:00:00Z" }),
    cycle({ id: "plan", name: "Planning", status: "planning", updated_at: "2026-09-04T10:00:00Z" }),
    cycle({ id: "active", name: "Active", status: "active", updated_at: "2026-09-01T10:00:00Z" }),
    cycle({ id: "archived", name: "Archived", status: "archived", updated_at: "2026-09-06T10:00:00Z" }),
  ]);

  expect(selected.map((item) => item.id)).toEqual(["active", "plan"]);
});

test("limits open cycle shortcuts to five", () => {
  const selected = openCycleShortcuts(
    Array.from({ length: 7 }, (_, index) =>
      cycle({
        id: `c${index}`,
        name: `Cycle ${index}`,
        status: "planning",
        updated_at: `2026-09-0${index + 1}T10:00:00Z`,
      }),
    ),
  );
  expect(selected).toHaveLength(5);
});
