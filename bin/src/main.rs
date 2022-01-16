#![allow(dead_code)]

mod container;
mod errors;
mod extractors;
mod prelude;
mod routes;
mod startup;
// mod ws;

#[tokio::main]
async fn main() {
    let config = common::Config::get();

    std::env::set_var("RUST_LOG", "rust_long_polling=info");
    tracing_subscriber::fmt::init();

    startup::run(config).await
}
