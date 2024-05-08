use crate::ctx::ApiContext;
use axum::{routing::get, Extension, Router};

use super::schemas::DatabaseStatus;

pub fn health() -> Router {
    Router::new().route("/", get(health_api))
}

async fn health_api(Extension(ctx): Extension<ApiContext>) -> DatabaseStatus {
    let result = sqlx::query(r#"SELECT 1=1;"#)
        .execute(ctx.db.as_ref())
        .await
        .ok();

    match result {
        Some(_) => DatabaseStatus::ok(),
        None => DatabaseStatus::error(),
    }
}
