use forge_application::AppError;
use forge_application::repos::TaskRepository;
use forge_domain::{ProjectId, Task, TaskId};
use sqlx::SqlitePool;
use time::Date;

use super::convert;

#[derive(Clone)]
pub struct SqliteTaskRepository {
    pool: SqlitePool,
}

impl SqliteTaskRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TaskRow {
    id: String,
    project_id: String,
    title: String,
    description: Option<String>,
    status: String,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TaskRow {
    fn into_entity(self) -> Result<Task, AppError> {
        Ok(Task::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.project_id)?,
            convert::title(&self.title)?,
            self.description,
            convert::parse(&self.status)?,
            None,
            convert::optional_rfc3339(&self.completed_at)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl TaskRepository for SqliteTaskRepository {
    async fn create(&self, task: &Task) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO tasks (id, project_id, title, description, status, completed_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(task.id().to_string())
        .bind(task.project_id().to_string())
        .bind(task.title().as_str())
        .bind(task.description())
        .bind(task.status().as_str())
        .bind(
            task.completed_at()
                .map(convert::format_rfc3339)
                .transpose()?,
        )
        .bind(convert::format_rfc3339(task.created_at())?)
        .bind(convert::format_rfc3339(task.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: TaskId) -> Result<Option<Task>, AppError> {
        sqlx::query_as::<_, TaskRow>(
            "SELECT id, project_id, title, description, status, completed_at, created_at, updated_at
             FROM tasks WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(TaskRow::into_entity)
        .transpose()
    }

    async fn list_by_project(&self, project_id: ProjectId) -> Result<Vec<Task>, AppError> {
        sqlx::query_as::<_, TaskRow>(
            "SELECT id, project_id, title, description, status, completed_at, created_at, updated_at
             FROM tasks
             WHERE project_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(project_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(TaskRow::into_entity)
        .collect()
    }

    async fn update(&self, task: &Task) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE tasks
             SET title = ?, description = ?, status = ?, completed_at = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(task.title().as_str())
        .bind(task.description())
        .bind(task.status().as_str())
        .bind(
            task.completed_at()
                .map(convert::format_rfc3339)
                .transpose()?,
        )
        .bind(convert::format_rfc3339(task.updated_at())?)
        .bind(task.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("task", task.id()));
        }
        Ok(())
    }

    async fn list_today_candidates(&self, _date: Date) -> Result<Vec<Task>, AppError> {
        Ok(Vec::new())
    }
}
