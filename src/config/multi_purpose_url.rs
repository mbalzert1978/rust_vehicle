use std::str::FromStr;

use url::Url;

use crate::{prelude::*, strings};

#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub(crate) struct PostgresDsn(Url);

use std::ops::{Deref, DerefMut};

impl Deref for PostgresDsn {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PostgresDsn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for PostgresDsn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let url = Url::parse(s)?;
        if is_postgres_scheme(&url) {
            return Ok(PostgresDsn(url));
        }
        Err(Error::Schema {
            detail: format!("{}[{}]", strings::en::INVALID_DSN_SCHEME, url.scheme()),
        })
    }
}

fn is_postgres_scheme(url: &Url) -> bool {
    ["postgres", "postgresql"].contains(&url.scheme())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_str_for_postgres_dsn_when_wrong_schema_should_return_error() {
        let dsn = PostgresDsn::from_str("http://user:pass@localhost:5432/foobar");
        match dsn {
            Err(Error::Schema { .. }) => (),
            _ => panic!("FAIL: Error should be Schema."),
        }
    }

    #[tokio::test]
    async fn from_str_for_postgres_dsn_when_correct_schema_should_return_dsn() {
        let dsn = PostgresDsn::from_str("postgres://user:pass@localhost:5432/foobar")
            .expect("Should not fail here.");

        assert_eq!(dsn.scheme(), "postgres");
        assert_eq!(dsn.username(), "user");
        assert_eq!(dsn.host_str(), Some("localhost"));
        assert_eq!(dsn.port(), Some(5432));
        assert_eq!(dsn.password(), Some("pass"));
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_with_lowercase_should_return_true() {
        let url = Url::parse("postgres://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_with_uppercase_should_return_true() {
        let url = Url::parse("POSTGRES://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_with_mixedcase_should_return_true() {
        let url = Url::parse("Postgres://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }
    #[tokio::test]
    async fn is_postgres_scheme_when_called_with_invalid_scheme_should_return_false() {
        let url = Url::parse("foo://localhost").unwrap();
        assert!(!is_postgres_scheme(&url), "FAIL: should return false.");
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_postgresql_with_lowercase_should_return_true() {
        let url = Url::parse("postgresql://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_postgresql_with_uppercase_should_return_true() {
        let url = Url::parse("POSTGRESQL://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }

    #[tokio::test]
    async fn is_postgres_scheme_when_called_postgresql_with_mixedcase_should_return_true() {
        let url = Url::parse("PostgresQl://localhost").unwrap();
        assert!(is_postgres_scheme(&url), "FAIL: should return true.");
    }
    #[tokio::test]
    async fn is_postgres_scheme_when_called_postgresql_with_invalid_scheme_should_return_false() {
        let url = Url::parse("foo://localhost").unwrap();
        assert!(!is_postgres_scheme(&url), "FAIL: should return false.");
    }
}
