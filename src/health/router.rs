use super::*;
use crate::ctx::ApiContext;
use axum::{routing::get, Extension, Router};

pub fn routes() -> Router {
    Router::new().route("/", get(health_api))
}

async fn health_api(Extension(ctx): Extension<ApiContext>) -> schemas::DatabaseStatus {
    let executor = ctx.db.as_ref();
    let result = services::get_database_status(executor).await.ok();

    match result {
        Some(_) => schemas::DatabaseStatus::ok(),
        None => schemas::DatabaseStatus::error(),
    }
}
