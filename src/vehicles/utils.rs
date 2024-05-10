use axum::Json;
use serde_json::json;

pub(crate) fn serializer<T: Sized + serde::Serialize>(
    value: &T,
    status: axum::http::StatusCode,
) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    let body = Json(json!(&value));
    (status, body)
}
