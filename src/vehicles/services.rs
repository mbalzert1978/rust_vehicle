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
            id,
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
            $5,
            $6
        ) 
        RETURNING
            id,
            name,
            manufacturer,
            manufacturing_year,
            is_driveable,
            body;
        ",
        uuid::Uuid::now_v7(),
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
        ORDER BY
            id ASC
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
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn get_test_vehicle(pool: &sqlx::Pool<sqlx::Postgres>) -> schemas::Vehicle {
        let to_create = schemas::CreateVehicle {
            name: "test_vehicle".to_string(),
            manufacturer: Some("test_manufacturer".to_string()),
            manufacturing_year: Some(2021),
            is_driveable: true,
            body: serde_json::json!({
                "foo" : ["bar", "baz"]
            }),
        };
        let result = insert(pool, &to_create).await;

        assert!(result.is_ok(), "FAIL: Could not insert vehicle.");

        result.unwrap()
    }

    #[sqlx::test()]
    async fn insert_vehicle_when_called_with_valid_create_vehicle_should_insert_into_db_and_retun_the_newly_created_vehicle(
        pool: sqlx::PgPool,
    ) {
        let to_create = schemas::CreateVehicle {
            name: "test_vehicle".to_string(),
            manufacturer: Some("test_manufacturer".to_string()),
            manufacturing_year: Some(2021),
            is_driveable: true,
            body: serde_json::json!({
                "foo" : ["bar", "baz"]
            }),
        };
        let result = insert(&pool, &to_create).await;
        assert!(result.is_ok(), "FAIL: Could not insert vehicle.");

        let found = get_by_id(&pool, result.unwrap().id).await;
        assert!(found.is_ok(), "FAIL: Could not get vehicle by id.");

        let found = found.unwrap();
        assert_eq!(to_create.name, found.name);
        assert_eq!(to_create.manufacturer, found.manufacturer);
        assert_eq!(to_create.manufacturing_year, found.manufacturing_year);
        assert_eq!(to_create.is_driveable, found.is_driveable);
        assert_eq!(to_create.body, found.body);
    }

    #[sqlx::test()]
    async fn get_vehicle_by_id_when_called_with_valid_id_should_return_the_vehicle(
        pool: sqlx::PgPool,
    ) {
        let expected = get_test_vehicle(&pool).await;
        let result = get_by_id(&pool, expected.id).await;

        assert!(result.is_ok(), "FAIL: Could not get vehicle by id.");
        assert_eq!(expected, result.unwrap());
    }

    #[sqlx::test()]
    async fn get_all_vehicles_when_called_should_return_all_vehicles(pool: sqlx::PgPool) {
        let expected = get_test_vehicle(&pool).await;
        let result = get_all(&pool).await;

        assert!(result.is_ok(), "FAIL: Could not get all vehicles.");
        assert!(
            result.unwrap().contains(&expected),
            "FAIL: Expected vehicle not in result."
        );
    }

    #[sqlx::test()]
    async fn update_vehicle_by_id_when_called_with_valid_id_should_update_the_vehicle(
        pool: sqlx::PgPool,
    ) {
        let to_update = get_test_vehicle(&pool).await;
        let new_values = schemas::UpdateVehicle {
            name: Some("updated_name".to_string()),
            manufacturer: None,
            manufacturing_year: None,
            is_driveable: None,
            body: Some(serde_json::json!({
                "baz":["foo"]
            })),
        };
        let result = update(&pool, to_update.id, &new_values).await;

        assert!(result.is_ok(), "FAIL: Could not update vehicle.");

        let result = result.unwrap();

        assert_eq!(to_update.manufacturer, result.manufacturer);
        assert_eq!(to_update.manufacturing_year, result.manufacturing_year);
        assert_eq!(to_update.is_driveable, result.is_driveable);
        assert_eq!(new_values.name, Some(result.name));
        assert_eq!(new_values.body, Some(result.body));
    }

    #[sqlx::test()]
    async fn delete_vehicle_by_id_when_called_with_valid_id_should_delete_the_vehicle(
        pool: sqlx::PgPool,
    ) {
        let to_delete = get_test_vehicle(&pool).await;

        assert!(
            delete_by_id(&pool, to_delete.id).await.is_ok(),
            "FAIL: Could not delete vehicle."
        );

        let result = get_by_id(&pool, to_delete.id).await;

        assert!(result.is_err(), "FAIL: Vehicle should not exist.");

        match result {
            Err(Error::NotFound { .. }) => (),
            _ => panic!("FAIL: Error should be NotFound."),
        }
    }
}
