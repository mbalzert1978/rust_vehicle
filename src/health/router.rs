use super::*;
use axum::{routing::get, Extension, Router};
use std::sync::Arc;

pub(crate) fn routes() -> Router {
    Router::new().route("/", get(health_api))
}

async fn health_api(Extension(pool): Extension<Arc<sqlx::PgPool>>) -> schemas::DatabaseStatus {
    services::get_database_status(pool.as_ref())
        .await
        .ok()
        .into()
}
