use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    TagInternalError,
    NotFound,
    WrongCredentials,
    DuplicateUserEmail,
    EncryptError,
    TokenError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, err_msg) = match self {
            Self::TagInternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "tag service internal error",
            ),
            Self::TokenError => (StatusCode::INTERNAL_SERVER_ERROR, "token error"),
            Self::WrongCredentials => (StatusCode::BAD_REQUEST, "wrong credentials"),
            Self::EncryptError => (StatusCode::INTERNAL_SERVER_ERROR, "encrypt error"),
            Self::NotFound => (StatusCode::BAD_REQUEST, "missing credential"),
            Self::DuplicateUserEmail => (StatusCode::BAD_REQUEST, "email has been exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
