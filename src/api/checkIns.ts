import { request } from "./client";
import type { CheckIn, CreateCheckInInput } from "./types";

export function getCheckIns(keyResultId: string): Promise<CheckIn[]> {
  return request(`/api/v1/key-results/${keyResultId}/check-ins`);
}

export function createCheckIn(
  keyResultId: string,
  input: CreateCheckInInput,
): Promise<CheckIn> {
  return request(`/api/v1/key-results/${keyResultId}/check-ins`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}
