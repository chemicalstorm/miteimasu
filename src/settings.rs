use std::net::IpAddr;

use config::{Config, ConfigError};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Deserialize)]
pub struct Miteimasu {
    pub log: Log,
    pub server: Server,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub port: u16,
    pub address: IpAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub name: String,
    pub url: Url,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub miteimasu: Miteimasu,
    pub cameras: Vec<Camera>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "miteimasu" configuration file
        s.merge(config::File::with_name("miteimasu"))?;

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
