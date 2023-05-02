use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    InvalidData,
    InvalidToken,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::NotFound => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::InvalidData => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create token"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
