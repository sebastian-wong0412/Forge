import { getObjectives, getProject, getProjects } from "../api";
import type { Cycle, Project, TodayResponse } from "../api/types";
import { openCycleShortcuts } from "./cycles";

export function uniqueProjectIds(today: TodayResponse): string[] {
  return [
    ...new Set(
      [
        ...today.scheduled,
        ...today.overdue,
        ...today.unscheduled_in_progress,
        ...today.completed,
      ].map((task) => task.project_id),
    ),
  ];
}

export async function loadProjectTitles(ids: string[]): Promise<Record<string, string>> {
  const titles: Record<string, string> = {};
  await Promise.all(
    ids.map(async (id) => {
      try {
        const project = await getProject(id);
        titles[id] = project.title;
      } catch {
        // A missing project should not hide the task itself.
      }
    }),
  );
  return titles;
}

export async function loadWorkableProjects(cycles: Cycle[]): Promise<Project[]> {
  const collected: Project[] = [];
  for (const cycle of openCycleShortcuts(cycles)) {
    try {
      const objectives = await getObjectives(cycle.id);
      for (const objective of objectives) {
        try {
          const projects = await getProjects(objective.id);
          for (const project of projects) {
            if (project.status === "draft" || project.status === "active") {
              collected.push(project);
            }
          }
        } catch {
          // Skip an objective whose projects cannot be loaded.
        }
      }
    } catch {
      // Skip a cycle whose objectives cannot be loaded.
    }
  }
  return collected;
}

export function executionDestination(cycles: Cycle[], projects: Project[] = []): string {
  const executable = projects.find(
    (project) => project.status === "draft" || project.status === "active",
  );
  if (executable) {
    return `/projects/${executable.id}`;
  }
  const open = openCycleShortcuts(cycles);
  if (open[0]) {
    return `/cycles/${open[0].id}`;
  }
  return "/cycles";
}
