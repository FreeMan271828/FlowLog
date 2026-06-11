use config::ConfigError;

pub mod s3;
pub mod file;

pub trait ConfigTrait : Default{
    fn load() -> Result<Self, ConfigError> where Self: Sized;
}