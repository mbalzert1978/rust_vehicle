use super::*;
use crate::prelude::*;

pub(crate) async fn insert(
    ctx: crate::ctx::ApiContext,
    payload: schemas::CreateVehicle,
) -> Result<schemas::Vehicle> {
    todo!()
}

pub(crate) async fn get_by_id(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
) -> Result<Option<schemas::Vehicle>> {
    todo!()
}

pub(crate) async fn get_all(ctx: crate::ctx::ApiContext) -> Result<Vec<Option<schemas::Vehicle>>> {
    let vehicle = sqlx::query_as!(schemas::Vehicle, r#"SELECT * FROM vehicles"#)
        .fetch_all(ctx.db.as_ref())
        .await
        .map_err(Error::generic);
}

pub(crate) async fn update(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
    payload: schemas::UpdateVehicle,
) -> Result<schemas::Vehicle> {
    todo!()
}

pub(crate) async fn delete_by_id(ctx: crate::ctx::ApiContext, id: uuid::Uuid) -> Result<()> {
    todo!()
}
