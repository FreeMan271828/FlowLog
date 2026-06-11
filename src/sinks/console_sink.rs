use std::sync::{Arc, OnceLock, RwLock};

use crate::{Configurable, LogHandler, entity::record::LogRecord};

pub struct ConsoleSink;

static INSTANCE: OnceLock<Arc<RwLock<ConsoleSink>>> = OnceLock::new();

impl Configurable for ConsoleSink {
    fn new() -> Arc<RwLock<Self>> where Self: Sized {
        INSTANCE.get_or_init(|| {
            Arc::new(RwLock::new(ConsoleSink{}))
        }).clone()
    }

    fn reload() {
        println!("Console has no config, not need to update") 
    }
}

impl LogHandler for ConsoleSink{
    fn handle(&self, record: &LogRecord) -> Result<(), std::io::Error> {
        let body = &record.body;
        let max_chars = 20; // 设置最大显示长度
         let display_body = if body.chars().count() > max_chars {
            format!("{}...", body.chars().take(max_chars).collect::<String>())
        } else {
            body.to_string()
        };
        println!(
            "[{}] [{:?}] ({}:{}) - {}",
            record.timestamp.format("%H:%M:%S%.3f"),
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            display_body,
        );
        Ok(())
    }
}