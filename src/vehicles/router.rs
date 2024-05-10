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
        .route("/:id", get(get_by_id))
        .route("/:id", put(update))
        .route("/:id", delete(delete_by_id))
}

async fn insert(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
    axum::Json(payload): axum::Json<schemas::CreateVehicle>,
) -> Result<schemas::DataOne> {
    tracing::info!(correlation_id = %cid.to_string(), "insert_vehicle");
    services::insert(ctx.db.as_ref(), &payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
    Path(id): Path<uuid::Uuid>,
) -> Result<schemas::DataOne> {
    tracing::info!(correlation_id = %cid.to_string(), "get_vehicle_by_id");
    services::get_by_id(ctx.db.as_ref(), id)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_all(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
) -> Result<schemas::DataMany> {
    tracing::info!(correlation_id = %cid.to_string(), "get_all_vehicles");
    services::get_all(ctx.db.as_ref())
        .await
        .map(Into::<schemas::DataMany>::into)
}

async fn update(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
    Path(id): Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<schemas::UpdateVehicle>,
) -> Result<schemas::DataOne> {
    tracing::info!(correlation_id = %cid.to_string(), "update_vehicle");
    services::update(ctx.db.as_ref(), id, &payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn delete_by_id(
    axum::Extension(ctx): axum::Extension<ApiContext>,
    axum::Extension(cid): axum::Extension<uuid::Uuid>,
    Path(id): Path<uuid::Uuid>,
) -> Result<()> {
    tracing::info!(correlation_id = %cid.to_string(), "delete_vehicle_by_id");
    services::delete_by_id(ctx.db.as_ref(), id).await
}
