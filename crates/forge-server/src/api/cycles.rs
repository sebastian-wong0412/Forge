use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::{CreateCycle, UpdateCycle};
use time::OffsetDateTime;

use super::dto::{CreateCycleRequest, CycleResponse, UpdateCycleRequest};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<CreateCycleRequest>,
) -> Result<(StatusCode, Json<CycleResponse>), ApiError> {
    let cycle = state
        .cycles
        .create(
            CreateCycle {
                name: body.name,
                start_on: body.start_on,
                end_on: body.end_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(CycleResponse::from(&cycle))))
}

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<CycleResponse>>, ApiError> {
    let cycles = state.cycles.list().await?;
    Ok(Json(cycles.iter().map(CycleResponse::from).collect()))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CycleResponse>, ApiError> {
    let id = parse_id(&id, "cycle")?;
    let cycle = state.cycles.get(id).await?;
    Ok(Json(CycleResponse::from(&cycle)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateCycleRequest>,
) -> Result<Json<CycleResponse>, ApiError> {
    let id = parse_id(&id, "cycle")?;
    let cycle = state
        .cycles
        .update(
            id,
            UpdateCycle {
                name: body.name,
                start_on: body.start_on,
                end_on: body.end_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(CycleResponse::from(&cycle)))
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CycleResponse>, ApiError> {
    let id = parse_id(&id, "cycle")?;
    let cycle = state.cycles.activate(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(CycleResponse::from(&cycle)))
}

pub async fn close(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CycleResponse>, ApiError> {
    let id = parse_id(&id, "cycle")?;
    let cycle = state.cycles.close(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(CycleResponse::from(&cycle)))
}

pub async fn archive(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<CycleResponse>, ApiError> {
    let id = parse_id(&id, "cycle")?;
    let cycle = state.cycles.archive(id, OffsetDateTime::now_utc()).await?;
    Ok(Json(CycleResponse::from(&cycle)))
}
