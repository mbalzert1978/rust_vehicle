use super::*;
use crate::prelude::*;

const LIMIT: i64 = 1000; //TODO: replace with parameter.

pub(crate) async fn insert(
    ctx: crate::ctx::ApiContext,
    payload: schemas::CreateVehicle,
) -> Result<schemas::Vehicle> {
    Ok(sqlx::query_as!(
        schemas::Vehicle,
        "
        INSERT INTO
            vehicles 
        (
            name,
            manufacturer,
            manufacturing_year,
            is_driveable,
            body
        ) 
        VALUES 
        (
            $1,
            $2,
            $3,
            $4,
            $5
        ) 
        RETURNING
        id,
        name,
        manufacturer,
        manufacturing_year,
        is_driveable,
        body;
        ",
        payload.name,
        payload.manufacturer,
        payload.manufacturing_year,
        payload.is_driveable,
        payload.body
    )
    .fetch_one(ctx.db.as_ref())
    .await?)
}

pub(crate) async fn get_by_id(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
) -> Result<schemas::Vehicle> {
    Ok(sqlx::query_as!(
        schemas::Vehicle,
        "
        SELECT
            id,
            name,
            manufacturer,
            manufacturing_year,
            is_driveable,
            body
        FROM
            vehicles
        WHERE
            id = $1;
        ",
        id
    )
    .fetch_one(ctx.db.as_ref())
    .await?)
}

pub(crate) async fn get_all(ctx: crate::ctx::ApiContext) -> Result<Vec<schemas::Vehicle>> {
    Ok(sqlx::query_as!(
        schemas::Vehicle,
        "
        SELECT
            id,
            name,
            manufacturer,
            manufacturing_year,
            is_driveable,
            body
        FROM
            vehicles
        LIMIT 
            $1;
        ",
        LIMIT,
    )
    .fetch_all(ctx.db.as_ref())
    .await?)
}

pub(crate) async fn update(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
    payload: schemas::UpdateVehicle,
) -> Result<schemas::Vehicle> {
    Ok(sqlx::query_as!(
        schemas::Vehicle,
        "
        UPDATE
            vehicles 
        SET
            name = COALESCE($2, name), 
            manufacturer = COALESCE($3, manufacturer), 
            manufacturing_year = COALESCE($4, manufacturing_year), 
            is_driveable = COALESCE($5, is_driveable), 
            body = COALESCE($6, body)
        WHERE
            id = $1
        RETURNING
            id,
            name,
            manufacturer,
            manufacturing_year,
            is_driveable,
            body;
        ",
        id,
        payload.name,
        payload.manufacturer,
        payload.manufacturing_year,
        payload.is_driveable,
        payload.body
    )
    .fetch_one(ctx.db.as_ref())
    .await?)
}

pub(crate) async fn delete_by_id(ctx: crate::ctx::ApiContext, id: uuid::Uuid) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM
            vehicles
        WHERE
            id = $1;
        ",
        id
    )
    .fetch_one(ctx.db.as_ref())
    .await?;
    Ok(())
}
