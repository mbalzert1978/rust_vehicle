use crate::{config::get_config, logging::init_tracing, prelude::*};

mod config;
mod constants;
mod error;
mod io;
mod logging;
mod prelude;
mod health;
mod database;

#[tokio::main]
async fn main() -> Result<()> {
    let config = get_config()?;

    let directory = io::create_directory(&config.directory)?;
    let file = io::create_or_open_file(&config.file_name, &directory)?;

    init_tracing(file)?;
    tracing::info!("{}", constants::STARTING);

    let shared_config = std::sync::Arc::new(config.clone());
    todo!();

    // let urls = urls::read::routes();

    // let routes = Router::new().nest("/urls", urls);

    // let app = Router::new()
    //     .nest(constants::PREFIX, routes)
    //     .fallback(fallback::fallback)
    //     .layer(Extension(shared_config))
    //     .layer(TraceLayer::new_for_http())
    //     .layer(from_fn(middleware::handle_method_not_allowed));

    // let listener = TcpListener::bind(format!("{}:{}", &config.host, &config.port))
    //     .await
    //     .map_err(|_| Error::BindError {
    //         message: constants::ERR_TCP_LISTENER.to_string(),
    //     })?;

    // tracing::info!(
    //     "listening on {}",
    //     listener.local_addr().map_err(|e| Error::LoggingError {
    //         message: e.to_string()
    //     })?
    // );

    // axum::serve(listener, app)
    //     .await
    //     .map_err(|_| Error::ServeError {
    //         message: constants::ERR_SERVER_CREATE_FAILED.to_string(),
    //     })?;
    // Ok(())
}
