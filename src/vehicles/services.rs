use crate::prelude::*;

pub(crate) async fn insert(
    ctx: crate::ctx::ApiContext,
    payload: super::schemas::CreateVehicle,
) -> Result<super::schemas::Vehicle> {
    todo!()
}

pub(crate) async fn get_by_id(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
) -> Result<Option<super::schemas::Vehicle>> {
    todo!()
}

pub(crate) async fn get_all(
    ctx: crate::ctx::ApiContext,
) -> Result<Vec<Option<super::schemas::Vehicle>>> {
    todo!()
}

pub(crate) async fn update(
    ctx: crate::ctx::ApiContext,
    id: uuid::Uuid,
    payload: super::schemas::UpdateVehicle,
) -> Result<super::schemas::Vehicle> {
    todo!()
}

pub(crate) async fn delete_by_id(ctx: crate::ctx::ApiContext, id: uuid::Uuid) -> Result<()> {
    todo!()
}
