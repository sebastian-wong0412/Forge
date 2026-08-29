use forge_application::AppError;
use forge_application::repos::ProjectRepository;
use forge_domain::{ObjectiveId, Project, ProjectId};
use sqlx::SqlitePool;

use super::convert;

#[derive(Clone)]
pub struct SqliteProjectRepository {
    pool: SqlitePool,
}

impl SqliteProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct ProjectRow {
    id: String,
    objective_id: String,
    title: String,
    description: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl ProjectRow {
    fn into_entity(self) -> Result<Project, AppError> {
        Ok(Project::reconstitute(
            convert::parse(&self.id)?,
            convert::parse(&self.objective_id)?,
            convert::title(&self.title)?,
            self.description,
            convert::parse(&self.status)?,
            convert::rfc3339(&self.created_at)?,
            convert::rfc3339(&self.updated_at)?,
        ))
    }
}

impl ProjectRepository for SqliteProjectRepository {
    async fn create(&self, project: &Project) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO projects (id, objective_id, title, description, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(project.id().to_string())
        .bind(project.objective_id().to_string())
        .bind(project.title().as_str())
        .bind(project.description())
        .bind(project.status().as_str())
        .bind(convert::format_rfc3339(project.created_at())?)
        .bind(convert::format_rfc3339(project.updated_at())?)
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        Ok(())
    }

    async fn get(&self, id: ProjectId) -> Result<Option<Project>, AppError> {
        sqlx::query_as::<_, ProjectRow>(
            "SELECT id, objective_id, title, description, status, created_at, updated_at
             FROM projects WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .map(ProjectRow::into_entity)
        .transpose()
    }

    async fn list_by_objective(&self, objective_id: ObjectiveId) -> Result<Vec<Project>, AppError> {
        sqlx::query_as::<_, ProjectRow>(
            "SELECT id, objective_id, title, description, status, created_at, updated_at
             FROM projects
             WHERE objective_id = ?
             ORDER BY created_at ASC, id ASC",
        )
        .bind(objective_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(convert::map_sqlx)?
        .into_iter()
        .map(ProjectRow::into_entity)
        .collect()
    }

    async fn update(&self, project: &Project) -> Result<(), AppError> {
        let result = sqlx::query(
            "UPDATE projects
             SET title = ?, description = ?, status = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(project.title().as_str())
        .bind(project.description())
        .bind(project.status().as_str())
        .bind(convert::format_rfc3339(project.updated_at())?)
        .bind(project.id().to_string())
        .execute(&self.pool)
        .await
        .map_err(convert::map_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(AppError::not_found("project", project.id()));
        }
        Ok(())
    }
}
