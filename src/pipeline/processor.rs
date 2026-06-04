use crate::{Config, core::record::LogRecord, pipeline::config::LogConfig, sinks::sink};
use std::sync::OnceLock;

pub struct LogProcessor {
    config: LogConfig,
}

impl LogProcessor {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<LogProcessor> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            let config =  LogConfig::load().unwrap_or_else(|err| {
                println!("无法加载配置文件: {}，将使用默认配置。", err);
                LogConfig::default()
            });
            LogProcessor { config }
        })
    }

    pub fn process(&self, record: &LogRecord){
        if record.level < self.config.level {
            return;
        }
        println!(
            "[{}] [{:?}] ({}:{}) - {}",
            record.timestamp.format("%m-%d %H:%M:%S%.1f"),
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            record.body
        );
        // for sink_type in &self.config.sink_type{
        //     sink(&sink_type, record);   
        // }
    }
}
