use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use sqlx::postgres::PgQueryResult;

use crate::utils::serializer;

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

impl From<Option<PgQueryResult>> for DatabaseStatus {
    fn from(value: Option<PgQueryResult>) -> Self {
        match value {
            Some(_) => DatabaseStatus::ok(),
            None => DatabaseStatus::error(),
        }
    }
}

impl IntoResponse for DatabaseStatus {
    fn into_response(self) -> axum::response::Response {
        serializer(&self, StatusCode::OK).into_response()
    }
}
