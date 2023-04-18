use axum::{routing::get, Router};

pub fn user_routes() -> Router {
    Router::new().route("/todos", get(test))
}

async fn test() -> &'static str {
    "Hello, World!"
}
