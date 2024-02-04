use axum::{http::StatusCode, routing, Router};

pub fn create_router() -> Router {
    Router::new().route("/", routing::get(|| async { StatusCode::OK }))
}
