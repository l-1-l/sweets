use deadpool_lapin::Config;
use serde::Deserialize;

/// settings for lapin(rabbit).
#[derive(Debug, Clone, Deserialize)]
pub struct RabbitConfig {
    pub url: String,
}

impl RabbitConfig {
    pub fn create_config(&self) -> Config {
        Config {
            url: Some(self.url.clone()),
            ..Default::default()
        }
    }
}
