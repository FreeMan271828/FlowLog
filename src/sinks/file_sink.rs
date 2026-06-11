use std::{fs, path::PathBuf};
use std::fs::create_dir_all;
use std::sync::{Arc, OnceLock, RwLock};
use chrono::Local;

use crate::config::ConfigTrait;
use crate::config::file_config::FileConfig;
use crate::{Configurable, LogHandler, constants};
use crate::entity::record::LogRecord;
use crate::tools::*;

/// 文件保存逻辑
/// 1. 获取最新的文件，判断当前文件是否超额
/// 2. 如果超额，则创建新文件，采用log-YY-mm-DD形式，并判断文件个数是否超过预设
/// 3. 如果超过，则删除最旧的文件

static INSTANCE: OnceLock<Arc<RwLock<FileSink>>> = OnceLock::new();
pub struct FileSink {
    config: FileConfig,
}


impl Configurable for FileSink {
    fn new() -> Arc<RwLock<Self>> where Self: Sized  {
        INSTANCE.get_or_init(|| {
            Arc::new(RwLock::new(Self::create_file_sink()))
        }).clone()
    }
    
    fn reload(){
        if let Some(instance) = INSTANCE.get() {
            let new_sink = Self::create_file_sink();
            if let Ok(mut sink) = instance.write() {
                *sink = new_sink;
                println!("The file config has updated");
            }
        }
    }
}

impl LogHandler for FileSink {
    fn handle(&self, record: &LogRecord) -> Result<(), std::io::Error> {
        let FileConfig { dir_path, max_size, rotate_num } = &self.config;
        if dir_path.to_string_lossy().is_empty() || max_size.is_none() || rotate_num.is_none() {
            eprintln!("Something err while handling file, you can choose to remove File option");
            return Ok(());
        }
        let file = Self::choose_file(dir_path, max_size.unwrap(), rotate_num.unwrap())?;
        file_tools::write_to_file_(&file, record)
    }
}

impl FileSink {

    fn create_file_sink() -> FileSink{
        let mut config = FileConfig::load().unwrap_or_else(|_| {
            FileConfig::default()
        });
        
        let min_config = constants::FILE_CONFIG_MIN;
        if config.max_size < Some(min_config.0) || config.rotate_num < Some(min_config.1) {
            eprintln!("日志配置小于最低要求：512KB，1个轮转文件");
            config = FileConfig::default();
        }
        
        let FileConfig { dir_path, max_size, rotate_num } = &config;
        if dir_path.to_string_lossy().is_empty() || max_size.is_none() || rotate_num.is_none() {
            eprintln!("无法加载文件日志配置，将自动禁用");
        }
        
        FileSink { config }
    }

    fn choose_file(dir_path: &PathBuf, max_size: u64, rotate_num: usize) -> Result<fs::File, std::io::Error> {
        if !dir_path.exists() {
            create_dir_all(dir_path).expect("Create dir Err");
        }
        if let Some(newest_file) = file_tools::get_newest_file(dir_path)? {
            if file_tools::get_file_size(&newest_file)? < max_size {
                return fs::OpenOptions::new().append(true).open(newest_file);
            }
            let file = Self::create_log_file(dir_path)?;
            let current_count = fs::read_dir(dir_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().starts_with("log-"))
                .count();
            if current_count > rotate_num {
                // 删除上面获取到的、排序最靠前的最旧文件
                if let Some(oldest) = file_tools::get_oldest_file(dir_path)? {
                    fs::remove_file(oldest)?;
                }
            }
            Ok(file)
        }
        // 文件夹中没有文件，创建
        else{
            Self::create_log_file(dir_path)
        }
    }

    fn create_log_file(dir_path: &PathBuf) -> Result<fs::File, std::io::Error> {
        let today_str = Local::now().format("%Y-%m-%d").to_string();
        let first_path = dir_path.join(format!("log-{}.log", today_str));
        Ok(fs::File::create(first_path)?)
    }
}