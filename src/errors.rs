use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde_json::json;

#[derive(Debug)]
pub enum RepositoryError {
    #[allow(dead_code)]
    NotFound,
    #[allow(dead_code)]
    InvalidData,
}

pub enum AppError {
    UserRepository(RepositoryError),
}

impl From<RepositoryError> for AppError {
    fn from(inner: RepositoryError) -> Self {
        AppError::UserRepository(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::UserRepository(RepositoryError::NotFound) => {
                (StatusCode::NOT_FOUND, "User not found")
            }
            AppError::UserRepository(RepositoryError::InvalidData) => {
                (StatusCode::UNPROCESSABLE_ENTITY, "Invalid data")
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
