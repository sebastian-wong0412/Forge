use forge_application::AppError;
use forge_application::repos::CheckInRepository;
use forge_domain::{CheckIn, CheckInId, KeyResultId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteCheckInRepository {
    pool: SqlitePool,
}

impl SqliteCheckInRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CheckInRow {
    id: String,
    key_result_id: String,
    value: f64,
    note: Option<String>,
    checked_on: String,
    created_at: String,
    updated_at: String,
}

impl CheckInRow {
    fn into_entity(self) -> Result<CheckIn, AppError> {
        Ok(CheckIn::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.key_result_id)?,
            self.value,
            self.note,
            convert::date(&self.checked_on)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl CheckInRepository for SqliteCheckInRepository {
    async fn create(&self, check_in: &CheckIn) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO check_ins (id, key_result_id, value, note, checked_on, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(check_in.id().to_string())
        .bind(check_in.key_result_id().to_string())
        .bind(check_in.value())
        .bind(check_in.note())
        .bind(check_in.checked_on().to_string())
        .bind(convert::format_rfc3339(check_in.created_at())?)
        .bind(convert::format_rfc3339(check_in.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: CheckInId) -> Result<Option<CheckIn>, AppError> {
        sqlx::query_as::<_, CheckInRow>(
            "SELECT id, key_result_id, value, note, checked_on, created_at, updated_at
             FROM check_ins WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(CheckInRow::into_entity)
        .transpose()
    }

    async fn list_by_key_result(
        &self,
        key_result_id: KeyResultId,
    ) -> Result<Vec<CheckIn>, AppError> {
        sqlx::query_as::<_, CheckInRow>(
            "SELECT id, key_result_id, value, note, checked_on, created_at, updated_at
             FROM check_ins
             WHERE key_result_id = ?
             ORDER BY checked_on ASC, created_at ASC, id ASC",
        )
        .bind(key_result_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(CheckInRow::into_entity)
        .collect()
    }
}
