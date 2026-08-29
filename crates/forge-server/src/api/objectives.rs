use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::{CreateObjective, UpdateObjective};
use time::OffsetDateTime;

use super::dto::{CreateObjectiveRequest, ObjectiveResponse, UpdateObjectiveRequest};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Path(cycle_id): Path<String>,
    Json(body): Json<CreateObjectiveRequest>,
) -> Result<(StatusCode, Json<ObjectiveResponse>), ApiError> {
    let cycle_id = parse_id(&cycle_id, "cycle")?;
    let objective = state
        .objectives
        .create(
            cycle_id,
            CreateObjective {
                title: body.title,
                description: body.description,
                start_on: body.start_on,
                end_on: body.end_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(ObjectiveResponse::from(&objective)),
    ))
}

pub async fn list(
    State(state): State<AppState>,
    Path(cycle_id): Path<String>,
) -> Result<Json<Vec<ObjectiveResponse>>, ApiError> {
    let cycle_id = parse_id(&cycle_id, "cycle")?;
    let objectives = state.objectives.list_by_cycle(cycle_id).await?;
    Ok(Json(
        objectives.iter().map(ObjectiveResponse::from).collect(),
    ))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ObjectiveResponse>, ApiError> {
    let id = parse_id(&id, "objective")?;
    let objective = state.objectives.get(id).await?;
    Ok(Json(ObjectiveResponse::from(&objective)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateObjectiveRequest>,
) -> Result<Json<ObjectiveResponse>, ApiError> {
    let id = parse_id(&id, "objective")?;
    let objective = state
        .objectives
        .update(
            id,
            UpdateObjective {
                title: body.title,
                description: body.description,
                start_on: body.start_on,
                end_on: body.end_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(ObjectiveResponse::from(&objective)))
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ObjectiveResponse>, ApiError> {
    let id = parse_id(&id, "objective")?;
    let objective = state
        .objectives
        .activate(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ObjectiveResponse::from(&objective)))
}

pub async fn complete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ObjectiveResponse>, ApiError> {
    let id = parse_id(&id, "objective")?;
    let objective = state
        .objectives
        .complete(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ObjectiveResponse::from(&objective)))
}

pub async fn archive(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ObjectiveResponse>, ApiError> {
    let id = parse_id(&id, "objective")?;
    let objective = state
        .objectives
        .archive(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(ObjectiveResponse::from(&objective)))
}
