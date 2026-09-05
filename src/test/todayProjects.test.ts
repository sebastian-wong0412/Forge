import { executionDestination, uniqueProjectIds } from "../lib/todayProjects";
import { cycle, project, task, today } from "./fixtures";

test("uniqueProjectIds collects every Today bucket", () => {
  expect(
    uniqueProjectIds(
      today({
        scheduled: [task({ project_id: "a" })],
        overdue: [task({ id: "o", project_id: "b" })],
        unscheduled_in_progress: [task({ id: "u", project_id: "a" })],
        completed: [task({ id: "c", project_id: "c" })],
      }),
    ).sort(),
  ).toEqual(["a", "b", "c"]);
});

test("executionDestination prefers a workable project over a cycle list", () => {
  expect(
    executionDestination(
      [cycle({ id: "c-active", status: "active" })],
      [project({ id: "p-ml", status: "active" })],
    ),
  ).toBe("/projects/p-ml");
});

test("executionDestination falls back to the first open cycle", () => {
  expect(executionDestination([cycle({ id: "c-active", status: "active" })])).toBe(
    "/cycles/c-active",
  );
});

test("executionDestination falls back to the cycle list", () => {
  expect(executionDestination([])).toBe("/cycles");
});
