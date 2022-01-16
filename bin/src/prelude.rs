use axum::Json;
use hyper::StatusCode;
use serde::Serialize;

pub use crate::errors::Error;

pub type Result<T> = std::result::Result<T, Error>;
pub type ApiResult<T> = std::result::Result<(StatusCode, Json<T>), Error>;

fn with<T: Serialize>(status: StatusCode, data: T) -> ApiResult<T> {
    Ok((status, Json(data)))
}

pub fn ok<T: Serialize>(data: T) -> ApiResult<T> {
    with(StatusCode::OK, data)
}
pub fn created<T: Serialize>(data: T) -> ApiResult<T> {
    with(StatusCode::CREATED, data)
}
