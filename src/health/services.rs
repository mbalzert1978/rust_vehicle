use super::*;
use crate::prelude::*;

pub(crate) async fn get_database_status(
    pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<sqlx::postgres::PgQueryResult> {
    Ok(sqlx::query(constants::QUERY).execute(pool).await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test()]
    async fn get_database_status_should_return_ok(pool: sqlx::PgPool) {
        let result = get_database_status(&pool).await.ok();
        assert!(result.is_some(), "FAIL: Could not get database status.");
    }
}
