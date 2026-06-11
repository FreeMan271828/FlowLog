pub mod entity;
pub mod formatters;
pub mod macros;
pub mod constants;
pub mod tools;
pub mod service;
pub mod config_watcher;

use std::{io, sync::{Arc, RwLock}};

pub use crate::entity::level::LogLevel;
use crate::entity::record::LogRecord;

pub trait Configurable {    
    fn new() -> Arc<RwLock<Self>> where Self: Sized;
    fn reload();
}

// 日志处理相关的 trait
pub trait LogHandler {
    fn handle(&self, record: &LogRecord) -> Result<(), io::Error>;
    fn redirect(&self, record: &LogRecord) -> Result<(), io::Error> {
        // 默认实现可以调用 handle
        self.handle(record)
    }
}