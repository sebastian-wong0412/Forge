use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::{CreateTask, UpdateTask};
use time::OffsetDateTime;

use super::dto::{CreateTaskRequest, ScheduleTaskRequest, TaskResponse, UpdateTaskRequest};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
    Json(body): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>), ApiError> {
    let project_id = parse_id(&project_id, "project")?;
    let task = state
        .tasks
        .create(
            project_id,
            CreateTask {
                title: body.title,
                description: body.description,
                scheduled_on: body.scheduled_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(TaskResponse::from(&task))))
}

pub async fn list(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<Vec<TaskResponse>>, ApiError> {
    let project_id = parse_id(&project_id, "project")?;
    let tasks = state.tasks.list_by_project(project_id).await?;
    Ok(Json(tasks.iter().map(TaskResponse::from).collect()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state.tasks.get(id).await?;
    Ok(Json(TaskResponse::from(&task)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state
        .tasks
        .update(
            id,
            UpdateTask {
                title: body.title,
                description: body.description,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(TaskResponse::from(&task)))
}

pub async fn start(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state.tasks.start(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(TaskResponse::from(&task)))
}

pub async fn complete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state.tasks.complete(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(TaskResponse::from(&task)))
}

pub async fn cancel(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state.tasks.cancel(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(TaskResponse::from(&task)))
}

pub async fn schedule(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<ScheduleTaskRequest>,
) -> Result<Json<TaskResponse>, ApiError> {
    let id = parse_id(&id, "task")?;
    let task = state
        .tasks
        .schedule(id, body.scheduled_on, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(TaskResponse::from(&task)))
}
