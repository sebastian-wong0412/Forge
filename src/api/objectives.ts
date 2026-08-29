import { request } from "./client";
import type { CreateObjectiveInput, Objective } from "./types";

export function getObjectives(cycleId: string): Promise<Objective[]> {
  return request(`/api/v1/cycles/${cycleId}/objectives`);
}

export function getObjective(id: string): Promise<Objective> {
  return request(`/api/v1/objectives/${id}`);
}

export function createObjective(
  cycleId: string,
  input: CreateObjectiveInput,
): Promise<Objective> {
  return request(`/api/v1/cycles/${cycleId}/objectives`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function activateObjective(id: string): Promise<Objective> {
  return request(`/api/v1/objectives/${id}/activate`, { method: "POST" });
}

export function completeObjective(id: string): Promise<Objective> {
  return request(`/api/v1/objectives/${id}/complete`, { method: "POST" });
}

export function archiveObjective(id: string): Promise<Objective> {
  return request(`/api/v1/objectives/${id}/archive`, { method: "POST" });
}
