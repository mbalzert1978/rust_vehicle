use super::*;
use crate::prelude::*;

use axum::{
    extract::Path,
    routing::{delete, get, post, put},
};
use sqlx::PgPool;
use std::sync::Arc;
pub(crate) fn routes() -> axum::Router {
    axum::Router::new()
        .route("/", get(get_all))
        .route("/", post(insert))
        .route("/:id", get(get_by_id))
        .route("/:id", put(update))
        .route("/:id", delete(delete_by_id))
}

async fn insert(
    axum::Extension(pool): axum::Extension<Arc<PgPool>>,
    axum::Json(payload): axum::Json<schemas::CreateVehicle>,
) -> Result<schemas::DataOne> {
    services::insert(pool.as_ref(), &payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_by_id(
    axum::Extension(pool): axum::Extension<Arc<PgPool>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<schemas::DataOne> {
    services::get_by_id(pool.as_ref(), id)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn get_all(axum::Extension(pool): axum::Extension<Arc<PgPool>>) -> Result<schemas::DataMany> {
    services::get_all(pool.as_ref())
        .await
        .map(Into::<schemas::DataMany>::into)
}

async fn update(
    axum::Extension(pool): axum::Extension<Arc<PgPool>>,
    Path(id): Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<schemas::UpdateVehicle>,
) -> Result<schemas::DataOne> {
    services::update(pool.as_ref(), id, &payload)
        .await
        .map(Into::<schemas::DataOne>::into)
}

async fn delete_by_id(
    axum::Extension(pool): axum::Extension<Arc<PgPool>>,
    Path(id): Path<uuid::Uuid>,
) -> Result<()> {
    services::delete_by_id(pool.as_ref(), id).await
}
