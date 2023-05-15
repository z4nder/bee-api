use axum::{Extension, Router};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod database;
mod dto;
mod errors;
mod extractors;
mod handlers;
mod model;
mod repository;
mod routes;
mod services;
mod utils;

use database::mysql::db_connect;
use routes::{auth::auth_routes, spend::spend_routes};

use crate::routes::tag::tag_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db_connect().await.unwrap();

    let layers = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(Extension(pool.clone()))
        .into_inner();

    let routes = Router::new()
        .merge(auth_routes(&pool))
        .merge(tag_routes(&pool))
        .merge(spend_routes(&pool))
        .layer(layers);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}
