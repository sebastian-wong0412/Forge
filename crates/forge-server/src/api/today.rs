use axum::Json;
use axum::extract::{Query, State};

use super::dto::{DateQuery, TodayResponse};
use super::{ApiError, AppState};

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<DateQuery>,
) -> Result<Json<TodayResponse>, ApiError> {
    let today = state.tasks.today(query.date).await?;
    Ok(Json(TodayResponse::from(&today)))
}
