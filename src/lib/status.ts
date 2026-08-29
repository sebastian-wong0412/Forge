const LABELS: Record<string, string> = {
  planning: "规划中",
  active: "进行中",
  closed: "已结束",
  archived: "已归档",
  draft: "草稿",
  todo: "待开始",
  in_progress: "进行中",
  done: "已完成",
  completed: "已完成",
  cancelled: "已取消",
};

export function statusLabel(status: string): string {
  return LABELS[status] ?? status;
}
