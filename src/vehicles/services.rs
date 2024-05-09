use super::*;
use crate::prelude::*;

pub(crate) async fn insert(
    ctx: crate::ctx::ApiContext,
    payload: schemas::CreateVehicle,
) -> Result<schemas::Vehicle> {
    sqlx::query_as!(
        schemas::Vehicle,
        "
        INSERT INTO vehicles 
        (name, manufacturer, manufacturing_year, is_driveable, body) 
        VALUES 
        ($1, $2, $3, $4, $5) 
        RETURNING *;
        ",
        payload.name,
        payload.manufacturer,
        payload.manufacturing_year,
        payload.is_driveable,
        payload.body
    )
    .fetch_one(ctx.db.as_ref())
    .await
    .map_err(Error::generic)
}

pub(crate) async fn get_by_id(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
) -> Result<Option<schemas::Vehicle>> {
    sqlx::query_as!(
        schemas::Vehicle,
        "
        SELECT * FROM vehicles WHERE id = $1;
        ",
        id
    )
    .fetch_optional(ctx.db.as_ref())
    .await
    .map_err(Error::generic)
}

pub(crate) async fn get_all(ctx: crate::ctx::ApiContext) -> Result<Vec<schemas::Vehicle>> {
    sqlx::query_as!(schemas::Vehicle, "SELECT * FROM vehicles;")
        .fetch_all(ctx.db.as_ref())
        .await
        .map_err(Error::generic)
}

pub(crate) async fn update(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
    payload: schemas::UpdateVehicle,
) -> Result<schemas::Vehicle> {
    sqlx::query_as!(
        schemas::Vehicle,
        "
        UPDATE vehicles 
        SET name = $2, 
        manufacturer = $3, 
        manufacturing_year = $4, 
        is_driveable = $5, 
        body = $6
        WHERE id = $1
        RETURNING *;
        ",
        id,
        payload.name,
        payload.manufacturer,
        payload.manufacturing_year,
        payload.is_driveable,
        payload.body
    )
    .fetch_one(ctx.db.as_ref())
    .await
    .map_err(Error::generic)
}

pub(crate) async fn delete_by_id(ctx: crate::ctx::ApiContext, id: uuid::Uuid) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM vehicles
        WHERE id = $1;
        ",
        id
    )
    .execute(ctx.db.as_ref())
    .await
    .map_err(Error::generic)?;
    Ok(())
}
