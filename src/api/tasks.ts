import { request } from "./client";
import type { CreateTaskInput, IsoDate, Task } from "./types";

export function getTasks(projectId: string): Promise<Task[]> {
  return request(`/api/v1/projects/${projectId}/tasks`);
}

export function getTask(id: string): Promise<Task> {
  return request(`/api/v1/tasks/${id}`);
}

export function createTask(projectId: string, input: CreateTaskInput): Promise<Task> {
  return request(`/api/v1/projects/${projectId}/tasks`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function startTask(id: string): Promise<Task> {
  return request(`/api/v1/tasks/${id}/start`, { method: "POST" });
}

export function completeTask(id: string): Promise<Task> {
  return request(`/api/v1/tasks/${id}/complete`, { method: "POST" });
}

export function cancelTask(id: string): Promise<Task> {
  return request(`/api/v1/tasks/${id}/cancel`, { method: "POST" });
}

export function scheduleTask(id: string, scheduledOn: IsoDate | null): Promise<Task> {
  return request(`/api/v1/tasks/${id}/schedule`, {
    method: "POST",
    body: JSON.stringify({ scheduled_on: scheduledOn }),
  });
}
