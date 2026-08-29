use forge_application::AppError;
use forge_application::repos::ObjectiveRepository;
use forge_domain::{CycleId, Objective, ObjectiveId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteObjectiveRepository {
    pool: SqlitePool,
}

impl SqliteObjectiveRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ObjectiveRow {
    id: String,
    cycle_id: String,
    title: String,
    description: Option<String>,
    status: String,
    start_on: Option<String>,
    end_on: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ObjectiveRow {
    fn into_entity(self) -> Result<Objective, AppError> {
        Ok(Objective::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.cycle_id)?,
            convert::title(&self.title)?,
            self.description,
            convert::parse(&self.status)?,
            convert::optional_date(&self.start_on)?,
            convert::optional_date(&self.end_on)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl ObjectiveRepository for SqliteObjectiveRepository {
    async fn create(&self, objective: &Objective) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO objectives
                (id, cycle_id, title, description, status, start_on, end_on, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(objective.id().to_string())
        .bind(objective.cycle_id().to_string())
        .bind(objective.title().as_str())
        .bind(objective.description())
        .bind(objective.status().as_str())
        .bind(objective.start_on().map(|d| d.to_string()))
        .bind(objective.end_on().map(|d| d.to_string()))
        .bind(convert::format_rfc3339(objective.created_at())?)
        .bind(convert::format_rfc3339(objective.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: ObjectiveId) -> Result<Option<Objective>, AppError> {
        sqlx::query_as::<_, ObjectiveRow>(
            "SELECT id, cycle_id, title, description, status, start_on, end_on, created_at, updated_at
             FROM objectives WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(ObjectiveRow::into_entity)
        .transpose()
    }

    async fn list_by_cycle(&self, cycle_id: CycleId) -> Result<Vec<Objective>, AppError> {
        sqlx::query_as::<_, ObjectiveRow>(
            "SELECT id, cycle_id, title, description, status, start_on, end_on, created_at, updated_at
             FROM objectives
             WHERE cycle_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(cycle_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(ObjectiveRow::into_entity)
        .collect()
    }

    async fn update(&self, objective: &Objective) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE objectives
             SET title = ?, description = ?, status = ?, start_on = ?, end_on = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(objective.title().as_str())
        .bind(objective.description())
        .bind(objective.status().as_str())
        .bind(objective.start_on().map(|d| d.to_string()))
        .bind(objective.end_on().map(|d| d.to_string()))
        .bind(convert::format_rfc3339(objective.updated_at())?)
        .bind(objective.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("objective", objective.id()));
        }
        Ok(())
    }
}
