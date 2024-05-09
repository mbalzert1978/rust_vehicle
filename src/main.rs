use axum::middleware::from_fn;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::prelude::*;

mod config;
mod constants;
mod ctx;
mod error;
mod fallback;
mod health;
mod io;
mod logging;
mod middleware;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::get_config()?;

    let directory = io::create_directory(&config.directory)?;
    let file = io::create_or_open_file(&config.file_name, directory)?;

    logging::init_tracing(file)?;
    tracing::info!("{}", constants::STARTING);

    let ctx = ctx::get_ctx(config.clone()).await?;

    let health = health::router::routes();

    let routes = axum::Router::new().nest(health::Tag::get(), health);

    let app = axum::Router::new()
        .nest(constants::Prefix::get(), routes)
        .fallback(fallback::fallback)
        .layer(ctx)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(5)))
        .layer(TraceLayer::new_for_http())
        .layer(from_fn(middleware::inject_cid));

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &config.host, &config.port))
        .await
        .map_err(Error::generic)?;

    tracing::info!(
        "listening on {}",
        listener.local_addr().map_err(Error::generic)?
    );

    axum::serve(listener, app).await.map_err(Error::generic)?;
    Ok(())
}
