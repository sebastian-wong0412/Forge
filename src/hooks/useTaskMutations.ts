import { useState } from "react";
import {
  cancelTask,
  completeTask,
  scheduleTask,
  startTask,
  type IsoDate,
  type Task,
} from "../api";
import { tCurrent } from "../i18n";

export function useTaskMutations(onChanged: () => Promise<void> | void) {
  const [busyId, setBusyId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [scheduling, setScheduling] = useState<Task | null>(null);

  async function run(task: Task, action: () => Promise<unknown>) {
    setBusyId(task.id);
    setError(null);
    try {
      await action();
      await onChanged();
    } catch (err) {
      setError(err instanceof Error ? err.message : tCurrent("error.taskUpdateFailed"));
    } finally {
      setBusyId(null);
    }
  }

  return {
    busyId,
    error,
    scheduling,
    setScheduling,
    start: (task: Task) => run(task, () => startTask(task.id)),
    complete: (task: Task) => run(task, () => completeTask(task.id)),
    cancel: (task: Task) => run(task, () => cancelTask(task.id)),
    unschedule: (task: Task) => run(task, () => scheduleTask(task.id, null)),
    saveSchedule: async (date: IsoDate) => {
      if (!scheduling) {
        return;
      }
      await scheduleTask(scheduling.id, date);
      await onChanged();
    },
  };
}
