use super::*;
use crate::ctx::ApiContext;
use axum::{routing::get, Extension, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(health_api))
}

async fn health_api(Extension(ctx): Extension<ApiContext>) -> schemas::DatabaseStatus {
    services::get_database_status(ctx.db.as_ref())
        .await
        .ok()
        .into()
}
