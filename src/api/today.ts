import { request, toQuery } from "./client";
import type { IsoDate, TodayResponse } from "./types";

export function getToday(date: IsoDate): Promise<TodayResponse> {
  return request(`/api/v1/today?${toQuery({ date })}`);
}
