#[derive(serde::Deserialize)]
pub struct CreateVehicle {
    id: uuid::Uuid,
    name: String,
    manufacturer: String,
    manufacturing_year: u32,
    is_driveable: bool,
}
#[derive(serde::Deserialize)]
pub struct UpdateVehicle {
    name: Option<String>,
    manufacturer: Option<String>,
    manufacturing_year: Option<u32>,
    is_driveable: Option<bool>,
}

#[derive(serde::Serialize)]
pub struct Vehicle {
    id: uuid::Uuid,
    name: String,
    manufacturer: String,
    manufacturing_year: u32,
    is_driveable: bool,
}

#[derive(serde::Serialize)]
pub struct DataOne(Vehicle);

#[derive(serde::Serialize)]
pub struct DataMany(Vec<Vehicle>);

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
