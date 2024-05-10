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
    Config { detail: String },
    MultiPurposeUrl { detail: String },
    Schema { detail: String },
    Logging { detail: String },

    // IO
    IO { detail: String },
    Pool { detail: String },

    // HTTP Errors
    NotFound { detail: String },
    InternalServer,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<tracing::subscriber::SetGlobalDefaultError> for Error {
    fn from(value: tracing::subscriber::SetGlobalDefaultError) -> Self {
        Error::Logging {
            detail: value.to_string(),
        }
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(value: validator::ValidationErrors) -> Self {
        Error::Config {
            detail: value.to_string(),
        }
    }
}

impl From<envconfig::Error> for Error {
    fn from(value: envconfig::Error) -> Self {
        Error::Config {
            detail: value.to_string(),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::PoolClosed => Error::Pool {
                detail: value.to_string(),
            },
            sqlx::Error::PoolTimedOut => Error::Pool {
                detail: value.to_string(),
            },
            sqlx::Error::RowNotFound => Error::NotFound {
                detail: "Vehicle with the specified ID was not found.".to_string(),
            },
            _ => Error::InternalServer,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IO {
            detail: value.to_string(),
        }
    }
}

impl From<url::ParseError> for Error {
    fn from(value: url::ParseError) -> Self {
        Error::MultiPurposeUrl {
            detail: value.to_string(),
        }
    }
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::NotFound { .. } => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::to_string(&self).unwrap_or_default();

        (status, body).into_response()
    }
}
