use axum::routing::post;
use axum::{routing::get, Router};

use crate::handlers::auth_handler::{authorize, login, register};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/authorize", get(authorize))
}
