use std::collections::HashMap;

use serde_json::Value;

#[derive(serde::Deserialize)]
pub(crate) struct CreateVehicle {
    id: uuid::Uuid,
    name: String,
    manufacturer: String,
    manufacturing_year: u32,
    is_driveable: bool,
    body: HashMap<String, Value>,
}
#[derive(serde::Deserialize)]
pub(crate) struct UpdateVehicle {
    name: Option<String>,
    manufacturer: String,
    manufacturing_year: Option<u32>,
    is_driveable: Option<bool>,
    body: HashMap<String, Value>,
}

#[derive(serde::Serialize, Clone)]
pub(crate) struct Vehicle {
    id: uuid::Uuid,
    name: String,
    manufacturer: String,
    manufacturing_year: u32,
    is_driveable: bool,
    body: HashMap<String, Value>,
    created_at: String,
    updated_at: String,
}

impl From<Option<Vehicle>> for DataOne {
    fn from(value: Option<Vehicle>) -> Self {
        DataOne(value)
    }
}

impl From<Vehicle> for DataOne {
    fn from(value: Vehicle) -> Self {
        DataOne(Some(value))
    }
}

impl From<Vec<Option<Vehicle>>> for DataMany {
    fn from(value: Vec<Option<Vehicle>>) -> Self {
        let vehicle = value.iter().flatten().cloned().collect::<Vec<_>>();
        DataMany(vehicle)
    }
}

impl From<Vec<Vehicle>> for DataMany {
    fn from(value: Vec<Vehicle>) -> Self {
        DataMany(value)
    }
}

#[derive(serde::Serialize)]
pub(crate) struct DataOne(Option<Vehicle>);

#[derive(serde::Serialize)]
pub(crate) struct DataMany(Vec<Vehicle>);

impl axum::response::IntoResponse for DataOne {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_default();
        let status = axum::http::StatusCode::OK;
        (status, body).into_response()
    }
}

impl axum::response::IntoResponse for DataMany {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_default();
        let status = axum::http::StatusCode::OK;
        (status, body).into_response()
    }
}
