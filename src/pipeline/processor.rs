use crate::{Configurable, LogHandler, config::{ConfigTrait, pipe_config::LogConfig}, entity::record::LogRecord, sinks::sink};
use std::sync::{Arc, OnceLock, RwLock};

static INSTANCE: OnceLock<Arc<RwLock<LogProcessor>>> = OnceLock::new();

pub struct LogProcessor {
    config: LogConfig,
}

impl Configurable for LogProcessor {
    fn new() -> Arc<RwLock<Self>> where Self: Sized {
        INSTANCE.get_or_init(|| {
            Arc::new(RwLock::new(Self::create_log_processor()))
        }).clone()
    }

    fn reload() {
        if let Some(instance) = INSTANCE.get(){
            let new_log_processor = Self::create_log_processor();
            if let Ok(mut processor) = instance.write(){
                *processor = new_log_processor;
                println!("The log processor config has updated");
            }

        }
    }
}

impl LogHandler for LogProcessor {
    fn handle(&self, record: &LogRecord) -> Result<(), std::io::Error> {
        if record.level < self.config.level {
            return Ok(());
        }
        for sink_type in &self.config.sink_type{
            sink(&sink_type, record).unwrap_or_else(|err| {
                println!("The {:?} has err {}", sink_type, err);
            })
        }
        Ok(())
    }
}

impl LogProcessor {

    fn create_log_processor() -> LogProcessor{
        let config =  LogConfig::load().unwrap_or_else(|err| {
            println!("无法加载日志核心配置文件: {}，将使用默认配置。", err);
            LogConfig::default()
        });
        LogProcessor { config }
    }
}
