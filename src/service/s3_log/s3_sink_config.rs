use config::{Config, File, ConfigError};
use serde::{Deserialize, Serialize};

use crate::{constants, tools::ConfigTrait};

#[derive(Serialize, Deserialize, Debug)]
pub struct S3SinkConfig{
    pub(crate) put_size : u64,
    pub(crate) put_min_ratio : f64,
}

impl Default for S3SinkConfig {
    fn default() -> Self {
        Self {
            put_size : 0,
            put_min_ratio : 0.5,
        }
    }
}

impl ConfigTrait for S3SinkConfig {
    fn load() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(constants::CONFIG_PATH)
            .required(false))
            .build()?;
        s.try_deserialize()
    }
}