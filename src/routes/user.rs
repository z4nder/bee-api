use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use axum::{routing::get, Router};
use jsonwebtoken::{encode, get_current_timestamp, Header};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::MySqlPool;

use crate::model;
use crate::model::auth::{Claims, Keys};
use crate::repository::user::UserRepository;
use model::auth::AuthLogin;

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
            exp: get_current_timestamp(),
        };

        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT SECRET ENV NOT SET");
        let secret = Keys::new(jwt_secret.as_bytes());

        let token = encode(&Header::default(), &claims, &secret.encoding).unwrap();

        Json(json!({ "access_token": token, "type": "Bearer" }))
    } else {
        Json(json!({ "error": "Bearer" }))
    }
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
