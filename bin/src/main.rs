#![allow(dead_code)]

mod container;
mod errors;
mod extractors;
mod prelude;
mod routes;
mod startup;
// mod ws;

use common::{
    tracing,
    tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt},
};

#[tokio::main]
async fn main() {
    let config = common::Config::get();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| tracing::Level::DEBUG.to_string()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    startup::run(config).await
}
