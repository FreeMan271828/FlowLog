use std::{fs, path::PathBuf};
use std::fs::create_dir_all;
use std::io::Write;
use std::sync::{OnceLock};
use chrono::Local;
use config::{Config, File};
use serde::Deserialize;

use crate::constants;
use crate::Config as MyConfig;
use crate::core::record::LogRecord;
use crate::sinks::{Sink};

/// file_path 保留日志文件夹
/// max_size 日志超过这个大小创建新的文件
/// rotate_num 最多保留多少日志文件
#[allow(dead_code)]
#[derive(Deserialize)]
pub struct FileConfig{
    dir_path: PathBuf,
    max_size: Option<u64>,
    rotate_num: Option<usize>,
}

impl Default for FileConfig {
    fn default() -> Self{
        FileConfig { 
            dir_path: PathBuf::from("log"),
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

/// 文件保存逻辑
/// 1. 获取最新的文件，判断当前文件是否超额
/// 2. 如果超额，则创建新文件，采用log-YY-mm-DD形式，并判断文件个数是否超过预设
/// 3. 如果超过，则删除最旧的文件
pub struct FileSink {
    config: FileConfig,
}

impl Sink for FileSink {

    fn redirect(&self, record: &LogRecord) -> Result<(), std::io::Error> {
        let FileConfig { dir_path, max_size, rotate_num } = &self.config;
        let mut file = Self::choose_file(dir_path, max_size.unwrap(), rotate_num.unwrap())?;
        let bytes = record.as_bytes();
        file.write_all(bytes.as_slice()).expect("Write File Err");
        file.flush().expect("Flush File Err");
        Ok(())
    }

    fn new() -> &'static Self {
        static INSTANCE: OnceLock<FileSink> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let config = FileConfig::load().unwrap_or_else(|e| {
                println!("无法加载日志配置文件: {}，将使用默认配置。", e);
                FileConfig::default()
            });
            FileSink{config}
        })
    }
}

impl FileSink {

    fn choose_file(dir_path: &PathBuf, max_size: u64, rotate_num: usize) -> Result<fs::File, std::io::Error> {
        if !dir_path.exists() {
            create_dir_all(dir_path).expect("Create dir Err");
        }
        if let Some(newest_file) = Self::get_newest_file(dir_path)? {
            let meta = fs::metadata(&newest_file)?;
            if meta.len() < max_size {
                return fs::OpenOptions::new().append(true).open(newest_file);
            }
            let file = Self::create_file(dir_path)?;
            let current_count = fs::read_dir(dir_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with("log-"))
                .count();
            if current_count > rotate_num {
                // 删除上面获取到的、排序最靠前的最旧文件
                if let Some(oldest) = Self::get_oldest_file(dir_path)? {
                    fs::remove_file(oldest)?;
                }
            }
            Ok(file)
        }
        // 文件夹中没有文件，创建
        else{
            Self::create_file(dir_path)
        }
    }

    fn create_file(dir_path: &PathBuf) -> Result<fs::File, std::io::Error> {
        let today_str = Local::now().format("%Y-%m-%d").to_string();
        let first_path = dir_path.join(format!("log-{}.log", today_str));
        Ok(fs::File::create(first_path)?)
    }

    fn get_oldest_file(dir_path: &PathBuf) -> Result<Option<PathBuf>, std::io::Error> {
        let files: Vec<PathBuf> = Self::get_sorted_files(dir_path)?;
        if files.is_empty() {
            Ok(None)
        } else {
            let oldest = files.first().unwrap().clone();
            Ok(Some(oldest))
        }
    }

    fn get_newest_file(dir_path: &PathBuf) -> Result<Option<PathBuf>, std::io::Error> {
        let files: Vec<PathBuf> = Self::get_sorted_files(dir_path)?;
        if files.is_empty() {
            Ok(None)
        } else {
            let oldest = files.last().unwrap().clone();
            Ok(Some(oldest))
        }
    }

    fn get_sorted_files(dir_path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut files: Vec<PathBuf> = dir_path.read_dir()?
            .filter_map(|entry|  {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    if let Some(file_name) = path.file_name()?.to_str() {
                        if file_name.starts_with("log-") {
                            return Some(path);
                        }
                    }
                }
                None
            }
            )
            .collect();
        files.sort();
        Ok(files)
    }

}