use crate::{config::get_config, prelude::*};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct Database(Arc<Pool<Postgres>>);

impl Database {
    pub async fn new() -> Result<Database> {
        let config = get_config()?;
        let pool = PgPoolOptions::new()
            .max_connections(config.max_pool_size)
            .connect(config.database_url.as_str())
            .await
            .map_err(|e| Error::Generic {
                message: e.to_string(),
            })?;

        Ok(Database(Arc::new(pool)))
    }
}
