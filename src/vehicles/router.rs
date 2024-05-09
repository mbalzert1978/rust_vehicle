use crate::ctx::ApiContext;

use super::*;
use axum::{
    extract::Path,
    routing::{delete, get, post, put},
};
pub fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_all))
        .route("/", post(insert))
        .route("/{id}", put(update))
        .route("/{id}", get(get_by_id))
        .route("/{id}", delete(delete_by_id))
}

async fn insert(axum::Extension(ctx): axum::Extension<ApiContext>) -> schemas::DataOne {
    todo!()
}

async fn get_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) -> schemas::DataOne {
    todo!()
}

async fn get_all(axum::Extension(ctx): axum::Extension<ApiContext>) -> schemas::DataMany {
    todo!()
}

async fn update(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) -> schemas::DataOne {
    todo!()
}

async fn delete_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) {
    todo!()
}
