use config::{Config, File, ConfigError};
use serde::{Deserialize, Serialize};

use crate::{config::ConfigTrait, constants};

#[derive(Serialize, Deserialize, Debug)]
pub struct S3SinkConfig{
    pub(crate) update_size : u64,
    pub(crate) update_min_ratio : f64,
}

impl Default for S3SinkConfig {
    fn default() -> Self {
        Self {
            update_size : 0,
            update_min_ratio : 0.5,
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