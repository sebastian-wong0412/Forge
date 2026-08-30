import { statusLabel } from "../lib/status";

test("maps task and planning statuses to Chinese", () => {
  expect(statusLabel("todo")).toBe("待开始");
  expect(statusLabel("in_progress")).toBe("进行中");
  expect(statusLabel("done")).toBe("已完成");
  expect(statusLabel("cancelled")).toBe("已取消");
  expect(statusLabel("planning")).toBe("规划中");
  expect(statusLabel("active")).toBe("进行中");
  expect(statusLabel("closed")).toBe("已结束");
  expect(statusLabel("archived")).toBe("已归档");
  expect(statusLabel("draft")).toBe("未开始");
  expect(statusLabel("not_started")).toBe("未开始");
  expect(statusLabel("achieved")).toBe("已达成");
});
