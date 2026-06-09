use std::sync::{Arc, OnceLock, RwLock};

use crate::{Configurable, LogHandler, config::console_config::ConsoleConfig, entity::record::LogRecord};

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
        println!(
            "[{}] [{:?}] ({}:{}) - {}",
            record.timestamp.format("%H:%M:%S%.3f"),
            record.level,
            record.file.unwrap_or("unknown"),
            record.line.unwrap_or(0),
            record.body
        );
        Ok(())
    }
}