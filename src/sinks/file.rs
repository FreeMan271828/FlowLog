use std::{path::PathBuf};

use config::{Config, File};
use serde::Deserialize;

use crate::constants;

/// max_size 日志超过这个大小创建新的文件
/// rotate_num 最多保留多少日志文件
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FileConfig{
    file_path: PathBuf,
    max_size: Option<u64>,
    rotate_num: Option<u8>,
}

impl Default for FileConfig {
    fn default() -> Self{
        FileConfig { 
            file_path: PathBuf::from("log"),
            max_size: Some(10485760), // 10MB
            rotate_num: Some(5),
        }
    }
}

impl crate::Config for FileConfig {
    fn load() -> Result<Self, config::ConfigError> where Self: Sized {
        let s = Config::builder()
            .add_source(File::with_name(constants::CONFIG_PATH).required(false))
            .build()?;
        s.try_deserialize()
    }
}