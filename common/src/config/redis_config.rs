use deadpool_redis::Config;
use serde::Deserialize;
// use serde_aux::field_attributes::deserialize_option_number_from_string;

#[derive(Debug, Deserialize, Clone)]
pub struct RedisConfig {
    pub name: Option<String>,
    pub host: Option<String>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
}

impl RedisConfig {
    pub fn url(&self) -> String {
        // redis://[user[:PASSWORD@]]HOST[:PORT][/DATABASE]
        let mut str = String::from("redis://");

        if let Some(user) = &self.user {
            str.push_str(user);
            str.push(':');
        }

        if let Some(password) = &self.password {
            str.push_str(password);
            str.push('@');
        }

        let host = self.host.as_deref().unwrap_or("127.0.0.1");
        str.push_str(host);

        let port = self.port.unwrap_or(6379).to_string();
        str.push(':');
        str.push_str(&port);

        if let Some(db_name) = &self.name {
            str.push('/');
            str.push_str(db_name);
        }

        str
    }
    pub fn create_config(&self) -> Config {
        Config::from_url(self.url())
    }
}
