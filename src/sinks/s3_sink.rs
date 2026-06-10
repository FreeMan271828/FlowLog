use std::{fs};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::config::s3_sink_config::S3SinkConfig;
use crate::entity::record::LogRecord;
use crate::tools::{file_tools, s3_tools};
use crate::{Configurable, LogHandler, config::{ConfigTrait}};

/// S3存储采用双磁盘异步写入
/// 1. 在当前文件夹下使用隐形文件.tmp_log_a，持续写入，如果文件大小超过阈值，那么就创建.tmp_log_b (a和b都是有时间顺序的随机数)，然后把之前的文件后台上传到S3，并删除
/// 2. 上传S3和删除旧文件的过程采用后台线程执行，受锁保护，不会受到配置文件更新的影响
/// 3. 若写入的时候终止，若大小已经达到预设的百分比，则直接上传，否则不上传，恢复后继续写入文件

/// 当前使用的临时文件
static TMP_FILE: OnceLock<RwLock<String>> =  OnceLock::new();
/// S3服务实例
static INSTANCE: OnceLock<Arc<RwLock<S3Sink>>> = OnceLock::new();

#[allow(dead_code)]
pub struct S3Sink{
    config: S3SinkConfig,
    s3_client : aws_sdk_s3::Client,
}

impl Configurable for S3Sink {
    fn new() -> Arc<RwLock<Self>> where Self: Sized {
        INSTANCE.get_or_init(|| {
            let s3_sink = Self::create_s3_sink();
            let ins = Arc::new(RwLock::new(s3_sink));
            ins
        }).clone()
    }

    /// 配置文件的重载，见3
    fn reload() {
        if let Some(instance) = INSTANCE.get(){
            let new_sink = Self::create_s3_sink();
            if let Ok(mut sink) = instance.write() {
                *sink = new_sink;
                println!("The s3 config has updated");
            }
        }
    }
}

impl LogHandler for S3Sink {
    fn handle(&self, record: &LogRecord) -> Result<(), std::io::Error> {
        // 获取文件大小
        let lock = TMP_FILE.get_or_init(|| RwLock::new(String::new()));
        let guard = lock.read().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to acquire lock")
        })?;
        if guard.is_empty() {
            println!("The tmp file is not existed");
            return Ok(());
        }
        let file_path = PathBuf::from(&*guard);
        let size = file_tools::get_file_size(&file_path)?;
        if size > self.config.update_size {
            
        }
        // 加锁，上传旧文件

        // 更改文件名称，解锁

        // 写入文件record
        Ok(())
    }
}

impl S3Sink {
    /// 创建或更新临时文件名称
    /// 如果文件名称不为空（有旧文件），则需要删除旧文件
    fn update_tmp_file_name() -> Result<(), std::io::Error>{
        let lock = TMP_FILE.get_or_init(|| RwLock::new(String::new()));
        let mut guard = lock.write().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to acquire lock")
        })?;
        if !guard.is_empty() {
        let old_path = Path::new(&*guard);
            if old_path.exists() {
                fs::remove_file(old_path)?;
            }
        }
        *guard = Self::generate_tmp_filename();
        Ok(())
    }

    fn generate_tmp_filename() -> String {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        format!(".tmp_log_{}.log", timestamp)
    }

    fn create_s3_sink() -> S3Sink{
        let config = S3SinkConfig::load().unwrap_or_else(|_| {
            S3SinkConfig::default()
        });
        let binding = s3_tools::S3Client::new();
        let s3_tools = binding.read().expect("Failed to acquire read lock");
        S3Sink { 
            config, 
            s3_client: s3_tools.client.clone() 
        }
    }
}