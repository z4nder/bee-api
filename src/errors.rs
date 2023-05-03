use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    UnexpectedError,
    NotFound,
    InvalidData,
    InvalidToken,
    WrongCredentials,
    DuplicateUserEmail,
    BcryptError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::BcryptError => (StatusCode::INTERNAL_SERVER_ERROR, "unexpected error"),
            Self::UnexpectedError => (StatusCode::INTERNAL_SERVER_ERROR, "unexpected error"),
            Self::WrongCredentials => (StatusCode::BAD_REQUEST, "wrong credentials"),
            Self::InvalidToken => (StatusCode::BAD_REQUEST, "invalid token"),
            Self::NotFound => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::InvalidData => (StatusCode::INTERNAL_SERVER_ERROR, "failed to create user"),
            Self::DuplicateUserEmail => (StatusCode::BAD_REQUEST, "email has been exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
