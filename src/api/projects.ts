import { request } from "./client";
import type { CreateProjectInput, Project } from "./types";

export function getProjects(objectiveId: string): Promise<Project[]> {
  return request(`/api/v1/objectives/${objectiveId}/projects`);
}

export function getProject(id: string): Promise<Project> {
  return request(`/api/v1/projects/${id}`);
}

export function createProject(
  objectiveId: string,
  input: CreateProjectInput,
): Promise<Project> {
  return request(`/api/v1/objectives/${objectiveId}/projects`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function activateProject(id: string): Promise<Project> {
  return request(`/api/v1/projects/${id}/activate`, { method: "POST" });
}

export function completeProject(id: string): Promise<Project> {
  return request(`/api/v1/projects/${id}/complete`, { method: "POST" });
}

export function archiveProject(id: string): Promise<Project> {
  return request(`/api/v1/projects/${id}/archive`, { method: "POST" });
}
