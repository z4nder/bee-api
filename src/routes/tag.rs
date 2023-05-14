use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::handlers::tag_handler::{destroy, find, index, store, update};
use crate::repository::tag_repository::TagRepository;

pub fn tag_routes(pool: &MySqlPool) -> Router {
    let tag_repository = TagRepository {
        db_connection: pool.to_owned(),
    };

    Router::new()
        .route("/tags", get(index))
        .route("/tags/:id", get(find))
        .route("/tags", post(store))
        .route("/tags", put(update))
        .route("/tags:id", delete(destroy))
        .with_state(tag_repository)
}
