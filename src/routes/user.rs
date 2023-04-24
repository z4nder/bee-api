use axum::{routing::get, Router};
use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::controller::auth_controller::login;
use crate::model::auth::{Claims, Keys};
use crate::repository::user_repository::UserRepository;

use jsonwebtoken::{decode, Validation};

pub fn user_routes(pool: MySqlPool) -> Router {
    let user_repository = UserRepository {
        db_connection: pool,
    };

    Router::new()
        .route("/login", get(login))
        .route("/me", get(user_profile))
        .layer(Extension(user_repository))
}

#[derive(Deserialize, Debug)]
pub struct PayloadTest {
    token: String,
}

pub async fn user_profile(Json(payload): Json<PayloadTest>) -> Json<Value> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
    let secret = Keys::new(jwt_secret.as_bytes());

    let data =
        decode::<Claims>(&payload.token, &secret.decoding, &Validation::default()).expect("ERROR");

    Json(json!({ "data": data.claims }))
}
