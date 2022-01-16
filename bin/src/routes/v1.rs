mod auth;

use axum::Router;

pub fn init() -> Router {
    Router::new().nest("/auth", auth::init())
}
