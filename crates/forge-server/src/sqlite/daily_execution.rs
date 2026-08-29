use forge_application::AppError;
use forge_application::repos::DailyExecutionRepository;
use forge_domain::{DailyExecution, DailyExecutionId, TaskId};
use sqlx::SqlitePool;
use time::Date;

use super::convert;

#[derive(Clone)]
pub struct SqliteDailyExecutionRepository {
    pool: SqlitePool,
}

impl SqliteDailyExecutionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct DailyExecutionRow {
    id: String,
    task_id: String,
    execution_date: String,
    notes: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl DailyExecutionRow {
    fn into_entity(self) -> Result<DailyExecution, AppError> {
        Ok(DailyExecution::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.task_id)?,
            convert::date(&self.execution_date)?,
            self.notes,
            convert::parse(&self.status)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl DailyExecutionRepository for SqliteDailyExecutionRepository {
    async fn create(&self, execution: &DailyExecution) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO daily_executions
                (id, task_id, execution_date, notes, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(execution.id().to_string())
        .bind(execution.task_id().to_string())
        .bind(execution.execution_date().to_string())
        .bind(execution.notes())
        .bind(execution.status().as_str())
        .bind(convert::format_rfc3339(execution.created_at())?)
        .bind(convert::format_rfc3339(execution.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: DailyExecutionId) -> Result<Option<DailyExecution>, AppError> {
        sqlx::query_as::<_, DailyExecutionRow>(
            "SELECT id, task_id, execution_date, notes, status, created_at, updated_at
             FROM daily_executions WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(DailyExecutionRow::into_entity)
        .transpose()
    }

    async fn list_by_task(&self, task_id: TaskId) -> Result<Vec<DailyExecution>, AppError> {
        sqlx::query_as::<_, DailyExecutionRow>(
            "SELECT id, task_id, execution_date, notes, status, created_at, updated_at
             FROM daily_executions
             WHERE task_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(task_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(DailyExecutionRow::into_entity)
        .collect()
    }

    async fn list_by_date(&self, execution_date: Date) -> Result<Vec<DailyExecution>, AppError> {
        sqlx::query_as::<_, DailyExecutionRow>(
            "SELECT id, task_id, execution_date, notes, status, created_at, updated_at
             FROM daily_executions
             WHERE execution_date = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(execution_date.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(DailyExecutionRow::into_entity)
        .collect()
    }

    async fn update(&self, execution: &DailyExecution) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE daily_executions
             SET notes = ?, status = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(execution.notes())
        .bind(execution.status().as_str())
        .bind(convert::format_rfc3339(execution.updated_at())?)
        .bind(execution.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("daily_execution", execution.id()));
        }
        Ok(())
    }
}
