use forge_domain::{CycleId, Review};
use time::{Date, OffsetDateTime};

use crate::AppError;
use crate::repos::{CycleRepository, ReviewRepository};

pub struct CreateReview {
    pub content: String,
    pub period_start: Option<Date>,
    pub period_end: Option<Date>,
}

#[derive(Clone)]
pub struct ReviewService<C, R> {
    cycles: C,
    reviews: R,
}

impl<C, R> ReviewService<C, R>
where
    C: CycleRepository,
    R: ReviewRepository,
{
    pub fn new(cycles: C, reviews: R) -> Self {
        Self { cycles, reviews }
    }

    pub async fn create(
        &self,
        cycle_id: CycleId,
        cmd: CreateReview,
        now: OffsetDateTime,
    ) -> Result<Review, AppError> {
        let cycle = self
            .cycles
            .get(cycle_id)
            .await?
            .ok_or_else(|| AppError::not_found("cycle", cycle_id))?;
        let review = Review::create(
            cycle_id,
            cmd.content,
            cmd.period_start,
            cmd.period_end,
            cycle.start_on(),
            cycle.end_on(),
            now,
        )?;
        self.reviews.create(&review).await?;
        Ok(review)
    }

    pub async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Review>, AppError> {
        self.cycles
            .get(cycle_id)
            .await?
            .ok_or_else(|| AppError::not_found("cycle", cycle_id))?;
        self.reviews.list_by_cycle(cycle_id).await
    }
}

#[cfg(test)]
mod tests {
    use time::macros::{date, datetime};

    use super::*;
    use crate::services::cycle::{CreateCycle, CycleService};
    use crate::test_support::{InMemoryCycleRepo, InMemoryReviewRepo};

    const NOW: OffsetDateTime = datetime!(2026-01-15 12:00:00 UTC);

    #[tokio::test]
    async fn review_allowed_after_cycle_close() {
        let cycles = InMemoryCycleRepo::default();
        let cycle_svc = CycleService::new(cycles.clone());
        let reviews = ReviewService::new(cycles, InMemoryReviewRepo::default());
        let cycle = cycle_svc
            .create(
                CreateCycle {
                    name: "Q1".into(),
                    start_on: date!(2026 - 01 - 01),
                    end_on: date!(2026 - 03 - 31),
                },
                NOW,
            )
            .await
            .unwrap();
        cycle_svc.activate(cycle.id(), NOW).await.unwrap();
        cycle_svc.close(cycle.id(), NOW).await.unwrap();
        let review = reviews
            .create(
                cycle.id(),
                CreateReview {
                    content: "What worked".into(),
                    period_start: Some(date!(2026 - 01 - 01)),
                    period_end: Some(date!(2026 - 03 - 31)),
                },
                NOW,
            )
            .await
            .unwrap();
        assert_eq!(review.cycle_id(), cycle.id());
        assert_eq!(reviews.list_by_cycle(cycle.id()).await.unwrap().len(), 1);
    }
}
