use crate::{config::{ConfigTrait, pipe_config::LogConfig}, entity::record::LogRecord, sinks::sink};
use std::sync::OnceLock;

pub struct LogProcessor {
    config: LogConfig,
}

impl LogProcessor {
    pub fn new() -> &'static Self {
        static INSTANCE: OnceLock<LogProcessor> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let config =  LogConfig::load().unwrap_or_else(|err| {
                println!("无法加载日志核心配置文件: {}，将使用默认配置。", err);
                LogConfig::default()
            });
            LogProcessor { config }
        })
    }

    pub fn process(&self, record: &LogRecord){
        if record.level < self.config.level {
            return;
        }
        for sink_type in &self.config.sink_type{
            sink(&sink_type, record).unwrap_or_else(|err| {
                println!("The {:?} has err {}", sink_type, err);
            })
        }
    }
}
