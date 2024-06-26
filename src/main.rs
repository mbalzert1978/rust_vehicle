use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

use crate::prelude::*;

mod config;
mod constants;
mod db;
mod error;
mod fallback;
mod health;
mod io;
mod logging;
mod prelude;
mod span;
mod strings;
mod utils;
mod vehicles;

#[tokio::main]
async fn main() -> Result<()> {
    let config = config::get_config()?;

    let directory = io::create_directory(&config.directory)?;
    let file = io::create_or_open_file(&config.file_name, directory)?;

    logging::init_tracing(file)?;
    tracing::info!("{}", strings::en::STARTUP);

    let pool = db::get_pool(config.clone()).await?;

    let health = health::router::routes();
    let vehicles = vehicles::router::routes();

    let routes = axum::Router::new()
        .nest(health::Tag::get(), health)
        .nest(vehicles::Tag::get(), vehicles);

    let app = axum::Router::new()
        .nest(constants::Prefix::get(), routes)
        .fallback(fallback::fallback)
        .layer(pool)
        .layer(TimeoutLayer::new(std::time::Duration::from_secs(
            constants::TIMEOUT_SECONDS,
        )))
        .layer(TraceLayer::new_for_http().make_span_with(span::CidSpan));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", &config.host, &config.port)).await?;

    tracing::info!("{}{}", strings::en::ACCEPTING, listener.local_addr()?);

    axum::serve(listener, app).await?;
    Ok(())
}
