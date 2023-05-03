use axum::{Extension, Json};
use axum_macros::debug_handler;

use crate::{
    dto::auth_dto::{LoginInput, RegisterInput, TokenPayload},
    errors::AppError,
    model::user::{User, UserProfile},
    repository::user_repository::UserRepository,
    services::auth_service::AuthService,
    utils::jwt,
};

pub async fn authorize(user: User) -> Json<UserProfile> {
    Json(UserProfile {
        id: user.id,
        name: user.name,
        email: user.email,
    })
}

#[debug_handler]
pub async fn login(
    Extension(user_repository): Extension<UserRepository>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<TokenPayload>, AppError> {
    let user = AuthService::login(payload, user_repository)
        .await
        .map_err(|_| AppError::WrongCredentials)?;

    let token = jwt::sign(user.id)?;

    Ok(Json(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

pub async fn register(
    Extension(user_repository): Extension<UserRepository>,
    Json(payload): Json<RegisterInput>,
) -> Result<Json<TokenPayload>, AppError> {
    let id = AuthService::register(payload, user_repository).await?;

    let token = jwt::sign(id)?;

    Ok(Json(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}
