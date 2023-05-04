use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::handlers::tag_handler::{index, store};
use crate::repository::tag_repository::TagRepository;

pub fn tag_routes(pool: &MySqlPool) -> Router {
    let tag_repository = TagRepository {
        db_connection: pool.to_owned(),
    };

    Router::new()
        .route("/tags", get(index))
        // .route("/tags:id", get(register))
        .route("/tags", post(store))
        // .route("/tags", put(authorize))
        // .route("/tags:id", delete(authorize))
        .with_state(tag_repository)
}
