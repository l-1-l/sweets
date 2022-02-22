mod v1;

use axum::Router;

pub fn init() -> Router {
    Router::new().nest("/v1", v1::init())
}
