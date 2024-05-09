use super::*;
use crate::prelude::*;

pub(crate) async fn get_database_status(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<sqlx::postgres::PgQueryResult> {
    sqlx::query(constants::QUERY)
        .execute(pool)
        .await
        .map_err(Error::generic)
}
