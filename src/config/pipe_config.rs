use crate::{config::ConfigTrait, constants, entity::level::LogLevel, sinks::SinkType};
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    pub level: LogLevel,
    pub enable_async: bool,
    pub buffer_size: usize,
    pub sink_type: Vec<SinkType>
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Log,
            enable_async: false,
            buffer_size: 1024,
            sink_type: vec![SinkType::Console]
        }
    }
}

impl ConfigTrait for LogConfig {
    fn load() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(constants::CONFIG_PATH)
            .required(false))
            .build()?;
        s.try_deserialize()
    }
}