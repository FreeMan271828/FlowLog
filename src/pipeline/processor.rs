use crate::core::config::LogConfig;
use crate::core::record::LogRecord;
use std::sync::OnceLock;

pub struct LogProcessor {
    config: LogConfig,
}

impl LogProcessor {
    /// 获取全局唯一的处理器单例（自动读取 config/log.toml）
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<LogProcessor> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            // 尝试加载 config/log.toml，如果找不到或解析失败，使用 Default 降级
            let config = LogConfig::load().unwrap_or_else(|err| {
                println!("⚠️ 无法加载配置文件: {}，将使用默认配置。", err);
                LogConfig::default()
            });
            LogProcessor { config }
        })
    }

    /// 核心过滤处理逻辑
    pub fn process(&self, record: &LogRecord) {
        // 🎯 关键拦截：由于派生了 PartialOrd 宏，这里会自动进行大小比对
        // 如果当前日志级别低于配置文件里设定的级别（如 Dbg < Log），直接丢弃
        if record.level < self.config.level {
            return;
        }

        // 只有符合级别要求的日志才会走到这里进行打印
        let time_str = record.timestamp.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        println!(
            "[{}] [{:?}] ({}:{}) - {}", // 使用 {:?} 打印枚举名
            time_str,
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            record.body
        );
    }
}
