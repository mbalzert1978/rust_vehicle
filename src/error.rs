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
    Generic { message: String },

    // IO
    IO { message: String },

    // Runtime
    RunTime { message: String },

    // HTTP Errors
    NotAllowed { message: String },
    NotFound { message: String },
    BadRequest { message: String },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl Error {
    pub fn not_found<E: ToString>(error: E) -> Self {
        Error::NotFound {
            message: error.to_string(),
        }
    }
    pub fn bad_request<E: ToString>(error: E) -> Self {
        Error::BadRequest {
            message: error.to_string(),
        }
    }
    pub fn not_allowed<E: ToString>(error: E) -> Self {
        Error::NotAllowed {
            message: error.to_string(),
        }
    }
    pub fn generic<E: ToString>(error: E) -> Self {
        Error::Generic {
            message: error.to_string(),
        }
    }
    pub fn io<E: ToString>(error: E) -> Self {
        Error::IO {
            message: error.to_string(),
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
