import type { Task } from "../api/types";
import { EmptyState } from "./EmptyState";
import { TaskRow } from "./TaskRow";

export function TaskList({
  title,
  tasks,
  empty,
  projectTitle,
  projectTitles,
  busyId,
  onStart,
  onComplete,
  onCancel,
  onSchedule,
  onUnschedule,
}: {
  title?: string;
  tasks: Task[];
  empty: string;
  projectTitle?: string;
  projectTitles?: Record<string, string>;
  busyId?: string | null;
  onStart: (task: Task) => void;
  onComplete: (task: Task) => void;
  onCancel: (task: Task) => void;
  onSchedule: (task: Task) => void;
  onUnschedule: (task: Task) => void;
}) {
  return (
    <section>
      {title ? <h2 className="section-title">{title}</h2> : null}
      {tasks.length === 0 ? (
        <EmptyState title={empty} />
      ) : (
        <div className="stack">
          {tasks.map((task) => (
            <TaskRow
              key={task.id}
              task={task}
              projectTitle={projectTitle ?? projectTitles?.[task.project_id]}
              busy={busyId === task.id}
              onStart={() => onStart(task)}
              onComplete={() => onComplete(task)}
              onCancel={() => onCancel(task)}
              onSchedule={() => onSchedule(task)}
              onUnschedule={() => onUnschedule(task)}
            />
          ))}
        </div>
      )}
    </section>
  );
}
