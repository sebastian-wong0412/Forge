use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use forge_application::CreateReview;
use time::OffsetDateTime;

use super::dto::{CreateReviewRequest, ReviewResponse};
use super::{ApiError, AppState, parse_id};

pub async fn create(
    State(state): State<AppState>,
    Path(cycle_id): Path<String>,
    Json(body): Json<CreateReviewRequest>,
) -> Result<(StatusCode, Json<ReviewResponse>), ApiError> {
    let cycle_id = parse_id(&cycle_id, "cycle")?;
    let review = state
        .reviews
        .create(
            cycle_id,
            CreateReview {
                content: body.content,
                period_start: body.period_start,
                period_end: body.period_end,
            },
            OffsetDateTime::now_utc(),
        )
        .await?;
    Ok((StatusCode::CREATED, Json(ReviewResponse::from(&review))))
}

pub async fn list(
    State(state): State<AppState>,
    Path(cycle_id): Path<String>,
) -> Result<Json<Vec<ReviewResponse>>, ApiError> {
    let cycle_id = parse_id(&cycle_id, "cycle")?;
    let reviews = state.reviews.list_by_cycle(cycle_id).await?;
    Ok(Json(reviews.iter().map(ReviewResponse::from).collect()))
}
