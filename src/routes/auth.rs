use axum::routing::post;
use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::handlers::auth_handler::{authorize, login, register};
use crate::repository::user_repository::UserRepository;

pub fn auth_routes(pool: &MySqlPool) -> Router {
    let user_repository = UserRepository {
        db_connection: pool.to_owned(),
    };

    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/authorize", get(authorize))
        .with_state(user_repository)
}
