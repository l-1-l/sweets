use crate::{container::AppContainer, routes};
use axum::extract::Extension;
use common::{tracing, Config};
use std::{net::SocketAddr, sync::Arc};
use tokio_postgres::NoTls;
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer,
};

pub async fn run(config: &Config) {
    let dp_runtime = Some(deadpool_runtime::Runtime::Tokio1);

    let pg_pool = Arc::new(
        config
            .postgres()
            .with_db()
            .create_pool(dp_runtime, NoTls)
            .expect("failed to connect to postgres."),
    );

    let redis_pool = Arc::new(
        config
            .redis()
            .create_config()
            .create_pool(dp_runtime)
            .expect("failed to connect to redis"),
    );

    let lapin_pool = Arc::new(
        config
            .rabbit()
            .create_config()
            .create_pool(dp_runtime)
            .expect("failed to connect to rabbit"),
    );

    let shared_settings = Arc::new(config.clone());
    let app_container = AppContainer::new(
        shared_settings.clone(),
        pg_pool.clone(),
        redis_pool.clone(),
        lapin_pool.clone(),
    )
    .await;

    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(Extension(app_container))
        .layer(CorsLayer::permissive())
        .into_inner();

    let router = routes::init().layer(middlewares);

    let addr = SocketAddr::from((config.service().bind, config.service().port));

    tracing::debug!("http server is listening on \"{}\"", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
