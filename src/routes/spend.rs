use axum::routing::{delete, post, put};
use axum::{routing::get, Router};
use sqlx::MySqlPool;

use crate::handlers::spend_handler::{destroy, find, index, store, update};
use crate::repository::spend_repository::SpendRepository;

pub fn spend_routes(pool: &MySqlPool) -> Router {
    let tag_repository = SpendRepository {
        db_connection: pool.to_owned(),
    };

    Router::new()
        .route("/spends", get(index))
        .route("/spends/:id", get(find))
        .route("/spends", post(store))
        .route("/spends/:id", put(update))
        .route("/spends/:id", delete(destroy))
        .with_state(tag_repository)
}
