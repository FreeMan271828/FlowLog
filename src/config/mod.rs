use config::ConfigError;

pub mod pipe_config;
pub mod file_config;

pub trait ConfigTrait : Default{
    fn load() -> Result<Self, ConfigError> where Self: Sized;
}