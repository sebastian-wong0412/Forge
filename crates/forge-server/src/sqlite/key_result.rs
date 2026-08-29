use forge_application::AppError;
use forge_application::repos::KeyResultRepository;
use forge_domain::{KeyResult, KeyResultId, ObjectiveId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteKeyResultRepository {
    pool: SqlitePool,
}

impl SqliteKeyResultRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct KeyResultRow {
    id: String,
    objective_id: String,
    title: String,
    description: Option<String>,
    status: String,
    start_value: f64,
    target_value: Option<f64>,
    unit: Option<String>,
    created_at: String,
    updated_at: String,
}

impl KeyResultRow {
    fn into_entity(self) -> Result<KeyResult, AppError> {
        Ok(KeyResult::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.objective_id)?,
            convert::title(&self.title)?,
            self.description,
            convert::parse(&self.status)?,
            self.start_value,
            self.target_value,
            self.unit,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl KeyResultRepository for SqliteKeyResultRepository {
    async fn create(&self, key_result: &KeyResult) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO key_results
                (id, objective_id, title, description, status, start_value, target_value, unit, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(key_result.id().to_string())
        .bind(key_result.objective_id().to_string())
        .bind(key_result.title().as_str())
        .bind(key_result.description())
        .bind(key_result.status().as_str())
        .bind(key_result.start_value())
        .bind(key_result.target_value())
        .bind(key_result.unit())
        .bind(convert::format_rfc3339(key_result.created_at())?)
        .bind(convert::format_rfc3339(key_result.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: KeyResultId) -> Result<Option<KeyResult>, AppError> {
        sqlx::query_as::<_, KeyResultRow>(
            "SELECT id, objective_id, title, description, status, start_value, target_value, unit, created_at, updated_at
             FROM key_results WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(KeyResultRow::into_entity)
        .transpose()
    }

    async fn list_by_objective(
        &self,
        objective_id: ObjectiveId,
    ) -> Result<Vec<KeyResult>, AppError> {
        sqlx::query_as::<_, KeyResultRow>(
            "SELECT id, objective_id, title, description, status, start_value, target_value, unit, created_at, updated_at
             FROM key_results
             WHERE objective_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(objective_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(KeyResultRow::into_entity)
        .collect()
    }

    async fn update(&self, key_result: &KeyResult) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE key_results
             SET title = ?, description = ?, status = ?, start_value = ?, target_value = ?, unit = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(key_result.title().as_str())
        .bind(key_result.description())
        .bind(key_result.status().as_str())
        .bind(key_result.start_value())
        .bind(key_result.target_value())
        .bind(key_result.unit())
        .bind(convert::format_rfc3339(key_result.updated_at())?)
        .bind(key_result.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("key_result", key_result.id()));
        }
        Ok(())
    }
}
