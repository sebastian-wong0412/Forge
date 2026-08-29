import { request } from "./client";
import type { CreateCycleInput, Cycle } from "./types";

export function getCycles(): Promise<Cycle[]> {
  return request("/api/v1/cycles");
}

export function getCycle(id: string): Promise<Cycle> {
  return request(`/api/v1/cycles/${id}`);
}

export function createCycle(input: CreateCycleInput): Promise<Cycle> {
  return request("/api/v1/cycles", {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function activateCycle(id: string): Promise<Cycle> {
  return request(`/api/v1/cycles/${id}/activate`, { method: "POST" });
}

export function closeCycle(id: string): Promise<Cycle> {
  return request(`/api/v1/cycles/${id}/close`, { method: "POST" });
}

export function archiveCycle(id: string): Promise<Cycle> {
  return request(`/api/v1/cycles/${id}/archive`, { method: "POST" });
}
