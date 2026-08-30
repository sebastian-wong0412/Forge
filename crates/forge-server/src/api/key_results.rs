use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::{CreateKeyResult, UpdateKeyResult};
use forge_domain::ProgressKind;
use time::OffsetDateTime;

use super::dto::{CreateKeyResultRequest, KeyResultResponse, UpdateKeyResultRequest};
use super::{ApiError, AppState, parse_id};

fn parse_progress_kind(raw: &str) -> Result<ProgressKind, ApiError> {
    raw.parse()
        .map_err(|err: forge_domain::DomainError| ApiError::bad_request(err.to_string()))
}

pub async fn create(
    State(state): State<AppState>,
    Path(objective_id): Path<String>,
    Json(body): Json<CreateKeyResultRequest>,
) -> Result<(StatusCode, Json<KeyResultResponse>), ApiError> {
    let objective_id = parse_id(&objective_id, "objective")?;
    let key_result = state
        .key_results
        .create(
            objective_id,
            CreateKeyResult {
                title: body.title,
                description: body.description,
                progress_kind: parse_progress_kind(&body.progress_kind)?,
                start_value: body.start_value,
                target_value: body.target_value,
                unit: body.unit,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((
        StatusCode::CREATED,
        Json(KeyResultResponse::from(&key_result)),
    ))
}

pub async fn list(
    State(state): State<AppState>,
    Path(objective_id): Path<String>,
) -> Result<Json<Vec<KeyResultResponse>>, ApiError> {
    let objective_id = parse_id(&objective_id, "objective")?;
    let key_results = state.key_results.list_by_objective(objective_id).await?;
    Ok(Json(
        key_results.iter().map(KeyResultResponse::from).collect(),
    ))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KeyResultResponse>, ApiError> {
    let id = parse_id(&id, "key_result")?;
    let key_result = state.key_results.get(id).await?;
    Ok(Json(KeyResultResponse::from(&key_result)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<UpdateKeyResultRequest>,
) -> Result<Json<KeyResultResponse>, ApiError> {
    let id = parse_id(&id, "key_result")?;
    let key_result = state
        .key_results
        .update(
            id,
            UpdateKeyResult {
                title: body.title,
                description: body.description,
                start_value: body.start_value,
                target_value: body.target_value,
                unit: body.unit,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok(Json(KeyResultResponse::from(&key_result)))
}

pub async fn activate(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KeyResultResponse>, ApiError> {
    let id = parse_id(&id, "key_result")?;
    let key_result = state
        .key_results
        .activate(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(KeyResultResponse::from(&key_result)))
}

pub async fn complete(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KeyResultResponse>, ApiError> {
    let id = parse_id(&id, "key_result")?;
    let key_result = state
        .key_results
        .complete(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(KeyResultResponse::from(&key_result)))
}

pub async fn archive(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<KeyResultResponse>, ApiError> {
    let id = parse_id(&id, "key_result")?;
    let key_result = state
        .key_results
        .archive(id, OffsetDateTime::now_utc())
        .await?;
    Ok(Json(KeyResultResponse::from(&key_result)))
}
