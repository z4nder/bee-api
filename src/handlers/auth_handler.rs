use std::sync::Arc;

use axum::{Extension, Json};
use axum_macros::debug_handler;
use serde_json::{json, Value};

use crate::{
    dto::auth_dto::{LoginInput, TokenPayload},
    errors::AppError,
    model::user::User,
    repository::user_repository::UserRepository,
    utils::jwt,
};

pub async fn authorize(user: User) -> Json<User> {
    Json(user)
}

#[debug_handler]
pub async fn login(
    Extension(user_repository): Extension<UserRepository>,
    Json(payload): Json<LoginInput>,
) -> Result<Json<TokenPayload>, AppError> {
    let user = user_repository.find_user_by_email(&payload.email).await?;

    if user.password != payload.password {
        panic!("Invalid password");
    }

    let token = jwt::sign(user.id).expect("Error mdfk");

    Ok(Json(TokenPayload {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}
