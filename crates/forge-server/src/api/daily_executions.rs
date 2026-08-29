use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use forge_application::{CreateDailyExecution, UpdateDailyExecution};
use forge_domain::DailyExecutionStatus;
use time::OffsetDateTime;

use super::dto::{
    CreateDailyExecutionRequest, DailyExecutionResponse, DateQuery, UpdateDailyExecutionRequest,
};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
    Json(body): Json<CreateDailyExecutionRequest>,
) -> Result<(StatusCode, Json<DailyExecutionResponse>), ApiError> {
    let task_id = parse_id(&task_id, "task")?;
    let status: DailyExecutionStatus = body
        .status
        .parse()
        .map_err(forge_application::AppError::from)?;
    let execution = state
        .daily_executions
        .create(
            task_id,
            CreateDailyExecution {
                execution_date: body.execution_date,
                notes: body.notes,
                status,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(DailyExecutionResponse::from(&execution)),
    ))
}

pub async fn list_by_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<Vec<DailyExecutionResponse>>, ApiError> {
    let task_id = parse_id(&task_id, "task")?;
    let executions = state.daily_executions.list_by_task(task_id).await?;
    Ok(Json(
        executions
            .iter()
            .map(DailyExecutionResponse::from)
            .collect(),
    ))
}

pub async fn list_by_date(
    State(state): State<AppState>,
    Query(query): Query<DateQuery>,
) -> Result<Json<Vec<DailyExecutionResponse>>, ApiError> {
    let executions = state.daily_executions.list_by_date(query.date).await?;
    Ok(Json(
        executions
            .iter()
            .map(DailyExecutionResponse::from)
            .collect(),
    ))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DailyExecutionResponse>, ApiError> {
    let id = parse_id(&id, "daily_execution")?;
    let execution = state.daily_executions.get(id).await?;
    Ok(Json(DailyExecutionResponse::from(&execution)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateDailyExecutionRequest>,
) -> Result<Json<DailyExecutionResponse>, ApiError> {
    let id = parse_id(&id, "daily_execution")?;
    let status: DailyExecutionStatus = body
        .status
        .parse()
        .map_err(forge_application::AppError::from)?;
    let execution = state
        .daily_executions
        .update(
            id,
            UpdateDailyExecution {
                notes: body.notes,
                status,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(DailyExecutionResponse::from(&execution)))
}
