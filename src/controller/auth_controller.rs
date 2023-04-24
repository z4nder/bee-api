use axum::{Extension, Json};
use jsonwebtoken::{encode, get_current_timestamp, Header};
use serde_json::{json, Value};

use crate::{
    errors::AppError,
    model::auth::{AuthLogin, Claims, Keys},
    repository::user_repository::UserRepository,
};

pub async fn login(
    user_repository: Extension<UserRepository>,
    Json(payload): Json<AuthLogin>,
) -> Result<Json<Value>, AppError> {
    let user = user_repository.find_user_by_email(&payload.email).await?;

    if user.password != payload.password {
        panic!("Invalid password");
    }

    let claims = Claims {
        email: payload.email,
        exp: get_current_timestamp(),
    };

    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
    let secret = Keys::new(jwt_secret.as_bytes());

    let token = encode(&Header::default(), &claims, &secret.encoding).unwrap();

    Ok(Json(json!({ "access_token": token, "type": "Bearer" })))
}
