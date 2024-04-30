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
    GenericError { message: String },

    // IO
    IOError { message: String },

    // Runtime
    RunTimeError { message: String },

    // HTTP Errors
    NotAllowed,
    NotFound,
    BadRequest { detail: String },
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::NotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::BadRequest { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::to_string(&self).unwrap_or_default();

        (status, body).into_response()
    }
}
