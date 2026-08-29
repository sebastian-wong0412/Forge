import { request } from "./client";
import type { CreateKeyResultInput, KeyResult } from "./types";

export function getKeyResults(objectiveId: string): Promise<KeyResult[]> {
  return request(`/api/v1/objectives/${objectiveId}/key-results`);
}

export function getKeyResult(id: string): Promise<KeyResult> {
  return request(`/api/v1/key-results/${id}`);
}

export function createKeyResult(
  objectiveId: string,
  input: CreateKeyResultInput,
): Promise<KeyResult> {
  return request(`/api/v1/objectives/${objectiveId}/key-results`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}

export function activateKeyResult(id: string): Promise<KeyResult> {
  return request(`/api/v1/key-results/${id}/activate`, { method: "POST" });
}

export function completeKeyResult(id: string): Promise<KeyResult> {
  return request(`/api/v1/key-results/${id}/complete`, { method: "POST" });
}
