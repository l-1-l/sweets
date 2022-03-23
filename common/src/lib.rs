mod env_utils;
mod validator;

pub mod cache;
pub mod config;
pub mod dtos;
pub mod error;
pub mod event;
pub mod infra;
pub mod models;
pub mod pg;
pub mod random;
pub mod result;
pub mod sql;

use std::sync::Arc;

// re-export deps
pub use async_trait::async_trait;
pub use celery;
pub use chrono;
pub use deadpool_lapin;
pub use deadpool_redis;
pub use once_cell;
pub use tracing;
pub use tracing_subscriber;

pub use config::Config;
pub use error::{Error, Result};

pub type SharedPgPool = Arc<pg::Pool>;
pub type SharedRedisPool = Arc<deadpool_redis::Pool>;
pub type SharedLapinPool = Arc<deadpool_lapin::Pool>;
pub type SharedConfig = Arc<Config>;
