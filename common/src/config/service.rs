use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub bind: Ipv4Addr,
    pub port: u16,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            bind: Ipv4Addr::new(127, 0, 0, 1),
            port: 8000,
        }
    }
}
