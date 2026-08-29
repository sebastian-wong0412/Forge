import { request } from "./client";
import type { CreateReviewInput, Review } from "./types";

export function getReviews(cycleId: string): Promise<Review[]> {
  return request(`/api/v1/cycles/${cycleId}/reviews`);
}

export function createReview(cycleId: string, input: CreateReviewInput): Promise<Review> {
  return request(`/api/v1/cycles/${cycleId}/reviews`, {
    method: "POST",
    body: JSON.stringify(input),
  });
}
