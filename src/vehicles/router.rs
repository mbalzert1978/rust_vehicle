use super::*;
use crate::ctx::ApiContext;
use crate::prelude::*;

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

async fn insert(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Json(payload): axum::Json<schemas::CreateVehicle>,
) -> Result<schemas::DataOne> {
    services::insert(ctx, payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) -> Result<schemas::DataOne> {
    services::get_by_id(ctx, id)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_all(axum::Extension(ctx): axum::Extension<ApiContext>) -> Result<schemas::DataMany> {
    services::get_all(ctx)
        .await
        .map(Into::<schemas::DataMany>::into)
}

async fn update(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<schemas::UpdateVehicle>,
) -> Result<schemas::DataOne> {
    services::update(ctx, id, payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn delete_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    Path(id): Path<uuid::Uuid>,
) -> Result<()> {
    services::delete_by_id(ctx, id).await
}
