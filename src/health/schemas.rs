use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type")]
pub(crate) struct DatabaseStatus {
    status: String,
}

impl DatabaseStatus {
    pub(crate) fn ok() -> Self {
        Self {
            status: String::from("OK"),
        }
    }

    pub(crate) fn error() -> Self {
        Self {
            status: String::from("ERROR"),
        }
    }
}

impl IntoResponse for DatabaseStatus {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self).unwrap_or_default();

        (StatusCode::OK, body).into_response()
    }
}
