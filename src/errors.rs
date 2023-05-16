use axum::{response::IntoResponse, Json};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    SpendInternalError,
    TagInternalError,
    NotFound,
    WrongCredentials,
    DuplicateUserEmail,
    EncryptError,
    TokenError,
    ResourceNotFound,
}

pub struct ApiError {
    error: AppError,
    message: Option<String>,
}

impl ApiError {
    pub fn new(error: AppError, message: Option<String>) -> ApiError {
        ApiError { error, message }
    }
}
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self.message {
            Some(message) => {
                println!("{:?}", message)
            }
            None => println!("Empty message api error"),
        };
        let (status, err_msg) = match self.error {
            AppError::SpendInternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "spend service internal error",
            ),
            AppError::TagInternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "tag service internal error",
            ),
            AppError::TokenError => (StatusCode::INTERNAL_SERVER_ERROR, "token error"),
            AppError::WrongCredentials => (StatusCode::BAD_REQUEST, "wrong credentials"),
            AppError::EncryptError => (StatusCode::INTERNAL_SERVER_ERROR, "encrypt error"),
            AppError::NotFound => (StatusCode::BAD_REQUEST, "missing credential"),
            AppError::ResourceNotFound => (StatusCode::NOT_FOUND, "resource not found"),
            AppError::DuplicateUserEmail => (StatusCode::BAD_REQUEST, "email has been exists"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
