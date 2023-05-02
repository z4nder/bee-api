use dotenv::dotenv;
use std::net::SocketAddr;

mod database;
mod dto;
mod errors;
mod extractors;
mod handlers;
mod model;
mod repository;
mod routes;
mod utils;

use database::mysql::db_connect;
use routes::user::user_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = db_connect().await.unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(user_routes(pool).into_make_service())
        .await
        .unwrap();
}
