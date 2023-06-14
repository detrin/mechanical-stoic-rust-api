use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    ResilienceOutOfRange,
    AcceptanceOutOfRange,
    WisdomOutOfRange,
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "an internal server error occured",
            ),
            Self::ResilienceOutOfRange => (StatusCode::BAD_REQUEST, "resilience out of range"),
            Self::AcceptanceOutOfRange => (StatusCode::BAD_REQUEST, "acceptance out of range"),
            Self::WisdomOutOfRange => (StatusCode::BAD_REQUEST, "wisdom out of range"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}