use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environments {
    LOCAL,
    TESTING,
    STAGING,
    PRODUCTION,
}

impl Environments {
    fn is_debug(&self) -> bool {
        self == &Environments::LOCAL
            || self == &Environments::STAGING
            || self == &Environments::TESTING
    }

    fn is_deployed(&self) -> bool {
        self == &Environments::STAGING || self == &Environments::PRODUCTION
    }

    fn is_testing(&self) -> bool {
        self == &Environments::TESTING
    }
}

impl FromStr for Environments {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "LOCAL" => Ok(Environments::LOCAL),
            "TESTING" => Ok(Environments::TESTING),
            "STAGING" => Ok(Environments::STAGING),
            _ => Ok(Environments::PRODUCTION),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn from_str_for_environments_when_wrong_string_should_return_production() {
        let env = Environments::from_str("foobar");
        assert_eq!(env, Ok(Environments::PRODUCTION));
    }

    #[tokio::test]
    async fn from_str_for_environments_when_correct_string_should_return_environment() {
        let env = Environments::from_str("LOCAL");
        assert_eq!(env, Ok(Environments::LOCAL));

        let env = Environments::from_str("TESTING");
        assert_eq!(env, Ok(Environments::TESTING));

        let env = Environments::from_str("STAGING");
        assert_eq!(env, Ok(Environments::STAGING));

        let env = Environments::from_str("PRODUCTION");
        assert_eq!(env, Ok(Environments::PRODUCTION));
    }

    #[tokio::test]
    async fn is_debug_when_called_on_local_staging_or_testing_should_return_true() {
        let env = Environments::from_str("TESTING").unwrap();
        assert!(env.is_debug());

        let env = Environments::from_str("STAGING").unwrap();
        assert!(env.is_debug());

        let env = Environments::from_str("TESTING").unwrap();
        assert!(env.is_debug());
    }

    #[tokio::test]
    async fn is_debug_when_called_on_production_should_return_false() {
        let env = Environments::from_str("PRODUCTION").unwrap();
        assert!(!env.is_debug());
    }

    #[tokio::test]
    async fn is_deployed_when_called_on_staging_or_production_should_return_true() {
        let env = Environments::from_str("STAGING").unwrap();
        assert!(env.is_deployed());

        let env = Environments::from_str("PRODUCTION").unwrap();
        assert!(env.is_deployed());
    }

    #[tokio::test]
    async fn is_deployed_when_called_on_local_or_testing_should_return_false() {
        let env = Environments::from_str("LOCAL").unwrap();
        assert!(!env.is_deployed());

        let env = Environments::from_str("TESTING").unwrap();
        assert!(!env.is_deployed());
    }

    #[tokio::test]
    async fn is_testing_when_called_on_testing_should_return_true() {
        let env = Environments::from_str("TESTING").unwrap();
        assert!(env.is_testing());
    }

    #[tokio::test]
    async fn is_testing_when_called_on_local_staging_or_production_should_return_false() {
        let env = Environments::from_str("LOCAL").unwrap();
        assert!(!env.is_testing());

        let env = Environments::from_str("STAGING").unwrap();
        assert!(!env.is_testing());

        let env = Environments::from_str("PRODUCTION").unwrap();
        assert!(!env.is_testing());
    }
}
