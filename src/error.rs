use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[cfg_attr(test, derive(PartialEq))]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // Setup
    Generic { detail: String },

    // IO
    IO { detail: String },

    // Runtime
    RunTime { detail: String },

    // HTTP Errors
    NotAllowed { detail: String },
    NotFound { detail: String },
    BadRequest { detail: String },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error::not_found(value),
            _ => Error::generic(value),
        }
    }
}

impl Error {
    pub fn not_found<E: ToString>(error: E) -> Self {
        Error::NotFound {
            detail: error.to_string(),
        }
    }
    pub fn bad_request<E: ToString>(error: E) -> Self {
        Error::BadRequest {
            detail: error.to_string(),
        }
    }
    pub fn not_allowed<E: ToString>(error: E) -> Self {
        Error::NotAllowed {
            detail: error.to_string(),
        }
    }
    pub fn generic<E: ToString>(error: E) -> Self {
        Error::Generic {
            detail: error.to_string(),
        }
    }
    pub fn io<E: ToString>(error: E) -> Self {
        Error::IO {
            detail: error.to_string(),
        }
    }
    pub fn runtime<E: ToString>(error: E) -> Self {
        Error::RunTime {
            detail: error.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::NotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            Error::BadRequest { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::to_string(&self).unwrap_or_default();

        (status, body).into_response()
    }
}
