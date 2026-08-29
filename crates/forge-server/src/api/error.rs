use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use forge_application::AppError;
use serde::Serialize;

#[derive(Debug)]
pub enum ApiError {
    App(AppError),
    BadRequest(String),
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }
}

impl From<AppError> for ApiError {
    fn from(value: AppError) -> Self {
        Self::App(value)
    }
}

#[derive(Serialize)]
struct ErrorEnvelope {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: &'static str,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "bad_request", message),
            Self::App(AppError::Domain(err)) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "domain", err.to_string())
            }
            Self::App(AppError::NotFound { entity, id }) => (
                StatusCode::NOT_FOUND,
                "not_found",
                format!("{entity} `{id}` was not found"),
            ),
            Self::App(AppError::Conflict { message }) => {
                (StatusCode::CONFLICT, "conflict", message)
            }
            Self::App(AppError::Persistence { message }) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "persistence", message)
            }
        };

        (
            status,
            Json(ErrorEnvelope {
                error: ErrorBody { code, message },
            }),
        )
            .into_response()
    }
}
