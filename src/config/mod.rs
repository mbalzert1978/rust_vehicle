use crate::prelude::*;

use envconfig::Envconfig;
use multi_purpose_url::PostgresDsn;
use validator::Validate;

mod multi_purpose_url;

pub fn get_config() -> Result<Config> {
    dotenvy::dotenv().ok();
    Ok(Config::init_from_env()?)
}

#[derive(Envconfig, Clone, Validate)]
pub struct Config {
    #[envconfig(from = "DATABASE_URL")]
    pub database_url: PostgresDsn,

    #[envconfig(from = "SITE_DOMAIN", default = "vehicle_api.test")]
    pub site_domain: String,

    #[envconfig(from = "SITE_NAME", default = "Vehicle API")]
    pub site_name: String,

    #[envconfig(from = "VERSION", default = "0.0.1")]
    pub version: String,

    #[envconfig(from = "DIRECTORY", default = "./logs")]
    pub directory: String,

    #[envconfig(from = "FILE_NAME", default = "app.log")]
    pub file_name: String,

    #[envconfig(from = "MAX_POOL_SIZE", default = "8")]
    #[validate(range(min = 1, max = 10))]
    pub max_pool_size: u32,

    #[envconfig(from = "HOST", default = "0.0.0.0")]
    pub host: String,

    #[envconfig(from = "port", default = "8000")]
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,
}
