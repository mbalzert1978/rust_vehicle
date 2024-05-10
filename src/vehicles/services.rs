use super::*;
use crate::prelude::*;

const LIMIT: i64 = 1000; //TODO: replace with parameter.

pub(crate) async fn insert(
    pool: &sqlx::Pool<sqlx::Postgres>,
    payload: &schemas::CreateVehicle,
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
    .fetch_one(pool)
    .await?)
}

pub(crate) async fn get_by_id(
    pool: &sqlx::Pool<sqlx::Postgres>,
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
    .fetch_one(pool)
    .await?)
}

pub(crate) async fn get_all(pool: &sqlx::Pool<sqlx::Postgres>) -> Result<Vec<schemas::Vehicle>> {
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
    .fetch_all(pool)
    .await?)
}

pub(crate) async fn update(
    pool: &sqlx::Pool<sqlx::Postgres>,
    id: uuid::Uuid,
    payload: &schemas::UpdateVehicle,
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
    .fetch_one(pool)
    .await?)
}

pub(crate) async fn delete_by_id(pool: &sqlx::Pool<sqlx::Postgres>, id: uuid::Uuid) -> Result<()> {
    sqlx::query!(
        "
        DELETE FROM
            vehicles
        WHERE
            id = $1;
        ",
        id
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test()]
    async fn insert_vehicle_when_called_with_valid_create_vehicle_should_insert_into_db_and_retun_the_newly_created_vehicle(
        pool: sqlx::PgPool,
    ) {
        let to_create = schemas::CreateVehicle {
            name: "test_vehicle".to_string(),
            manufacturer: Some("test".to_string()),
            manufacturing_year: Some(2021),
            is_driveable: true,
            body: serde_json::json!({
                "test": "test"
            }),
        };
        let result = insert(&pool, &to_create)
            .await
            .expect("Could not insert vehicle.");

        let found = sqlx::query_as!(
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
            result.id,
        )
        .fetch_one(&pool)
        .await
        .expect("Vehicle not found.");

        assert_eq!(to_create.name, found.name);
        assert_eq!(to_create.manufacturer, found.manufacturer);
        assert_eq!(to_create.manufacturing_year, found.manufacturing_year);
        assert_eq!(to_create.is_driveable, found.is_driveable);
        assert_eq!(to_create.body, found.body);
    }
}
