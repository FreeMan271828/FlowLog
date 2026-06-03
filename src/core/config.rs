use crate::core::level::LogLevel;
use config::{Config, ConfigError, File};
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct LogConfig {
    #[serde(default = "default_level")]
    pub level: LogLevel,
    
    #[serde(default = "default_async")]
    pub enable_async: bool,
    
    #[serde(default = "default_buffer")]
    pub buffer_size: usize,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: LogLevel::Log,
            enable_async: false,
            buffer_size: 1024,
        }
    }
}

#[allow(dead_code)]
impl LogConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/log").required(false))
            .build()?;

        s.try_deserialize()
    }
}

/// 获取默认日志等级
fn default_level() -> LogLevel { LogLevel::Log }
fn default_async() -> bool { false }
fn default_buffer() -> usize { 1024 }