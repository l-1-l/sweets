use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, SslMode};

#[derive(Debug, Clone)]
pub struct PostgresConfig {
    pub host: String,

    pub port: u16,
    pub user: String,
    pub password: String,
    pub name: String,
    pub use_ssl: bool,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        PostgresConfig {
            host: "localhost".into(),
            port: 5432,
            user: "uname".into(),
            password: "password".into(),
            name: "sweets".into(),
            use_ssl: false,
        }
    }
}

impl PostgresConfig {
    pub fn url(&self) -> String {
        format!(
            "postgres://{ua}:{pwd}@{host}/{db}",
            ua = self.user,
            pwd = self.password,
            host = self.host,
            db = self.name
        )
    }

    pub fn without_db(&self) -> Config {
        let mut cfg = Config::new();

        let ssl_mode = if self.use_ssl {
            SslMode::Require
        } else {
            SslMode::Prefer
        };

        cfg.ssl_mode = Some(ssl_mode);
        cfg.host = Some(self.host.clone());
        cfg.port = Some(self.port);
        cfg.user = Some(self.user.clone());
        cfg.password = Some(self.password.clone());

        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        cfg
    }

    pub fn with_db(&self) -> Config {
        let mut cfg = self.without_db();
        cfg.dbname = Some(self.name.clone());
        cfg
    }
}
