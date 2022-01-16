mod v1;

use axum::Router;

pub fn init() -> Router {
    Router::new().nest("/1", v1::init())
}
