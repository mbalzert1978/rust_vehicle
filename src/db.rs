use std::sync::Arc;

use axum::Extension;
use sqlx::PgPool;
use tower::layer::util::Stack;
use tower::ServiceBuilder;

use crate::config::Config;
use crate::prelude::*;

pub async fn get_pool(
    cfg: Config,
) -> Result<ServiceBuilder<Stack<Extension<Arc<PgPool>>, tower::layer::util::Identity>>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(cfg.max_pool_size)
        .connect(cfg.database_url.as_str())
        .await?;

    let pool = std::sync::Arc::new(pool);

    Ok(ServiceBuilder::new().layer(Extension(pool)))
}
