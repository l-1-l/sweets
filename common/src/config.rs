mod postgres_config;
mod rabbit_config;
mod redis_config;
mod rust_env;
mod secrets;
mod service;
mod sms;

pub use postgres_config::PostgresConfig;
pub use redis_config::RedisConfig;
pub use rust_env::RustEnv;
pub use secrets::SecretsConfig;
pub use service::ServiceConfig;
pub use sms::UniSMSConfig;

use dotenv::dotenv;
use once_cell::sync::OnceCell;

use crate::config::rabbit_config::RabbitConfig;

static CACHED_CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Config {
    service: ServiceConfig,
    uni_sms: UniSMSConfig,
    postgres: PostgresConfig,
    redis: RedisConfig,
    rabbit: RabbitConfig,
    rust_env: RustEnv,
    secrets: SecretsConfig,
}

impl<'a> Config {
    fn init() -> Self {
        use crate::env_utils::or;
        use std::env::var;

        dotenv().ok();
        // default config
        let srv = ServiceConfig::default();
        let pg = PostgresConfig::default();

        Config {
            service: ServiceConfig {
                bind: or("BIND", srv.bind),
                port: or("PORT", srv.port),
            },
            uni_sms: UniSMSConfig {
                key: var("SMS_UNI_KEY").expect("SMS_UNI_KEY is required"),
                secret: var("SMS_UNI_SECRET")
                    .expect("SMS_UNI_SECRET is required"),
                sign: var("SMS_UNI_SIGN_NAME")
                    .expect("SMS_UNI_SIGN is required"),
            },
            postgres: PostgresConfig {
                host: or("POSTGRES_HOST", pg.host),
                port: or("POSTGRES_PORT", pg.port),
                user: or("POSTGRES_USER", pg.user),
                password: or("POSTGRES_PASSWORD", pg.password),
                name: or("POSTGRES_DB", pg.name),
                use_ssl: or("POSTGRES_USE_SSL", false),
            },
            redis: RedisConfig {
                name: var("REDIS_NAME").ok(),
                host: var("REDIS_HOST").ok(),
                user: var("REDIS_USER").ok(),
                password: var("REDIS_PASSWORD").ok(),
                port: var("REDIS_PORT").ok().map(|x| {
                    x.parse::<u16>().expect("REDIS_PORT must be a number")
                }),
            },
            rabbit: RabbitConfig {
                url: var("RABBIT_URL").expect("RABBIT_URL is required"),
            },
            rust_env: or("RUST_ENV", RustEnv::default()),
            secrets: SecretsConfig::new(
                var("JWT_SECRET").expect("JWT_SECRET is required"),
                or("COMPANY", "".to_owned()),
                or("ACCESS_TOKEN_EXPIRES_IN", 60 * 60 * 24 * 30),
            ),
        }
    }

    pub fn get() -> &'a Config {
        CACHED_CONFIG.get_or_init(Config::init)
    }

    pub fn service(&self) -> &ServiceConfig {
        &self.service
    }
    pub fn uni_sms(&self) -> &UniSMSConfig {
        &self.uni_sms
    }
    pub fn postgres(&self) -> &PostgresConfig {
        &self.postgres
    }
    pub fn rust_env(&self) -> &RustEnv {
        &self.rust_env
    }
    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }
    pub fn rabbit(&self) -> &RabbitConfig {
        &self.rabbit
    }
    pub fn secrets(&self) -> &SecretsConfig {
        &self.secrets
    }
}
