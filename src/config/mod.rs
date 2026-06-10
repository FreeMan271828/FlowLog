use config::ConfigError;

pub mod config_watcher;
pub mod pipe_config;
pub mod file_config;
pub mod console_config;
pub mod s3_config;
pub mod s3_sink_config;

pub trait ConfigTrait : Default{
    fn load() -> Result<Self, ConfigError> where Self: Sized;
}