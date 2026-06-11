use std::path::PathBuf;

use config::{Config, File};
use serde::Deserialize;

use crate::{constants, tools::ConfigTrait};

/// file_path 保留日志文件夹
/// max_size 日志超过这个大小创建新的文件
/// rotate_num 最多保留多少日志文件
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FileConfig{
    pub dir_path: PathBuf,
    pub max_size: Option<u64>,
    pub rotate_num: Option<usize>,
}

impl Default for FileConfig {
    fn default() -> Self{
        FileConfig { 
            dir_path: PathBuf::from(""),
            max_size: None,
            rotate_num: None,
        }
    }
}

impl ConfigTrait for FileConfig {
    fn load() -> Result<Self, config::ConfigError> where Self: Sized {
        let s = Config::builder()
            .add_source(File::with_name(constants::CONFIG_PATH).required(false))
            .build()?;
        s.try_deserialize()
    }
}
