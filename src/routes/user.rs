use axum::routing::post;
use axum::Extension;
use axum::{routing::get, Router};
use sqlx::MySqlPool;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::handlers::auth_handler::{authorize, login};
use crate::repository::user_repository::UserRepository;

pub fn user_routes(pool: MySqlPool) -> Router {
    let user_repository = UserRepository {
        db_connection: pool,
    };

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(Extension(user_repository))
        .into_inner();

    Router::new()
        .route("/login", post(login))
        .route("/authorize", get(authorize))
        .layer(middleware_stack)
}
