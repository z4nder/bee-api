use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use axum::{routing::get, Router};
use jsonwebtoken::{encode, get_current_timestamp, Header};
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::model;
use crate::model::auth::{Claims, Keys};
use crate::repository::user::UserRepository;
use model::auth::AuthLogin;

pub fn user_routes(pool: MySqlPool) -> Router {
    let user_repository = UserRepository {
        db_connection: pool,
    };

    Router::new()
        .route("/login", get(login))
        .layer(Extension(user_repository))
}

pub async fn login(
    user_repository: Extension<UserRepository>,
    Json(payload): Json<AuthLogin>,
) -> Json<Value> {
    let user = user_repository.find_user_by_email(&payload.email).await;

    if let Ok(user) = user {
        if user.password != payload.password {
            panic!("Invalid password");
        }

        let claims = Claims {
            email: payload.email,
            expire_at: get_current_timestamp(),
        };

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
        let secret = Keys::new(jwt_secret.as_bytes());

        let token = encode(&Header::default(), &claims, &secret.encoding).unwrap();

        Json(json!({ "access_token": token, "type": "Bearer" }))
    } else {
        Json(json!({ "error": "Bearer" }))
    }
}
