use super::*;

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
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct Vehicle {
    pub(crate) id: uuid::Uuid,
    pub(crate) name: String,
    pub(crate) manufacturer: Option<String>,
    pub(crate) manufacturing_year: Option<i32>,
    pub(crate) is_driveable: bool,
    pub(crate) body: serde_json::Value,
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
        utils::serializer(&self, axum::http::StatusCode::OK).into_response()
    }
}

impl axum::response::IntoResponse for DataMany {
    fn into_response(self) -> axum::response::Response {
        utils::serializer(&self, axum::http::StatusCode::OK).into_response()
    }
}
