use axum::Extension;
use tower::layer::util::Stack;
use tower::ServiceBuilder;

use crate::config::Config;
use crate::prelude::*;

#[derive(Clone)]
pub struct ApiContext {
    pub config: std::sync::Arc<Config>,
    pub db: std::sync::Arc<sqlx::Pool<sqlx::postgres::Postgres>>,
}

pub async fn get_ctx(
    cfg: Config,
) -> Result<ServiceBuilder<Stack<Extension<ApiContext>, tower::layer::util::Identity>>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.max_pool_size)
        .connect(cfg.database_url.as_str())
        .await?;

    let pool = std::sync::Arc::new(pool);

    let api_context = ApiContext {
        config: std::sync::Arc::new(cfg),
        db: pool,
    };

    Ok(ServiceBuilder::new().layer(Extension(api_context)))
}
