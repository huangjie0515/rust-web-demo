use std::sync::Arc;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Conf {
    pub server: Server,
    pub mysql: Mysql,
    pub redis: Redis,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: u32,
    pub log: String,
}

#[derive(Debug, Deserialize)]
pub struct Mysql {
    pub dsn: String,
    pub min: u32,
    pub max: u32,
}

#[derive(Debug, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: u32,
    pub min: u64,
    pub max: u64,
}

pub fn global() -> &'static Arc<Conf> {
    static CONFIG: OnceCell<Arc<Conf>> = OnceCell::new();
    CONFIG.get_or_init(|| {
        let s = std::fs::read_to_string(&"config.yaml").unwrap();
        Arc::new(serde_yaml::from_str(&s).unwrap())
    })
}

impl Conf {
    pub fn addr(&self) -> String {
        format!("0.0.0.0:{}", self.server.port)
    }

    pub fn redis_addr(&self) -> String {
        format!("redis://{}:{}", self.redis.host, self.redis.port)
    }
}
