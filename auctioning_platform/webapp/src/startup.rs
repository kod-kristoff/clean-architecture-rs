use crate::blueprints::{auctions_blueprint, health_blueprint};

use axum::Router;

pub fn create_app() -> Router {
    Router::new()
        .nest("/auctions", auctions_blueprint::create_router())
        .nest("/health_check", health_blueprint::create_router())
}
