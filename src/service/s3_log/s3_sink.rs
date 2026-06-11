use std::{fs, io};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::tools::ConfigTrait;
use crate::{Configurable, LogHandler};
use crate::entity::record::LogRecord;
use crate::service::s3_log::s3_sink_config::S3SinkConfig;
use crate::tools::s3::s3_tools::S3Client;
use crate::tools::file::file_tools::*;


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
    s3_client : Arc<RwLock<S3Client>>,
}

impl Configurable for S3Sink {
    fn new() -> Arc<RwLock<Self>> where Self: Sized {
        Self::init_tmp_file();
        INSTANCE.get_or_init(|| {
            let s3_sink = Self::create_s3_sink();
            let ins = Arc::new(RwLock::new(s3_sink));
            ins
        }).clone()
    }

    /// 配置文件的重载，见3
    /// 如果当前临时日志文件大小大于旧的配置文件的阈值（文件大小*比率），直接上传并创建新的文件
    #[allow(unused)]
    fn reload() {
        if let Some(instance) = INSTANCE.get(){
            if let Some(mut file_path) = Self::parse_file_path().unwrap(){
                let threasold = 
                    instance.read().unwrap().config.put_size as f64 * 
                    instance.read().unwrap().config.put_min_ratio;
                if get_file_size(&file_path).unwrap() as f64 > threasold {
                    let sink = &*instance.read().expect("Failed to acquire read lock");
                    let client = &*sink.s3_client.read().expect("Failed to acquire read lock");
                    client.put_file(&file_path.to_string_lossy()).unwrap();
                    file_path = PathBuf::from(Self::update_tmp_file_name().unwrap());
                }
            }
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
        let client = self.s3_client.read().expect("Failed to acquire read lock");
        let mut file_path = Self::parse_file_path()?.unwrap();
        // 判断是否需要更新
        let need_update = {
            let lock = TMP_FILE.get_or_init(|| RwLock::new(String::new()));
            let guard = lock.read().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to acquire lock")
            })?;
            if guard.is_empty() {
                eprintln!("The tmp file is not existed");
                return Ok(());
            }
            let file_path = PathBuf::from(&*guard);
            get_file_size(&file_path)? > self.config.put_size
        }; 
        if need_update{
            let _ = client.put_file(&file_path.to_string_lossy());
            file_path = PathBuf::from(Self::update_tmp_file_name().unwrap());
        }
        let _ = write_to_file(&file_path, record, true);
        Ok(())
    }
}

impl S3Sink {

    /// 从TMP_FILE中读取文件路径，由于只占用读锁，所以安全
    fn parse_file_path() -> Result<Option<PathBuf>, io::Error>{
        let lock = TMP_FILE.get_or_init(|| RwLock::new(String::new()));
            let guard = lock.read().map_err(|_| {
                std::io::Error::new(std::io::ErrorKind::Other, "Failed to acquire lock")
            })?;
            if guard.is_empty() {
                eprintln!("The tmp file is not existed");
                return Ok(None);
            }
            Ok(Some(PathBuf::from(&*guard)))
    }

    /// 初始化临时文件：检查是否存在旧的临时文件
    fn init_tmp_file() {
        let lock = TMP_FILE.get_or_init(|| RwLock::new(String::new()));
        if let Ok(mut guard) = lock.write() {
            // 如果已经有值，说明已经初始化过了，直接返回
            if !guard.is_empty() {
                return;
            }
            // 查找现有的临时文件
            if let Some(existing_file) = Self::find_existing_tmp_file() {
                *guard = existing_file;
                println!("Found existing tmp file: {}", guard);
            } else {
                let new_file = Self::generate_tmp_filename();
                // 创建物理文件
                if let Ok(_) = fs::File::create(&new_file) {
                    println!("Created new tmp file: {}", new_file);
                }
                *guard = new_file;
            }
        }
    }
    
    /// 查找现有的临时文件
    fn find_existing_tmp_file() -> Option<String> {
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();
                if file_name_str.starts_with(".tmp_log_") && file_name_str.ends_with(".log") {
                    // 找到第一个匹配的临时文件就返回
                    return Some(file_name_str.to_string());
                }
            }
        }
        None
    }

    /// 创建或更新临时文件名称，
    /// 如果文件名称不为空（有旧文件），则需要删除旧文件，
    /// !!!需要持有TMP_FILE的写锁，调用前注意释放读锁!!!
    fn update_tmp_file_name() -> Result<String, std::io::Error>{
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
        let new_filename = Self::generate_tmp_filename();
        println!("new_filename: {}", new_filename);
        fs::File::create(&new_filename)?;
        *guard = new_filename.clone();
        Ok(new_filename)
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
        S3Sink { 
            config, 
            s3_client: S3Client::new(),
        }
    }
}

#[cfg(test)]
mod test{
use std::{thread, time::Duration};
use super::*;

    #[test]
    fn test_handle(){
        let s3_sink = S3Sink::new();
        // 创建1M大小的内容
        let large_body = "A".repeat(1024 * 1024); // 1MB
        let record = LogRecord::new(crate::LogLevel::Dbg, &large_body);
        let s3_sink = &*s3_sink.read().unwrap();
        println!("Writing records, each record is 1MB");
        let mut index = 1;
        loop {
            let _= s3_sink.handle(&record);  
            index = index + 1;
            if index % 5 == 0 {
                println!("Written {} records", index);
            } 
            thread::sleep(Duration::from_secs(2));
        }
    }
}