use std::str::FromStr;

use url::Url;

use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PostgresDsn(Url);

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
        let url = Url::parse(s).map_err(|e| Error::GenericError {
            message: e.to_string(),
        })?;
        if !vec!["postgres", "postgresql"].contains(&url.scheme()) {
            return Err(Error::GenericError {
                message: format!("invalid scheme dsn: [{}]", url.scheme()),
            });
        }
        Ok(PostgresDsn(url))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_for_postgres_dsn_when_wrong_schema_should_return_error() {
        let dsn = PostgresDsn::from_str("http://user:pass@localhost:5432/foobar");
        assert_eq!(
            dsn,
            Err(Error::GenericError {
                message: "invalid scheme dsn: [http]".to_string()
            })
        );
    }

    #[test]
    fn from_str_for_postgres_dsn_when_correct_schema_should_return_dsn() {
        let dsn = PostgresDsn::from_str("postgres://user:pass@localhost:5432/foobar")
            .expect("Should not fail here.");

        assert_eq!(dsn.scheme(), "postgres");
        assert_eq!(dsn.username(), "user");
        assert_eq!(dsn.host_str(), Some("localhost"));
        assert_eq!(dsn.port(), Some(5432));
        assert_eq!(dsn.password(), Some("pass"));
    }
    
}
