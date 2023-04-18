use axum::{extract, http::StatusCode, response::IntoResponse, Extension, Json};
use axum::{routing::get, Router};

use crate::model;
use model::auth::AuthLogin;

pub fn user_routes() -> Router {
    Router::new().route("/login", get(login))
}

pub async fn login(Json(payload): Json<AuthLogin>) -> impl IntoResponse {
    println!("BODY {:?}", payload);

    let created_id: u64 = 12;

    (StatusCode::CREATED, Json(created_id))
}
