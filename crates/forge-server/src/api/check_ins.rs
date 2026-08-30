use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::CreateCheckIn;
use forge_domain::MilestoneState;
use time::OffsetDateTime;

use super::dto::{CheckInResponse, CreateCheckInRequest};
use super::{ApiError, AppState, parse_id};

fn parse_state(raw: Option<String>) -> Result<Option<MilestoneState>, ApiError> {
    raw.map(|value| {
        value
            .parse()
            .map_err(|err: forge_domain::DomainError| ApiError::bad_request(err.to_string()))
    })
    .transpose()
}

pub async fn create(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(body): Json<CreateCheckInRequest>,
) -> Result<(StatusCode, Json<CheckInResponse>), ApiError> {
    let key_result_id = parse_id(&id, "key_result")?;
    let check_in = state
        .check_ins
        .create(
            key_result_id,
            CreateCheckIn {
                value: body.value,
                state: parse_state(body.state)?,
                note: body.note,
                checked_on: body.checked_on,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(CheckInResponse::from(&check_in))))
}

pub async fn list(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Vec<CheckInResponse>>, ApiError> {
    let key_result_id = parse_id(&id, "key_result")?;
    let check_ins = state.check_ins.list_by_key_result(key_result_id).await?;
    Ok(Json(check_ins.iter().map(CheckInResponse::from).collect()))
}
