use forge_application::AppError;
use forge_application::repos::ReviewRepository;
use forge_domain::{CycleId, Review, ReviewId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteReviewRepository {
    pool: SqlitePool,
}

impl SqliteReviewRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ReviewRow {
    id: String,
    cycle_id: String,
    content: String,
    period_start: Option<String>,
    period_end: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ReviewRow {
    fn into_entity(self) -> Result<Review, AppError> {
        Ok(Review::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.cycle_id)?,
            self.content,
            convert::optional_date(&self.period_start)?,
            convert::optional_date(&self.period_end)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl ReviewRepository for SqliteReviewRepository {
    async fn create(&self, review: &Review) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO reviews (id, cycle_id, content, period_start, period_end, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(review.id().to_string())
        .bind(review.cycle_id().to_string())
        .bind(review.content())
        .bind(review.period_start().map(|d| d.to_string()))
        .bind(review.period_end().map(|d| d.to_string()))
        .bind(convert::format_rfc3339(review.created_at())?)
        .bind(convert::format_rfc3339(review.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: ReviewId) -> Result<Option<Review>, AppError> {
        sqlx::query_as::<_, ReviewRow>(
            "SELECT id, cycle_id, content, period_start, period_end, created_at, updated_at
             FROM reviews WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(ReviewRow::into_entity)
        .transpose()
    }

    async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Review>, AppError> {
        sqlx::query_as::<_, ReviewRow>(
            "SELECT id, cycle_id, content, period_start, period_end, created_at, updated_at
             FROM reviews
             WHERE cycle_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(cycle_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(ReviewRow::into_entity)
        .collect()
    }
}
