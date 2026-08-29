use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::{CreateProject, UpdateProject};
use time::OffsetDateTime;

use super::dto::{CreateProjectRequest, ProjectResponse, UpdateProjectRequest};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Path(objective_id): Path<String>,
    Json(body): Json<CreateProjectRequest>,
) -> Result<(StatusCode, Json<ProjectResponse>), ApiError> {
    let objective_id = parse_id(&objective_id, "objective")?;
    let project = state
        .projects
        .create(
            objective_id,
            CreateProject {
                title: body.title,
                description: body.description,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(ProjectResponse::from(&project))))
}

pub async fn list(
    State(state): State<AppState>,
    Path(objective_id): Path<String>,
) -> Result<Json<Vec<ProjectResponse>>, ApiError> {
    let objective_id = parse_id(&objective_id, "objective")?;
    let projects = state.projects.list_by_objective(objective_id).await?;
    Ok(Json(projects.iter().map(ProjectResponse::from).collect()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let id = parse_id(&id, "project")?;
    let project = state.projects.get(id).await?;
    Ok(Json(ProjectResponse::from(&project)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateProjectRequest>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let id = parse_id(&id, "project")?;
    let project = state
        .projects
        .update(
            id,
            UpdateProject {
                title: body.title,
                description: body.description,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(ProjectResponse::from(&project)))
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let id = parse_id(&id, "project")?;
    let project = state
        .projects
        .activate(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ProjectResponse::from(&project)))
}

pub async fn complete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let id = parse_id(&id, "project")?;
    let project = state
        .projects
        .complete(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ProjectResponse::from(&project)))
}

pub async fn archive(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ProjectResponse>, ApiError> {
    let id = parse_id(&id, "project")?;
    let project = state
        .projects
        .archive(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ProjectResponse::from(&project)))
}
