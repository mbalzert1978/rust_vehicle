use crate::utils::serializer;

#[derive(serde::Deserialize)]
pub(crate) struct CreateVehicle {
    pub(crate) name: String,
    pub(crate) manufacturer: Option<String>,
    pub(crate) manufacturing_year: Option<i32>,
    pub(crate) is_driveable: bool,
    pub(crate) body: serde_json::Value,
}
#[derive(serde::Deserialize)]
pub(crate) struct UpdateVehicle {
    pub(crate) name: Option<String>,
    pub(crate) manufacturer: Option<String>,
    pub(crate) manufacturing_year: Option<i32>,
    pub(crate) is_driveable: Option<bool>,
    pub(crate) body: Option<serde_json::Value>,
}

#[derive(serde::Serialize, Clone)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct Vehicle {
    pub(crate) id: uuid::Uuid,
    pub(crate) name: String,
    pub(crate) manufacturer: Option<String>,
    pub(crate) manufacturing_year: Option<i32>,
    pub(crate) is_driveable: bool,
    pub(crate) body: serde_json::Value,
}

impl From<Option<Vehicle>> for Product {
    fn from(value: Option<Vehicle>) -> Self {
        Product { data: value }
    }
}

impl From<Vehicle> for Product {
    fn from(value: Vehicle) -> Self {
        Product { data: Some(value) }
    }
}

impl From<Vec<Vehicle>> for Products {
    fn from(value: Vec<Vehicle>) -> Self {
        Products { data: value }
    }
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct Product {
    data: Option<Vehicle>,
}

#[derive(serde::Serialize)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct Products {
    data: Vec<Vehicle>,
}

impl axum::response::IntoResponse for Product {
    fn into_response(self) -> axum::response::Response {
        serializer(&self, axum::http::StatusCode::OK).into_response()
    }
}

impl axum::response::IntoResponse for Products {
    fn into_response(self) -> axum::response::Response {
        serializer(&self, axum::http::StatusCode::OK).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::StatusCode, response::IntoResponse};
    use serde_json::json;

    fn get_test_vehicle() -> Vehicle {
        Vehicle {
            id: uuid::Uuid::now_v7(),
            name: "Test Vehicle".to_string(),
            manufacturer: None,
            manufacturing_year: None,
            is_driveable: false,
            body: json!({}),
        }
    }

    #[tokio::test]
    async fn when_creating_vehicle_instance_should_return_valid_instance() {
        let vehicle = Vehicle {
            id: uuid::Uuid::now_v7(),
            name: "Test Vehicle".to_string(),
            manufacturer: Some("Test Manufacturer".to_string()),
            manufacturing_year: Some(2020),
            is_driveable: true,
            body: json!({"color": "red"}),
        };

        assert_eq!(vehicle.name, "Test Vehicle");
        assert_eq!(vehicle.manufacturer, Some("Test Manufacturer".to_string()));
        assert_eq!(vehicle.manufacturing_year, Some(2020));
        assert!(vehicle.is_driveable);
    }

    #[tokio::test]
    async fn when_calling_into_on_vehicle_instance_should_return_valid_data_one_instance() {
        let test_vehicle = get_test_vehicle();
        let expected = Product {
            data: Some(test_vehicle.clone()),
        };
        let result: Product = test_vehicle.into();
        assert_eq!(result, expected);
    }

    #[tokio::test]
    async fn when_calling_into_on_vehicles_vector_should_return_valid_data_many_instance() {
        let vehicles = vec![
            Vehicle {
                id: uuid::Uuid::now_v7(),
                name: "Vehicle 1".to_string(),
                manufacturer: None,
                manufacturing_year: None,
                is_driveable: true,
                body: json!({}),
            },
            Vehicle {
                id: uuid::Uuid::now_v7(),
                name: "Vehicle 2".to_string(),
                manufacturer: None,
                manufacturing_year: None,
                is_driveable: false,
                body: json!({}),
            },
        ];

        let vehicles: Products = vehicles.into();
        assert_eq!(vehicles.data.len(), 2);
    }

    #[tokio::test]
    async fn when_into_response_is_called_on_data_one_instance_should_return_valid_response() {
        let data_one: Product = get_test_vehicle().into();
        let response = data_one.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn when_into_response_is_called_on_data_many_instance_should_return_valid_response() {
        let vehicles = vec![
            Vehicle {
                id: uuid::Uuid::now_v7(),
                name: "Vehicle 1".to_string(),
                manufacturer: None,
                manufacturing_year: None,
                is_driveable: true,
                body: json!({}),
            },
            Vehicle {
                id: uuid::Uuid::now_v7(),
                name: "Vehicle 2".to_string(),
                manufacturer: None,
                manufacturing_year: None,
                is_driveable: false,
                body: json!({}),
            },
        ];

        let data_many: Products = vehicles.into();
        let response = data_many.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
