use forge_application::AppError;
use forge_application::repos::CycleRepository;
use forge_domain::{Cycle, CycleId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteCycleRepository {
    pool: SqlitePool,
}

impl SqliteCycleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CycleRow {
    id: String,
    name: String,
    start_on: String,
    end_on: String,
    status: String,
    created_at: String,
    updated_at: String,
}

impl CycleRow {
    fn into_entity(self) -> Result<Cycle, AppError> {
        Ok(Cycle::reconstitute(
            convert::parse(&self.id)?,
            convert::title(&self.name)?,
            convert::date(&self.start_on)?,
            convert::date(&self.end_on)?,
            convert::parse(&self.status)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl CycleRepository for SqliteCycleRepository {
    async fn create(&self, cycle: &Cycle) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO cycles (id, name, start_on, end_on, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(cycle.id().to_string())
        .bind(cycle.name().as_str())
        .bind(cycle.start_on().to_string())
        .bind(cycle.end_on().to_string())
        .bind(cycle.status().as_str())
        .bind(convert::format_rfc3339(cycle.created_at())?)
        .bind(convert::format_rfc3339(cycle.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: CycleId) -> Result<Option<Cycle>, AppError> {
        sqlx::query_as::<_, CycleRow>(
            "SELECT id, name, start_on, end_on, status, created_at, updated_at
             FROM cycles WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(CycleRow::into_entity)
        .transpose()
    }

    async fn list(&self) -> Result<Vec<Cycle>, AppError> {
        sqlx::query_as::<_, CycleRow>(
            "SELECT id, name, start_on, end_on, status, created_at, updated_at
             FROM cycles
             ORDER BY created_at ASC, id ASC",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(CycleRow::into_entity)
        .collect()
    }

    async fn update(&self, cycle: &Cycle) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE cycles
             SET name = ?, start_on = ?, end_on = ?, status = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(cycle.name().as_str())
        .bind(cycle.start_on().to_string())
        .bind(cycle.end_on().to_string())
        .bind(cycle.status().as_str())
        .bind(convert::format_rfc3339(cycle.updated_at())?)
        .bind(cycle.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("cycle", cycle.id()));
        }
        Ok(())
    }
}
