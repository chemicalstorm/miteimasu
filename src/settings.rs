use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Miteimasu {
    pub log: Log,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub miteimasu: Miteimasu,
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
